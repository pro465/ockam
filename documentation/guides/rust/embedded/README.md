```yaml
title: Get Started
```

# Ockam Embedded Microcontroller Guide

> Discover the world of microcontrollers with [Ockam] and [Embedded Rust]!

[Ockam]: https://www.ockam.io/
[Embedded Rust]: https://www.rust-lang.org/what/embedded


## Build End-to-End Encrypted and Secure Messaging Channels

In this step-by-step guide we’ll learn how to build mutually-authenticated, end-to-end encrypted,
secure messaging channels that protect en-route messages against eavesdropping, tampering, and forgery.

Data, within modern distributed applications, are rarely exchanged over a single point-to-point
transport connection. Application messages routinely flow over complex, multi-hop, multi-protocol
routes — _across data centers, through queues and caches, via gateways and brokers_ — before reaching
their end destination.

Transport layer security protocols are unable to protect application messages because their protection
is constrained by the length and duration of the underlying transport connection. Ockam is a collection of
programming libraries (in Rust and Elixir) that make it simple for our applications to guarantee end-to-end
integrity, authenticity, and confidentiality of data.

We no longer have to implicitly depend on the defenses of every machine or application within the same,
usually porous, network boundary. Our application's messages don't have to be vulnerable at every point,
along their journey, where a transport connection terminates.

Instead, our application can have a strikingly smaller vulnerability surface and easily make
_granular authorization decisions about all incoming information and commands._

Let's build mutually-authenticated, end-to-end protected communication between distributed applications!


## Setup

In order to follow along with these guides you will need to do some prepatory setup:

1. Obtain the embedded development hardware you'll be working with.
2. Set up a local software development environment that can compile and debug programs for your development board.


### Hardware requirements

For these guides you'll be using the [Microchip SAM E54 Xplained Pro evaluation kit].

Please read the <a href="./hardware#readme">Hardware requirements</a> section to find out more.

[Microchip SAM E54 Xplained Pro evaluation kit]: https://www.microchip.com/en-us/development-tool/ATSAME54-XPRO


### Software development environment

To develop for Ockam and Embedded Rust you'll need to set up a development environment with the following tools:

1. A Rust [toolchain] for your development workstation.
2. A Rust [cross-compilation target] for the embedded system you're targeting. (`thumbv7em-none-eabihf` in this case)
3. A debugger that can debug programs running on your development board. (a copy of `gdb` compiled with ARM Cortex-M support in this case)
4. An on-chip debugger frontend to flash your firmware and manage the connection between your workstation and `gdb`. (`openocd`)

[toolchain]: https://rust-lang.github.io/rustup/concepts/toolchains.html
[cross-compilation target]: https://rust-lang.github.io/rustup/cross-compilation.html

To find out more, please read the instructions corresponding to your development workstation:

<ul>
<li><a href="./software/linux.md">Linux</a></li>
<li><a href="./software/macos.md">macOS</a></li>
<li><a href="./software/windows.md">Windows</a></li>
</ul>


## Step-by-step

<ul>
<li><a href="./hardware#readme">Hardware requirements</a></li>
<li><a href="./software#readme">Setting up a development environment</a></li>
<li><a href="./01-node#readme">01. Node</a></li>
<li><a href="./02-worker#readme">02. Worker</a>
<li><a href="./03-routing#readme">03. Routing</a></li>
<li><a href="./04-transport#readme">04. Transport</a></li>
<li><a href="./05-secure-channel#readme">05. Secure Channel</a></li>
</ul>

<div style="display: none; visibility: hidden;">
<hr><b>Next:</b> <a href="./hardware#readme">Hardware requirements</a>
</div>
