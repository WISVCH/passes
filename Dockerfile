FROM rust:bookworm AS builder

WORKDIR /src
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /src/target/release/passes /usr/local/bin/passes
COPY ./Event.pass /usr/local/bin/Event.pass

ENTRYPOINT ["/usr/local/bin/passes"]
