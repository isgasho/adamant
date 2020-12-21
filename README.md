# Build

```shell
git submodule init
./create_disk.sh
```

# Run
```shell
qemu-system-x86_64 test.img -serial stdio -d cpu_reset,int -no-reboot
```