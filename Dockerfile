FROM rust:1.97.1-bookworm AS builder

WORKDIR /src
COPY . .
RUN cargo build --release --locked

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends libssl3 && \
    groupadd --gid 10001 passes && \
    useradd --uid 10001 --gid passes --no-create-home --no-log-init --shell /usr/sbin/nologin passes && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder --chown=10001:10001 /src/target/release/passes /usr/local/bin/passes
COPY --chown=10001:10001 ./Event.pass /usr/local/bin/Event.pass

USER 10001:10001
ENTRYPOINT ["/usr/local/bin/passes"]
