# armv8-baremetal-demo-rust

using rust to write an armv8 hypervisor, for the purpose of being better than humza, and ofc, learning

## ~/.cargo/config
```shell
[build]
target="aarch64-unknown-none"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
rustflags = [
    "-C", "link-arg=-nostartfiles -Tlinker.ld",
]

[target.aarch64-unknown-none]
linker = "aarch64-none-elf-gcc"
```
youll need to install linker: `aarch64-none-elf-` address: https://developer.arm.com/-/media/Files/downloads/gnu-a/10.3-2021.07/binrel/gcc-arm-10.3-2021.07-x86_64 -aarch64-none-elf.tar.xz?rev=9d9808a2d2194b1283d6a74b40d46ada&hash=4E429A41C958483C9DB8ED84B051D010F86BA62
install the rust toolchain: `rustup install nightly && rustup default nightly && rustup target add aarch64-unknown-none (optional, we use the json config)`

`apt install gdb-multiarch`

## compile
```shell
make
```

## qemu
```shell
make start
```
or
```shell
qemu-system-aarch64 \
    -M virt \
    -m 1024M \
    -cpu cortex-a53 \
    -nographic \
    -kernel target/aarch64-unknown-linux-gnu/debug/armv8-baremetal-demo-rust
```

## qemu debug
```shell
qemu-system-aarch64 \
    -M virt \
    -m 1024M \
    -cpu cortex-a53 \
    -nographic
    -machine virtualization=on \
    #-machine secure=on \
    -kernel target/aarch64-unknown-linux-gnu/debug/armv8-baremetal-demo-rust \
    -S -s
```
and then use
`gdb-multiarch target/aarch64-unknown-linux-gnu/debug/armv8-baremetal-demo-rust`
enter gdb and put: `target remote:1234` to start debugging
(-machine virtualization=on enables el2, secure=on turns on el3, but we only really need to start with el2)
then use aarch64-linux-gnu-gdb -x debug.gdb, qemu starts virt from el1 by default

references:
https://stackoverflow.com/questions/42824706/qemu-system-aarch64-entering-el1-when-emulating-a53-power-up
https://stackoverflow.com/questions/31787617/what-is-the-current-execution-mode-exception-level-etc
https://github.com/cirosantilli/linux-kernel-module-cheat/tree/35684b1b7e0a04a68987056cb15abd97e3d2f0cc#arm-exception-level

## compile gdb for aarch64 (unsuccessful)
1. download gdb from source: https://ftp.gnu.org/gnu/gdb/gdb-13.1.tar.gz
2. tar -xzvf gdb-13.1.tar.gz
3. mkdir build
4. cd $_
5. ../configure --prefix=$PWD --target=aarch64-linux-gnu
6. make -j$(nproc) [CFLAGS=-static CXXFLAGS=-static]

--target specifies the architecture of the program to be debugged, --host specifies the architecture of the gdb program to run

## compile qemu (available)
1. download qemu10.0.source
2. tar decompress
3. mkdir build && cd build
4. ../qemu-10.0.1/configure --enable-kvm --enable-slirp --enable-debug --target-list=aarch64-softmmu,x86_64-softmmu
5. make-j2
