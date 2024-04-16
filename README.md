# decip

a helper command to `sort(1)` by IPv4/IPv6 address with decorating in hex format

## example usage

```
(ip-addr-from-some-log) | sort | uniq -c | decip -r | env LC_ALL=C sort -k 1,1 | cut -f 2-
```

```
(some-csv-with-ipaddr-in-first-column) | decip -d ',' | env LC_ALL=C sort -k 1,1 | cut -f 2-
```

## tips

`sort -V` will also work sorting IPv4 address if your `sort(1)` supports version sort.
