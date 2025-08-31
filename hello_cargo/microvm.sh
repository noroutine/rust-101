#!/bin/bash
# run-in-vm.sh
limactl start --name temp-vm template://alpine
limactl copy temp-vm "$1" /tmp/binary
limactl shell temp-vm /tmp/binary
limactl delete temp-vm