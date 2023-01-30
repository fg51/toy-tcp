# rust-template

## 1. send tcp segment
```sh
$ sudo ip netns exec host2 nc -l 10.0.1.1 40000
```

```sh
$ sudo ip netns exec host1 tcpdump -l
```
or 
$ sudo ip netns exec host1 wireshark


```sh
$ sudo ip netns exec host1 ./target/debug/examples/echo-client 10.0.1.1 40000
```