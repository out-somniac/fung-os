## Installing dependencies
Install the required target with rustup using:
```
rustup target add aarch64-unknown-none-softfloat
```
Install `cargo-binutils` and `llvm-tools`

```
cargo install cargo-binutils
rustup component add llvm-tools-preview
```
Install docker using your prefered package manager.

## Running
### QEMU
Start the `docker` daemon. With systemd this would be done using:
```
sudo systemctl start docker.service
```
  
Run the `qemu` target from the `Makefile`.
```
make qemu
```

## Kernel Image
Currenty the kernel image can be found in `kernel8.img`. So far these are the two instructions - wfe and b
