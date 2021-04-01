all: build strip xfer

build:
    PATH=$PATH:$LINKER_DIR cargo build --release --target mipsel-unknown-linux-musl

strip:
    # cargo strip --target mipsel-unknown-linux-musl
    PATH=$PATH:$LINKER_DIR mipsel-openwrt-linux-musl-strip target/mipsel-unknown-linux-musl/release/omegabox
    size target/mipsel-unknown-linux-musl/release/omegabox

xfer:
    ssh $OMEGA2_HOST "mkdir -p bin"
    rsync -aH target/mipsel-unknown-linux-musl/release/omegabox $OMEGA2_HOST:bin
