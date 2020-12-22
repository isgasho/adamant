### Requirements
 - qemu-system-x86
 - echfs-utils
 - parted
 - cargo-xbuild
 - Rust of course

### Build
```sh
git submodule init
./create_disk.sh
```

### Run
```sh
qemu-system-x86_64 test.img -serial stdio -d cpu_reset,int -no-reboot
```