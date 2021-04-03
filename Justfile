# Run build, strip, compress, and xfer recipes in order.
all: build strip compress xfer

# Build omegabox in release mode.
build:
    PATH=$PATH:$LINKER_DIR cargo build --release --target mipsel-unknown-linux-musl

# cargo strip --target mipsel-unknown-linux-musl does not work for now.
# Strip debug symbols using mipsel-openwrt-linux-musl-strip.
strip:
    PATH=$PATH:$LINKER_DIR mipsel-openwrt-linux-musl-strip target/mipsel-unknown-linux-musl/release/omegabox

# Compress binary using UPX (if it exists).
compress:
    #!/bin/sh

    if which upx > /dev/null; then
        rm target/mipsel-unknown-linux-musl/release/omegabox-comp
        upx -9 -o target/mipsel-unknown-linux-musl/release/omegabox-comp target/mipsel-unknown-linux-musl/release/omegabox;
    else
        cp target/mipsel-unknown-linux-musl/release/omegabox target/mipsel-unknown-linux-musl/release/omegabox-comp
    fi

# Transfer omegabox from the host computer to the Omega2.
xfer:
    ssh $OMEGA2_HOST "mkdir -p bin"
    rsync -aH target/mipsel-unknown-linux-musl/release/omegabox-comp $OMEGA2_HOST:bin/omegabox

# Taken mostly from: https://shane.logsdon.io/writing/cross-compiling-rust-applications-for-the-onion-omega2-from-macos/
# Compile the C linker and compiler driver to link Omega2 MIPS programs from Rust.
make-toolchain:
    #!/bin/sh

    set -e

    if ! [ -d $OMEGA2_SRC ]; then
        git clone https://github.com/OnionIoT/source.git -o onion $OMEGA2_SRC
        cd $OMEGA2_SRC
        sh scripts/onion-feed-setup.sh
        python scripts/onion-setup-build.py
    else
        cd $OMEGA2_SRC
        git pull onion
    fi

    make toolchain/install
