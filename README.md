#sysHyper
![Static Badge](https://img.shields.io/badge/sysHyper-orange)
![GitHub](https://img.shields.io/github/license/syswonder/sysHyper?color=red)

[![Contributors](https://img.shields.io/github/contributors/syswonder/sysHyper?color=blue)](https://github.com/syswonder/sysHyper)
![GitHub Repo stars](https://img.shields.io/github/stars/syswonder/sysHyper?color=yellow)
![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/w/syswonder/sysHyper?color=black)

![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/syswonder/sysHyper?color=green)
![GitHub repo size](https://img.shields.io/github/repo-size/syswonder/sysHyper?color=white)
![GitHub top language](https://img.shields.io/github/languages/top/syswonder/sysHyper?color=orange)




Armv8 hypervisor based on Linux & implemented in Rust, porting from [RVM1.5](https://github.com/rcore-os/RVM1.5) & [jailhouse](https://github.com/siemens/jailhouse)

## Progress
- [x] arch_entry
- [x] cpu
- [x]logging
- [x]exception
- [x]gicv3
- [x]memory
- [ ] ....
## Platform
- [x] qemu
- [ ] imx
- [ ] ti
- [ ] rpi4
## Environment Configuration
### Install Rust
First, install Rust version manager rustup and Rust package manager cargo.
### qemu simulator compilation
```sh
sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
gawk build-essential bison flex texinfo gperf libtool patchutils bc \
zlib1g-dev libexpat-dev pkg-config libglib2.0-dev libpixman-1-dev git tmux python3 ninja-build # Install the required dependencies for compilation
wget https://download.qemu.org/qemu-7.0.0.tar.xz # Download source code
tar xvJf qemu-7.0.0.tar.xz # Unzip
cd qemu-7.0.0
./configure # Generate configuration file
make -j$(nproc) # Compile
qemu-system-aarch64 --version # View version
```
Qemu version > 7.2 requires additional configuration, otherwise the following problems may occur at startup
```
network backend user is not compiled into this binary
```
The following settings need to be made before compiling:
```sh
sudo apt install libslirp-dev
../configure --enable-slirp
```
After compiling, you can `sudo make install` to install Qemu to the `/usr/local/bin` directory,
You can also edit ` ~/.bashrc` File (if you are using the default bash terminal), add the following to the end of the file:
```
export PATH=$PATH:/path/to/qemu-7.0.0/build
```

### Start qemu
```sh
mkdir qemu-test # Create a new folder for testing
git submodule update --init --recursive # Update submodules
cp -r test-img/* qemu-test # Transfer the required files to the test folder
cd qemu-test/host
./test.sh # Start qemu
```
The default user password for Linux is root/root
### Compile sysHyper
Execute on the host
```sh
make # Compile the hypervisor image rvmarm.bin
make scp # Transfer the obtained rvmarm.bin file to the Linux running on qemu
```
### Run sysHyper
Transfer the necessary files to the guest Linux:
```sh
scp -P 2333 -r qemu-test/guest/* root@localhost:~/
```
In guest linux
```sh
./setup.sh #Set file path
./enable.sh #Run sysHyper, enable virtualization
cat /proc/cpuinfo #View current linux cpuinfo
jailhouse cell create configs/qemu-arm64-gic-demo.cell #Create a new cell, move cpu 3 out of the root cell
cat /proc/cpuinfo #View current linux cpuinfo, cpu3 is shutdown
jailhouse disable #Disable virtualization
```
### output
You should be able to see some information printed by the hypervisor

### Debugging
You can use vscode for visual debugging, add `-s -S` to the end of the original qemu command
```sh
qemu-system-aarch64 \
    -drive file=./rootfs.qcow2,discard=unmap,if=none,id=disk,format=qcow2 \
    -device virtio-blk-device,drive=disk \
    -m 1G -serial mon:stdio \
    -kernel Image \
    -append "root=/dev/vda mem=768M" \
    -cpu cortex-a57 -smp 4 -nographic -machine virt,gic-version=3,virtualization=on \
    -device virtio-serial-device -device virtconsole,chardev=con \
    -chardev vc,id=con \
    -net nic \
    -net user,hostfwd=tcp::2333-:22 -s -S
```
Start qemu first, then press F5 to start debugging

### Original jailhouse
During the development and debugging process, in order to facilitate comparison with the original jailhouse, the original jailhouse running environment of version v0.12 is also provided:
    - test-img/host/jail-img kernel
    - test-img/guest/jail Original jailhouse compiled generated files
The running command is:
```sh
qemu-system-aarch64 \
    -drive file=./rootfs.qcow2,discard=unmap,if=none,id=disk,format=qcow2 \
    -m 1G -serial mon:stdio -netdev user,id=net,hostfwd=tcp::23333-:22 \
    -kernel jail-img \
    -append "root=/dev/vda mem=768M"  \
    -cpu cortex-a57 -smp 16 -nographic -machine virt,gic-version=3,virtualization=on \
    -device virtio-serial-device -device virtconsole,chardev=con -chardev vc,id=con -device virtio-blk-device,drive=disk \
    -device virtio-net-device,netdev=net
```
In guest:
```sh
cd jail
insmod ./jailhouse.ko
cp jailhouse.bin /lib/firmware/
./jailhouse enable configs/qemu-arm64.cell
```

Related documents for this project are at
https://github.com/saltytine/aarch64-cpu
https://github.com/saltytine/sysHyper-testimg
