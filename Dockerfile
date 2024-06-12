# Etapa de construcción
FROM rust:1.63 as builder

WORKDIR /usr/src/phserver

COPY . .

RUN cargo build --release

# Etapa de producción
FROM debian:buster-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/phserver/target/release/phserver /usr/local/bin/phserver

CMD ["phserver"]
