# ベースイメージを指定（Ubuntuの最新版を使用）
ARG RUST_VERSION
FROM rust:${RUST_VERSION}-slim

# 必要なツールをインストール
RUN apt-get update && \
    apt-get install -y build-essential libssl-dev pkg-config curl && \
    rustup update && \
    rustup default ${RUST_VERSION} && \
    rustup component add rustfmt clippy && \
    cargo install cargo-watch && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# ワークスペースディレクトリの設定
WORKDIR /workspace

# デフォルトでCargoのビルドキャッシュを維持する
ENV CARGO_HOME=/workspace/.cargo

# 必要に応じて環境変数の設定
ENV PATH="/root/.cargo/bin:${PATH}"

# デフォルトのコマンド
CMD ["bash"]
