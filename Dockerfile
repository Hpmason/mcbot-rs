ARG APP=mcbot-rs
FROM rust:latest AS builder
ARG APP
WORKDIR /usr/src/${APP}
COPY . .
RUN cargo build --release

FROM debian:buster-slim 
ARG APP
COPY --from=builder /usr/src/${APP}/target/release/${APP} /usr/local/bin/${APP}
CMD ${APP}
