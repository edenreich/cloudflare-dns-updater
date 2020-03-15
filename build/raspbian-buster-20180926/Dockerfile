FROM resin/raspberry-pi-debian:buster-20180926

ENV ZLIB_VERSION=1.2.11 \
    OPENSSL_VERSION=1.0.2r \
    RUST_VERSION=1.42.0 \
    PATH=/home/rust/.cargo/bin:/home/rust/app/bin:$PATH

ENV OPENSSL_DIR=/usr/local/musl/ \
    OPENSSL_INCLUDE_DIR=/usr/local/musl/include/ \
    DEP_OPENSSL_INCLUDE=/usr/local/musl/include/ \
    OPENSSL_LIB_DIR=/usr/local/musl/lib/ \
    OPENSSL_STATIC=1

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    autoconf \
    automake \
    binutils \
    ca-certificates \
    curl \
    file \
    git \
    libtool \
    m4 \
    cmake \
    xutils-dev \
    musl-tools \
    pkg-config

RUN echo "Building OpenSSL" && \
    ls /usr/include/linux && \
    mkdir -p /usr/local/musl/include && \
    ln -s /usr/include/linux /usr/local/musl/include/linux && \
    ln -s /usr/include/arm-linux-gnueabihf/asm /usr/local/musl/include/asm && \
    ln -s /usr/include/asm-generic /usr/local/musl/include/asm-generic && \
    cd /tmp && \
    curl -LO "https://www.openssl.org/source/openssl-$OPENSSL_VERSION.tar.gz" && \
    tar xvzf "openssl-$OPENSSL_VERSION.tar.gz" && cd "openssl-$OPENSSL_VERSION" && \
    CC=musl-gcc ./Configure no-shared no-zlib -fPIC --prefix=$OPENSSL_DIR -DOPENSSL_NO_SECURE_MEMORY linux-generic32 && \
    C_INCLUDE_PATH=$DEP_OPENSSL_INCLUDE make depend && \
    C_INCLUDE_PATH=$OPENSSL_INCLUDE_DIR make -j$(nproc) && \
    make install && \
    rm /usr/local/musl/include/linux /usr/local/musl/include/asm /usr/local/musl/include/asm-generic && \
    rm -r /tmp/*

RUN echo "Building zlib" && \
    cd /tmp && \
    curl -LO "http://zlib.net/zlib-$ZLIB_VERSION.tar.gz" && \
    tar xzf "zlib-$ZLIB_VERSION.tar.gz" && cd "zlib-$ZLIB_VERSION" && \
    CC=musl-gcc ./configure --static --prefix=/usr/local/musl && \
    make -j$(nproc) && make install && \
    rm -r /tmp/*

RUN useradd rust --user-group --create-home --shell /bin/bash --groups sudo && \
    echo '%sudo    ALL=(ALL:ALL) NOPASSWD:ALL' > /etc/sudoers.d/rust
USER rust
WORKDIR /home/rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    rustup target add armv7-unknown-linux-musleabihf

COPY --chown=rust:rust build/cargo_default_config.toml /home/rust/.cargo/config
COPY --chown=rust:rust Cargo.toml Cargo.lock app/
COPY --chown=rust:rust src app/src 

RUN cd app && \ 
    CARGO_INSTALL_ROOT=$PWD \
    TARGET=musl \
    LIBZ_SYS_STATIC=1 \
    PKG_CONFIG_ALL_STATIC=1 \
    cargo install --path . --no-track --target armv7-unknown-linux-musleabihf