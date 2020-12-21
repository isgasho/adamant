#!/bin/sh
rm test.img
cargo xbuild

dd if=/dev/zero bs=1M count=0 seek=64 of=test.img
parted -s test.img mklabel msdos
parted -s test.img mkpart primary 1 100%
parted -s test.img set 1 boot on # Workaround for buggy BIOSes

echfs-utils -m -p0 test.img quick-format 32768
echfs-utils -m -p0 test.img import limine.cfg limine.cfg
echfs-utils -m -p0 test.img import target/adamant-x86/debug/adamant kernel.elf
#echfs-utils -m -p0 test.img import <path to file> <path in image>
cd thirdparty/limine
make limine-install
./limine-install limine.bin ../../test.img