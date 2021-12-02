# Linux

## 1. Rust toolchain

If you don't already have it, please [install] the latest version of Rust.

### rustc and Cargo

For Linux, the easiest way to install Rust is to run the [rustup installer script]:

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

### Ubuntu 20.04 LTS or newer

On Ubuntu 20.04 or newer you can use the `gdb-multiarch` binary which supports multiple architectures, including ARM Cortex-M.

To install it, use:

```zsh
$ sudo apt install gdb-multiarch

$ gdb-multiarch --version
GNU gdb (Ubuntu 9.2-0ubuntu1~20.04) 9.2
```

### Other Linux Distributions

Most Linux distributions come with packages for either `gdb-multiarch` or the `arm-none-eabi-gdb` binary from ARM's pre-built GNU Arm Embedded Toolchain.

If need be you can also obtain the `arm-none-eabi-gdb` binary directly from [ARM's download site] and install it using something like:

```zsh
$ mkdir -p ~/local && cd ~/local
$ tar xjf /path/to/downloaded/gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2
```

Then, use your editor of choice to append to your PATH in the appropriate shell init file (e.g. ~/.zshrc or ~/.bashrc):

```
PATH=$PATH:$HOME/local/gcc-arm-none-eabi-10.3-2021.10/bin
```

Finally, check that you call the `arm-none-eabi-gdb` binary:

```zsh
$ arm-none-eabi-gdb --version
GNU gdb (GNU Arm Embedded Toolchain 10.3-2021.10) 10.2.90.20210621-git
```


## 4. OpenOCD

### Ubuntu 20.04 LTS or newer

TODO the version of openocd installed does not yet support atsame54

You can install the `openocd` binary with:

```zsh
$ sudo apt install openocd

$ openocd --version
Open On-Chip Debugger 0.10.0
```



### Other Linux Distributions

Even if your distribution does not provide an `openocd` package you can still obtain a precompiled binary from The xPack Project: [@xpack-dev-tools/openocd]

[@xpack-dev-tools/openocd]:  https://xpack.github.io/openocd/install/


## 5. XPM

The xPack Project provides a set of cross-platform tools to manage, configure and build complex, modular, multi-target (multi-architecture, multi-board, multi-toolchain) projects, with an emphasis on C/C++ and bare-metal embedded projects.

To install it you will need a recent nodejs install:

```zsh
$ sudo apt install nodejs npm
```

Install the `xpm` tool:

```zsh
$ npm install --global xpm@latest
```

_OR_

```zsh
$ npm init
$ npm install xpm --save

$ npx xpm --version
0.10.8

$ npx xpm init
```

Now you can install the rest of the tools we'll need to follow the guide.


## GDB

```zsh
$ npx xpm install @xpack-dev-tools/arm-none-eabi-gcc@latest --save

$ ./xpacks/.bin/arm-none-eabi-gdb --version
GNU gdb (xPack GNU Arm Embedded GCC aarch64) 10.2.90.20210621-git
```


## OpenOCD

```zsh
$ npx xpm install @xpack-dev-tools/openocd@latest --save

$ ./xpacks/.bin/openocd --version
xPack OpenOCD aarch64 Open On-Chip Debugger 0.11.0+dev (2021-10-16-21:18)
```



## 6. udev rules

These rules will let you use the USB debugging interface on your development board without needing root privileges.

Create a file `/etc/udev/rules.d/99-atsame54.rules` with the following content:

```
# CMSIS-DAP for ATSAME54
ATTRS{idVendor}=="03eb", ATTRS{idProduct}=="2111", MODE="0666"
```

Then reload the udev rules with:

```zsh
$ sudo udevadm control --reload-rules && sudo udevadm trigger
```

If your board was plugged in you will need to plug it out and in again.


----

If the above instructions don't work on your machine, please [post a question](https://github.com/ockam-network/ockam/discussions/1642), we would love to help.
