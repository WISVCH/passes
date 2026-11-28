FROM rust:1.100.0-bookworm@sha256:75d26c23c24cc6ea1cdabf0714a6bff8fd7bf3248716043fd7ae86dac7f41b94 AS chef
RUN cargo install --locked cargo-chef --version 0.1.73

WORKDIR /src
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /src/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked

FROM debian:bookworm-slim@sha256:e67eeead736ff7bbe258ef088bff683635c561b1fcfefad0806c1df4ab49ce0e
RUN apt-get update && apt-get install -y --no-install-recommends libssl3 && \
    groupadd --gid 10001 passes && \
    useradd --uid 10001 --gid passes --no-create-home --no-log-init --shell /usr/sbin/nologin passes && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder --chown=10001:10001 /src/target/release/passes /usr/local/bin/passes
COPY --chown=10001:10001 ./Event.pass /usr/local/bin/Event.pass

USER 10001:10001
ENTRYPOINT ["/usr/local/bin/passes"]
