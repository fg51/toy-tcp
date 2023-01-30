# !/bin/bash
sudo ip netns exec host2 ip route get 10.0.0.1
# 10.0.0.1 via 10.0.1.254 dev host2-veth1 src 10.0.1.1 uid 0
# cache
