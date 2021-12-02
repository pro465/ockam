# Setting up a development environment

To develop for Ockam and Embedded Rust you'll need to set up a development environment with the following tools:

1. A Rust [toolchain] for your development machine.
2. A Rust [cross-compilation target] for the embedded system you're targeting. (`thumbv7em-none-eabihf` in this case)
3. A debugger that can debug programs running on your development board. (a copy of `gdb` compiled with ARM Cortex-M support in this case)
4. An on-chip debugger frontend to flash your firmware and manage the connection between the device and `gdb`. (`openocd`)

[toolchain]: https://rust-lang.github.io/rustup/concepts/toolchains.html
[cross-compilation target]: https://rust-lang.github.io/rustup/cross-compilation.html


See the instructions corresponding to your development machine for more information:

<ul>
<li><a href="./linux.md">Linux</a></li>
<li><a href="./macos.md">macOS</a></li>
<li><a href="./windows.md">Windows</a></li>
</ul>


----

If the above instructions don't work on your machine, please
[post a question](https://github.com/ockam-network/ockam/discussions/1642),
we would love to help.
