FROM rust:latest AS builder

# Install ca-certificates package
RUN apt-get update && apt-get install -y ca-certificates openssl

WORKDIR /usr/src/passes
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim


# Copy ca-certificates from builder image
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

COPY --from=builder /usr/local/cargo/bin/passes /usr/local/bin/passes
CMD ["passes"]