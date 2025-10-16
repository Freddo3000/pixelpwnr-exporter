FROM rust:alpine as build

WORKDIR /build
RUN apk add --no-cache \
    musl-dev
COPY Cargo.toml ./
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release
COPY src/ src/
RUN touch src/main.rs \
    && cargo build --release

FROM alpine
LABEL authors="Fredrik Falk <freddo@ludd.ltu.se>"

ENV ROCKET_address 0.0.0.0
ENV ROCKET_stats_prefix pixelpwnr
ENV ROCKET_stats_file /app/pixelpwnr.yaml

EXPOSE 8000

WORKDIR /app
COPY LICENSE.txt README.md ./
COPY --from=build /build/target/release/pixelpwnr-exporter .

ENTRYPOINT exec /app/pixelpwnr-exporter