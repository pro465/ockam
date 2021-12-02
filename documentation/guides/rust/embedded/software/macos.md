# macOS

## 1. Rust toolchain

If you don't already have it, please [install] the latest version of Rust.

### rustc and Cargo

For macOS, the easiest way to install Rust is to run the [rustup installer script]:

```zsh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you already have rustup installed double check that you are on the stable channel and your stable toolchain is up-to-date. rustc -V should return a date newer than the one shown below:

```zsh
$ rustc -V
rustc 1.57.0 (f1edd0429 2021-11-29)
```

[install]: https://www.rust-lang.org/tools/install
[rustup install script]: https://rustup.rs

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

You can install the ARM GCC toolchain on macOS using [Homebrew]:

```zsh
$ brew tap ArmMbed/homebrew-formulae
$ brew install arm-none-eabi-gcc
```

Check that the `arm-none-eabi-gdb` binary is installed:

```zsh
$ arm-none-eabi-gdb --version
GNU gdb (GNU Arm Embedded Toolchain 10-2020-q4-major) 10.1.90.20201028-git
```


## 4. OpenOCD

You can install the `openocd` binary on macOS using [Homebrew]:

```zsh
$ brew install open-ocd

$ openocd --version
Open On-Chip Debugger 0.11.0
```


----

If the above instructions don't work on your machine, please [post a question](https://github.com/ockam-network/ockam/discussions/1642), we would love to help.
