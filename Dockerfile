FROM rust:latest AS builder

WORKDIR /usr/src/passes
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl-dev

COPY --from=builder /usr/local/cargo/bin/passes /usr/local/bin/passes
CMD ["passes"]