FROM rust:latest AS builder

WORKDIR /usr/src/mcbot-rs
COPY . .

RUN cargo install --path .

CMD ["mcbot-rs"]

FROM debian:buster-slim 
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/mcbot-rs /usr/local/bin/mcbot-rs
CMD ["mcbot-rs"]