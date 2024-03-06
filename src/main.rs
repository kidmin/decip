use std::env;
use std::io::{BufRead, Write};
use std::net::IpAddr;
use std::str::FromStr;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DEFAULT_DELIMITER: &[char] = &[' ', '\t'];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let args: Vec<String> = env::args().collect();
    let _progname = args[0].clone();
    let mut opts = getopts::Options::new();
    let mut delimiter = ['\0'];
    opts.optopt("d", "", "delimiter character", "DELIMITER");
    opts.optflag("r", "", "parse the rightmost element instead of the leftmost one");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!("{}", e.to_string()),
    };
    let delimiter_char = match matches.opt_str("d") {
        Some(s) => {
            let mut it = s.chars();
            delimiter[0] = it.next().ok_or(format!("invalid delimiter character: {}", s))?;
            if it.next().is_some() {
                panic!("the delimiter must be a single character");
            }
            &delimiter
        },
        None => DEFAULT_DELIMITER,
    };
    let opt_rflag = matches.opt_present("r");

    let mut outfh = std::io::BufWriter::new(std::io::stdout().lock());

    for l in std::io::BufReader::new(std::io::stdin().lock()).lines() {
        let line = l?;
        let ipaddr_str =
            if opt_rflag {
                match line.rsplit_once(delimiter_char) {
                    Some((_, ipaddr_str)) => ipaddr_str,
                    None => &line,
                }
            } else {
                match line.split_once(delimiter_char) {
                    Some((ipaddr_str, _)) => ipaddr_str,
                    None => &line,
                }
            }
        ;

        match IpAddr::from_str(ipaddr_str) {
            Ok(IpAddr::V4(a)) =>
                writeln!(outfh, "4@{}\t{}", a.octets().map(|b| format!("{:02x}", b)).concat(), line)?,
            Ok(IpAddr::V6(a)) =>
                writeln!(outfh, "6@{}\t{}", a.octets().map(|b| format!("{:02x}", b)).concat(), line)?,
            Err(_) =>
                writeln!(outfh, "0@\t{}", line)?,
        };
    }

    outfh.flush()?;

    Ok(())
}

// vim: set fileencoding=utf-8 nobomb fileformat=unix filetype=rust number expandtab tabstop=8 softtabstop=4 shiftwidth=4 autoindent smartindent :
