# `omegabox`
`omegabox` is intended to be a [multi-call](https://flameeyes.blog/2009/10/19/multicall-binaries/)
Rust binary that interfaces with the [Onion Omega2](https://onion.io/omega2/)
hardware. Although the Omega2+ can benefit as well, this binary is optimized
for size to try to use the least amount of the 16MB flash of the Omega2(+) as
possible.

A multi-call binary is used because Rust strongly prefers static linking. The
idea is that with [fat LTO](https://doc.rust-lang.org/rustc/codegen-options/index.html#lto)
enabled, the cost of adding a new command becomes amortized over time,
as each new binary can reuse already compiled-in crates (especially from
`libstd`). The application should be small enough that extra compile time from
using fat LTO is negligible.

LTO and text-segment sharing will (hopefully) result in on-disk size reduction
over time similar to dynamic linking, with a negligible increase in RAM usage
across running `omegabox` instances.

Other coding practices optimized for size include:

* Trait objects are prefered over static dispatch.
* Debug symbols are stripped.
* Use [UPX](https://github.com/upx/upx) compression to compress the binary,
  while still allowing text-segment sharing.
* Use [`pico-args`](https://github.com/RazrFalcon/pico-args) for subcommands,
  which has a unified layout/representation for parsing args (at the cost of
  more RAM?); [`argh`](https://github.com/google/argh), for instance, prefers
  static dispatch via a trait.

## Prerequisites
1. At present, only POSIX OSes in practice are supported for compiling this
   crate, since the initial setup attempts to compile the linker and driver
   for you using Onion's [OpenWRT fork](https://github.com/OnionIoT/source).

2. You will need to add the appropriate MIPS target to your Rust installation:

   ```
   rustup target add mipsel-unknown-linux-musl
   ```

3. I assume you will be using [`just`](https://github.com/casey/just) to
   _at least_ setup the linker:

   ```
   cargo install just
   ```

4. `rsync` and `ssh` are required to transfer `omegabox` from your host to the
   Omega2. Check your distro package manager for more information.

5. `rsync` and `ssh` above assume you have public-key authentication set up
   on your Omega2. See the "Using SSH Key Pairs" section of the [linked](https://docs.onion.io/omega2-docs/connecting-to-the-omega-terminal.html)
   Omega2 Docs page for instructions on how to set this up.

6. Make sure `$HOME/bin` on your Omega2 is on the path. For instance:

   ```
   root@Omega-ABCD:~# cat .profile
   export PATH=$HOME/bin:$PATH
   root@Omega-ABCD:~#
   ```

7. [`upx`](https://github.com/upx/upx) is an optional requirement to compress
   `omegabox` before sending it to the Omega2. Check your distro package
   manager for more information.

## Quick Start
1. `just` uses an `.env` file to set environment variables. I provide an
   `.env.sample`, which should be copied to `.env` and then modified.
    Variables to be set include:

    * `OMEGA2_SRC`- Path to where you want the Onion OpenWRT source
       to live. The linker and compiler driver is compiled there.
    * `OMEGA2_HOST`- `user@hostname` or `user@ip_address` associated with your
       Omega2.

    The remaining variables get their values from above.

2. For the `mipsel-unknown-linux-musl` target, `rustc` will [default to](https://shane.logsdon.io/writing/cross-compiling-rust-applications-for-the-onion-omega2-from-macos/)
   the host linker, which is probably not what you want. This crate uses
   `.cargo/config` to override the linker, but it's likely we need to _build_
   a suitable one first.

   With your `.env` set up, we can compile and install the linker and a
   MIPS (cross)-compiler driver suitable for Omega2 by running the following:

   ```
   just make-toolchain
   ```

3. With the Rust and C toolchains installed and an `.env` file set up, running
   `just` or `just all` should:

   1. Compile `omegabox` (`just build`).
   2. Strip debug symbols (`just strip`).
   3. Compress `omegabox` with `upx` if the binary exists (`just compress`).
   4. Transfer the compressed `omegabox` to your Omega2 (`just xfer`).
      `omegabox` will be in `$HOME/bin`.

4. If you want more control over your build, look at the above commands in the
   `Justfile`, especially the environment variables used. Run `just -l` for
   a brief description on each command.

5. _I am still deciding on how to automate adding symlinks for multi-call binary
   behavior._ For now, they need to be added manually. A list of valid commands
   is generated in `build.rs` as a [perfect hash function](https://github.com/sfackler/rust-phf).

## Adding A New Command

TODO
