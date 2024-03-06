use assert_cmd::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn parse_ipv4addr() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("decip")?;

    cmd.write_stdin("192.0.2.254\n")
        .assert()
        .success()
        .stdout("4@c00002fe\t192.0.2.254\n");

    Ok(())
}

#[test]
fn parse_ipv6addr() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("decip")?;

    cmd.write_stdin("2001:db8:feed::1:beef\n")
        .assert()
        .success()
        .stdout("6@20010db8feed0000000000000001beef\t2001:db8:feed::1:beef\n");

    Ok(())
}

#[test]
fn invalid_ipaddr() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("decip")?;

    cmd.write_stdin("##str1##\t##str2##\n")
        .assert()
        .success()
        .stdout("0@\t##str1##\t##str2##\n");

    Ok(())
}

#[test]
fn default_delimiter_left() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("decip")?;

    cmd.write_stdin("2001:db8:feed::1:beef str1\n\
                     2001:db8:feed::1:cafe\tstr2\n")
        .assert()
        .success()
        .stdout("6@20010db8feed0000000000000001beef\t2001:db8:feed::1:beef str1\n\
                 6@20010db8feed0000000000000001cafe\t2001:db8:feed::1:cafe\tstr2\n");

    Ok(())
}

#[test]
fn delimiter_from_arg_left() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("decip")?;

    cmd.arg("-d").arg(",");
    cmd.write_stdin("2001:db8:feed::1:beef,str1\n\
                     2001:db8:feed::1:cafe,str2\n")
        .assert()
        .success()
        .stdout("6@20010db8feed0000000000000001beef\t2001:db8:feed::1:beef,str1\n\
                 6@20010db8feed0000000000000001cafe\t2001:db8:feed::1:cafe,str2\n");

    Ok(())
}

#[test]
fn default_delimiter_right() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("decip")?;

    cmd.arg("-r");
    cmd.write_stdin("str1 2001:db8:feed::1:beef\n\
                     str2\t2001:db8:feed::1:cafe\n")
        .assert()
        .success()
        .stdout("6@20010db8feed0000000000000001beef\tstr1 2001:db8:feed::1:beef\n\
                 6@20010db8feed0000000000000001cafe\tstr2\t2001:db8:feed::1:cafe\n");

    Ok(())
}

#[test]
fn delimiter_from_arg_right() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("decip")?;

    cmd.arg("-r").arg("-d").arg(",");
    cmd.write_stdin("str1,2001:db8:feed::1:beef\n\
                     str2,2001:db8:feed::1:cafe\n")
        .assert()
        .success()
        .stdout("6@20010db8feed0000000000000001beef\tstr1,2001:db8:feed::1:beef\n\
                 6@20010db8feed0000000000000001cafe\tstr2,2001:db8:feed::1:cafe\n");

    Ok(())
}

#[test]
fn odd_ipv4addr() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("decip")?;

    cmd.write_stdin("192.0.02.0xff\n")
        .assert()
        .success()
        .stdout("0@\t192.0.02.0xff\n");

    Ok(())
}

#[test]
fn delimiter_needs_an_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("decip")?;

    cmd.arg("-d");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(" missing"));

    Ok(())
}

#[test]
fn delimiter_must_be_a_single_character() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("decip")?;

    cmd.arg("-d").arg("four");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(" must be a single character"));

    Ok(())
}

#[test]
#[ignore]
fn write_to_devfull_fails() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = std::process::Command::cargo_bin("decip")?;

    let devfull_path = std::path::Path::new("/dev/full");
    let devfull_fh = match std::fs::File::create(&devfull_path) {
        Ok(fh) => fh,
        Err(e) => panic!("couldn't open {}: {}", devfull_path.display(), e),
    };
    // TODO
    cmd.stdout(devfull_fh);
    let mut cmd = Command::from_std(cmd);
    cmd.write_stdin("foo\n")
        .assert()
        .failure()
        .stderr(predicate::str::contains(" kind: StorageFull,"));

    Ok(())
}
