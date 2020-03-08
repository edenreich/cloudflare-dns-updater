FROM ubuntu:18.04 as build-environment

ENV PATH=/home/rust/.cargo/bin:/usr/local/musl/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

ENV ZLIB_VERSION=1.2.11
ENV POSTGRESQL_VERSION=11.2
ENV MDBOOK_VERSION=0.2.1

RUN apt-get update && \
    apt-get install -y \
        build-essential \
        cmake \
        curl \
        file \
        git \
        musl-dev \
        musl-tools \
        libpq-dev \
        libsqlite-dev \
        libssl-dev \
        linux-libc-dev \
        pkgconf \
        sudo \
        xutils-dev \
        gcc-multilib-arm-linux-gnueabihf \
        && \
    apt-get clean && rm -rf /var/lib/apt/lists/* && \
    useradd rust --user-group --create-home --shell /bin/bash --groups sudo && \
    curl -LO https://github.com/rust-lang-nursery/mdBook/releases/download/v$MDBOOK_VERSION/mdbook-v$MDBOOK_VERSION-x86_64-unknown-linux-musl.tar.gz && \
    tar xf mdbook-v$MDBOOK_VERSION-x86_64-unknown-linux-musl.tar.gz && \
    mv mdbook /usr/local/bin/ && \
    rm -f mdbook-v$MDBOOK_VERSION-x86_64-unknown-linux-musl.tar.gz

RUN sudo ln -s "/usr/bin/g++" "/usr/bin/musl-g++"

RUN echo "Building OpenSSL" && \
    ls /usr/include/linux && \
    sudo mkdir -p /usr/local/musl/include && \
    sudo ln -s /usr/include/linux /usr/local/musl/include/linux && \
    sudo ln -s /usr/include/x86_64-linux-gnu/asm /usr/local/musl/include/asm && \
    sudo ln -s /usr/include/asm-generic /usr/local/musl/include/asm-generic && \
    cd /tmp && \
    curl -LO "https://www.openssl.org/source/openssl-1.0.2r.tar.gz" && \
    tar xvzf "openssl-1.0.2r.tar.gz" && cd "openssl-1.0.2r" && \
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

RUN echo "Building libpq" && \
    cd /tmp && \
    curl -LO "https://ftp.postgresql.org/pub/source/v$POSTGRESQL_VERSION/postgresql-$POSTGRESQL_VERSION.tar.gz" && \
    tar xzf "postgresql-$POSTGRESQL_VERSION.tar.gz" && cd "postgresql-$POSTGRESQL_VERSION" && \
    CC=musl-gcc CPPFLAGS=-I/usr/local/musl/include LDFLAGS=-L/usr/local/musl/lib ./configure --with-openssl --without-readline --prefix=/usr/local/musl && \
    cd src/interfaces/libpq && make all-static-lib && sudo make install-lib-static && \
    cd ../../bin/pg_config && make && sudo make install && \
    rm -r /tmp/*

ENV OPENSSL_DIR=/usr/local/musl/ \
    OPENSSL_INCLUDE_DIR=/usr/local/musl/include/ \
    DEP_OPENSSL_INCLUDE=/usr/local/musl/include/ \
    OPENSSL_LIB_DIR=/usr/local/musl/lib/ \
    OPENSSL_STATIC=1 \
    PQ_LIB_STATIC_X86_64_UNKNOWN_LINUX_MUSL=1 \
    PG_CONFIG_X86_64_UNKNOWN_LINUX_GNU=/usr/bin/pg_config \
    PKG_CONFIG_ALLOW_CROSS=true \
    PKG_CONFIG_ALL_STATIC=true \
    LIBZ_SYS_STATIC=1 \
    TARGET=musl

RUN echo '%sudo   ALL=(ALL:ALL) NOPASSWD:ALL' > /etc/sudoers.d/nopasswd
RUN chown -R rust:rust /home/rust
USER rust
RUN mkdir /home/rust/src
WORKDIR /home/rust/src
RUN mkdir /home/rust/libs
COPY --chown=rust:rust build/cargo-config.toml /home/rust/.cargo/config

RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- -y --default-toolchain stable && \
    rustup target add x86_64-unknown-linux-musl && \
    rustup target add armv7-unknown-linux-musleabihf

RUN cargo install -f cargo-audit && \
    cargo install -f cargo-deny && \
    rm -rf /home/rust/.cargo/registry/

COPY --chown=rust:rust src src
COPY --chown=rust:rust Cargo.lock Cargo.lock
COPY --chown=rust:rust Cargo.toml Cargo.toml 

FROM build-environment as normal-build

RUN cargo build --release

RUN mkdir bin && cp target/x86_64-unknown-linux-musl/release/cloudflare bin/cloudflare

FROM build-environment as arm7-build

RUN cargo build --release --target armv7-unknown-linux-musleabihf

RUN mkdir bin && cp target/armv7-unknown-linux-musleabihf/release/cloudflare bin/cloudflare
