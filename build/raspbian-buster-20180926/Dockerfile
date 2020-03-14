FROM resin/raspberry-pi-debian:buster-20180926

ENV ZLIB_VERSION=1.2.11 \
    OPENSSL_VERSION=1.0.2r \
    RUST_VERSION=1.42.0 \
    PATH=/home/rust/.cargo/bin:$PATH

ENV OPENSSL_DIR=/usr/local/musl/ \
    OPENSSL_INCLUDE_DIR=/usr/local/musl/include/ \
    DEP_OPENSSL_INCLUDE=/usr/local/musl/include/ \
    OPENSSL_LIB_DIR=/usr/local/musl/lib/

RUN apt-get update && apt-get install -y \
    sudo \
    file \
    curl \
    cmake \
    musl-dev \
    musl-tools \
    libsqlite-dev \
    libssl-dev \
    linux-libc-dev \
    pkgconf \
    xutils-dev

RUN echo "Building OpenSSL" && \
    ls /usr/include/linux && \
    sudo mkdir -p /usr/local/musl/include && \
    sudo ln -s /usr/include/linux /usr/local/musl/include/linux && \
    sudo ln -s /usr/include/x86_64-linux-gnu/asm /usr/local/musl/include/asm && \
    sudo ln -s /usr/include/asm-generic /usr/local/musl/include/asm-generic && \
    cd /tmp && \
    curl -LO "https://www.openssl.org/source/openssl-$OPENSSL_VERSION.tar.gz" && \
    tar xvzf "openssl-$OPENSSL_VERSION.tar.gz" && cd "openssl-$OPENSSL_VERSION" && \
    CC=musl-gcc ./Configure no-shared no-zlib -fPIC --prefix=/usr/local/musl -DOPENSSL_NO_SECURE_MEMORY linux-x86_64 && \
    C_INCLUDE_PATH=/usr/local/musl/include/ make depend && \
    C_INCLUDE_PATH=/usr/local/musl/include/ make && \
    sudo make install && \
    sudo rm /usr/local/musl/include/linux /usr/local/musl/include/asm /usr/local/musl/include/asm-generic && \
    rm -r /tmp/*

RUN echo "Building zlib" && \
    cd /tmp && \
    curl -LO "http://zlib.net/zlib-$ZLIB_VERSION.tar.gz" && \
    tar xzf "zlib-$ZLIB_VERSION.tar.gz" && cd "zlib-$ZLIB_VERSION" && \
    CC=musl-gcc ./configure --static --prefix=/usr/local/musl && \
    make && sudo make install && \
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
    OPENSSL_STATIC=1 \
    cargo install --path . --no-track --target armv7-unknown-linux-musleabihf