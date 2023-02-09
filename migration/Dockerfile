# ------------- build ----------------
FROM clux/muslrust:stable AS builder

user root

WORKDIR /home/rust

COPY Cargo.toml ./

RUN mkdir -p src
# 適当な実行ファイルの生成
RUN touch src/lib.rs
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
# 依存関係のみ先にコンパイルして、キャッシュしておく
RUN cargo build --release

# ここでちゃんとけせてないと正しくバイナリが生成されない
RUN rm target/x86_64-unknown-linux-musl/release/deps/migration-* target/x86_64-unknown-linux-musl/release/migration target/x86_64-unknown-linux-musl/release/deps/libmigration-*
RUN rm -rf src

COPY src ./src
RUN cargo build --release

# ------------- runtime ----------------
FROM scratch
ENV ROCKET_ADDRESS=0.0.0.0
WORKDIR /app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/migration .
EXPOSE 8000
ENTRYPOINT [ "./migration" ]
