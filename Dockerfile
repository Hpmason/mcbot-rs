ARG APP=mcbot-rs
FROM rust:latest AS builder

WORKDIR /usr/src/${APP}
COPY . .
RUN cargo install --path .

FROM debian:buster-slim 
COPY --from=builder /usr/local/cargo/bin/${APP} /usr/local/bin/${APP}
CMD ["mcbot-rs"]