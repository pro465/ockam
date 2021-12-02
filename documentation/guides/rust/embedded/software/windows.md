# Windows

## 1. Rust toolchain

If you don't already have it, you'll need to [install] the latest version of Rust.

### rustc and Cargo

For Windows, the easiest way to install Rust is to go to the [rustup homepage] and follow the onscreen instructions.

If you already have rustup installed double check that you are on the stable channel and your stable toolchain is up-to-date. rustc -V should return a date newer than the one shown below:

```zsh
$ rustc -V
rustc 1.57.0 (f1edd0429 2021-11-29)
```

[install]: https://www.rust-lang.org/tools/install
[rustup homepage]: https://rustup.rs

### nightly channel

Install the Rust nightly channel, and check that the date is newer than the own shown:

```zsh
rustup toolchain install nightly

$ rustc +nightly -V
rustc 1.59.0-nightly (efec54529 2021-12-04)
```


## 2. Rust cross-compilation target for ARM Cortex-M

The SAM E54 development board we are using is based on a [ATSAME54P20A] micro-controller that features a 32-bit ARM Cortex-M4 processor.


The [Rust target] corresponding to this processor is `thumbv7em-none-eabihf` and can be installed with:

```zsh
$ rustup target add thumbv7em-none-eabihf --toolchain nightly
```

[ATSAME54P20A]: https://www.microchip.com/en-us/product/ATSAME54P20A
[Rust target]: https://doc.rust-lang.org/nightly/rustc/platform-support.html


## 3. GDB

You can obtain the `arm-none-eabi-gdb` binary from The xPack Project: [@xpack-dev-tools/arm-none-eabi-gcc]

[@xpack-dev-tools/arm-none-eabi-gcc]: https://xpack.github.io/arm-none-eabi-gcc/install/


```zsh
$ openocd --version
Open On-Chip Debugger 0.11.0
```



## 4. OpenOCD

You can obtain the `openocd` binary from The xPack Project: [@xpack-dev-tools/openocd]

[@xpack-dev-tools/openocd]:  https://xpack.github.io/openocd/install/


```zsh
$ openocd --version
Open On-Chip Debugger 0.11.0
```


----

If the above instructions don't work on your machine, please [post a question](https://github.com/ockam-network/ockam/discussions/1642), we would love to help.
