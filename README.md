# decip

a helper command to `sort(1)` by IPv4/IPv6 address

## example usage

```
(ip-addr-from-some-log) | sort | uniq -c | decip -r | sort -k 1,1 | cut -f 2-
```

```
(some-csv-with-ipaddr-in-first-column) | decip -d ',' | sort -k 1,1 | cut -f 2-
```

## tips

`sort -V` will also work sorting IPv4 address if your `sort(1)` supports version sort.
