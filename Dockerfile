FROM rust:latest

RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rust-src rustfmt clippy rust-analyzer
