#!/bin/sh
qemu-system-x86_64 test.img -no-reboot --enable-kvm -serial stdio
