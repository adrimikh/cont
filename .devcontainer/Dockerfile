FROM --platform=linux/amd64 rust:latest

WORKDIR /workspace

RUN apt-get update && \
    apt-get install -y \
        build-essential \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rust-src clippy rustfmt