# prefix-aggregator
A command-line tool for aggregating IPv4 prefixes 

# How to use
## Aggregate to the largest prefix possible 
```
$ prefix-aggregator << EOS
10.0.0.0/24
10.0.1.0/24
10.0.2.0/24
EOS
```
Expected to
```
10.0.0.0/23
10.0.2.0/24
```
  
## Large prefix includes small prefix 
```
$ prefix-aggregator << EOS
10.0.0.0/8
10.0.0.0/16
10.0.1.0/16
EOS
```
Expected to
```
10.0.0.0/8
```

## Focus only on the information you need 
```
$ curl -s https://ip-ranges.amazonaws.com/ip-ranges.json | jq -r '.prefixes[].ip_prefix' | sort -V | uniq | wc -l
4314
$ curl -s https://ip-ranges.amazonaws.com/ip-ranges.json | jq -r '.prefixes[].ip_prefix' | sort -V | uniq | prefix-aggregator | wc -l
1101
```
