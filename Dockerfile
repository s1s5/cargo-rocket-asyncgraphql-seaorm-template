# ------------- build ----------------
FROM clux/muslrust:stable AS builder

user root

WORKDIR /home/rust

COPY Cargo.toml Cargo.lock ./
COPY entity/Cargo.toml ./entity/
COPY api/Cargo.toml ./api/
COPY migration/Cargo.toml ./migration/
RUN mkdir entity/src && touch entity/src/lib.rs && mkdir api/src && touch api/src/lib.rs && mkdir migration/src && touch migration/src/lib.rs

RUN mkdir -p src
# 適当な実行ファイルの生成
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
# 依存関係のみ先にコンパイルして、キャッシュしておく
RUN cargo build --release

# ここでちゃんとけせてないと正しくバイナリが生成されない
RUN rm target/x86_64-unknown-linux-musl/release/deps/{{crate_name}}-* target/x86_64-unknown-linux-musl/release/{{crate_name}} target/x86_64-unknown-linux-musl/release/deps/api-* target/x86_64-unknown-linux-musl/release/deps/libapi-* target/x86_64-unknown-linux-musl/release/deps/entity-* target/x86_64-unknown-linux-musl/release/deps/libentity-*
RUN rm src/main.rs
RUN rm -rf entity/src api/src src

COPY entity/src ./entity/src
COPY api/src ./api/src
COPY src ./src
RUN cargo build --release

# ------------- runtime ----------------
FROM scratch
ENV ROCKET_ADDRESS=0.0.0.0
WORKDIR /app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/{{project-name}} .
EXPOSE 8000
ENTRYPOINT [ "./{{project-name}}" ]
