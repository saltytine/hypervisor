The image comes fromt he Linux kernel compiled by yjy and cxy, and the kernel version is 5.4.0.
The matching jailhouse.ko and cell files have been patched with RVM1.5.
The rootfs comes from demo-image-jailhouse-demo-qemu-arm64.ext4.img compiled by jailhouse-image, about 1.4g, among which there is no jailhouse with patch, so it needs to be replaced.

The qemu startup command is
```sh
qemu-system-aarch64 -drive file=demo-image-jailhouse-demo-qemu-arm64.ext4.img,discard=unmap,if=none,id=disk,format=raw \
-m 1G -serial mon:stdio -netdev user,id=net,hostfwd=tcp::23333-:22 \
-kernel Image \
-append "root=/dev/vda mem=768M" -initrd demo-image-jailhouse-demo-qemu-arm64-initrd.img \
-cpu cortex-a57 -smp 16 -nographic -machine virt,gic-version=3,virtualization=on \
-device virtio-serial-device -device virtconsole,chardev=con -chardev vc,id=con -device virtio-blk-device,drive=disk \
-device virtio-net-device,netdev=net
```
After startup, transfer the matching jailhouse.ko, config files, and rvmarm.bin to be tested via scp
