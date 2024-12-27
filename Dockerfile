FROM rust:1.80.1-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app/clear_tools
COPY . .
RUN cargo install --path . --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/clear_tools/target/x86_64-unknown-linux-musl/release/clear_tools /clear_tools
CMD ["/clear_tools"]
