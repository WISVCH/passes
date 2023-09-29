FROM rust:bookworm AS builder

WORKDIR /usr/src/passes
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev

COPY --from=builder /usr/local/cargo/bin/passes /usr/local/bin/passes
CMD ["passes"]