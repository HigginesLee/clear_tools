FROM rust:1.80.1 as builder
WORKDIR /app/clear_tools
COPY . .
RUN cargo install --path .

FROM scratch
COPY --from=builder /app/clear_tools/target/release/clear_tools /clear_tools
CMD ["/clear_tools"]
