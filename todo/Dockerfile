# Docker Official の Rust イメージを使う
FROM rust:1.55 AS builder

# build stage -----------------------------------------------------------------
# /todo でビルドする
WORKDIR /todo

# Cargo.toml だけ先にイメージに追加する
COPY Cargo.toml Cargo.toml

# ビルドするために何もしないソースコードを追加する
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs

# 依存クレートをビルドする
RUN cargo build --release

# アプリケーションのコードをイメージにコピーする
COPY ./src ./src
COPY ./templates ./templates

# 先程ビルドした生成物のうと、アプリケーションのもののみを削除する
RUN rm -f target/release/deps/todo*

# 改めてアプリケーションををビルドする
RUN cargo build --release

# パスの通った場所にインストールする。
RUN cargo install --path .

# release stage ---------------------------------------------------------------
# リリース用のイメージを用意する
# FROM alpine
# FROM rust:alpine3.14
# FROM debian:10.4
FROM rust:1.55-slim-buster
WORKDIR /todo
# builder イメージから todo のみをコピーして /bin に配置する
COPY --from=builder /todo/target/release/todo /usr/local/bin/todo
