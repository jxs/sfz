FROM rust as builder

ENV APP_HOME /usr/src/app/

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y upx musl-tools

COPY . $APP_HOME
WORKDIR $APP_HOME

RUN cargo build --release --locked --target=x86_64-unknown-linux-musl

FROM alpine:3
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/sfz /app/

ENTRYPOINT ["/app/sfz"]
