# hvisor
<p align = "center">
<br><br>
<img src="https://img.shields.io/badge/hvisor-orange" />
<img src="https://img.shields.io/github/license/saltytine/hypervisor?color=red" />
<img src="https://img.shields.io/github/contributors/saltytine/hypervisor?color=blue" />
<img src="https://img.shields.io/github/languages/code-size/saltytine/hypervisor?color=green">
<img src="https://img.shields.io/github/repo-size/saltytine/hypervisor?color=white">
<img src="https://img.shields.io/github/languages/top/saltytine/hypervisor?color=orange">
<br><br>
</p>

Armv8 hypervisor based on Linux & implemented in Rust，porting from [RVM1.5](https://github.com/rcore-os/RVM1.5) & [jailhouse](https://github.com/siemens/jailhouse).

Working In Progress.

## Progress

- [x] Architecture: aarch64
- [x] Platform: Qemu virt aarch64
- [x] Exception
- [x] Gicv3
- [x] Memory
- [x] Enable non root linux
- [ ] VirtIO device: block, net
- [ ] Architecture: riscv64
- [ ] Platform: nxp

## Build & Run

For detailed build and running tutorials, including building the development environment and creating a file system, please refer to [here](https://github.com/saltytine/notes-and-guides/blob/main/arm64-qemu-jailhouse.md).

To make it easy to get started, [here](https://archive.org/download/ubuntu-20.04-rootfs_ext4/hypervisor/) (extraction code: `sysH`) provides a  Linux kernel `Image` and a file system `ubuntu-20.04-rootfs_ext4.img` with the username `arm64` and the password as a whitespace. The directories are organized as follows:

```
├── home
	├── arm64
        ├── images: Contains a Linux Image and ramfs.
        ├── hvisor: Files required to run hvisor.
        ├── jailhouse: Files required to run jailhouse.
```

The following describes how to run a non-root-linux on jailhouse/hvisor based on `ubuntu-20.04-rootfs_ext4.img`:

1. Build `rvmarm.bin`:

   ```bash
   make all
   ```

   Then copy `target/aarch64/debug/rvmarm.bin` to `~/hypervisor/` in `ubuntu-20.04-rootfs_ext4.img`.

2. Start QEMU:

   ```bash
   sudo qemu-system-aarch64 \
       -machine virt,gic_version=3 \
       -machine virtualization=true \
       -cpu cortex-a57 \
       -machine type=virt \
       -nographic \
       -smp 16  \
       -m 1024 \
       -kernel your-linux-Image-path/Image \
       -append "console=ttyAMA0 root=/dev/vda rw mem=768m" \
       -drive if=none,file=your-rootfs-path/ubuntu-20.04-rootfs_ext4.img,id=hd0,format=raw \
       -device virtio-blk-device,drive=hd0 \
       -net nic \
       -net user,hostfwd=tcp::2333-:22
   ```

3. Enter the username `arm64` and the password as a whitespace after startup.

4. Go to the home directory and start non-root-linux:

   * For hvisor: go to the `hvisor` folder and run:

     ```
     ./setup.sh
     ./linux.sh
     ```

   * For Jailhouse: go to the `jailhouse` folder and run:

     ```
     ./linux.sh
     ```

### Enable a second serial console

If someone wants non-root-linux and root-linux in two different terminals, add this line at the end of the qemu startup command:

```
-device virtio-serial-device -chardev pty,id=serial3 -device virtconsole,chardev=serial3
```

After starting qemu, the `char device redirected to /dev/pts/num (label serial3)` message will output by the first terminal, execute this in another terminal:

```
sudo screen /dev/pts/num
```

where num is a specific number.

Related documents for this project are at
https://github.com/saltytine/aarch64-cpu
https://github.com/saltytine/sysHyper-testimg
