FROM blackdex/rust-musl:x86_64-musl AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM blackdex/rust-musl:x86_64-musl AS cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


FROM blackdex/rust-musl:x86_64-musl AS builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/recipe.json recipe.json
RUN cargo build --release


FROM alpine:latest AS minimizer
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/uni-sys-rs /app/uni-sys-rs
RUN apk add upx
RUN upx --best --lzma /app/uni-sys-rs


FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/Config.toml /app/Config.toml
COPY --from=minimizer /app/uni-sys-rs /app/uni-sys-rs
EXPOSE 465 2122
CMD [ "/app/uni-sys-rs" ]
