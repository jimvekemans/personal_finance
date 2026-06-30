# /home/jim/Documents/finance_tracker/backend/Dockerfile
FROM rust:1.80-bookworm AS builder
WORKDIR /usr/src/app

# Download and extract the ADBC PostgreSQL driver shared library
RUN apt-get update && apt-get install -y wget tar
RUN wget https://github.com/apache/arrow-adbc/releases/download/apache-arrow-adbc-12/apache-arrow-adbc-12-linux-amd64.tar.gz -O adbc.tar.gz && \
    mkdir -p /opt/adbc && \
    tar -xzf adbc.tar.gz -C /opt/adbc

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary and the ADBC shared library
COPY --from=builder /usr/src/app/target/release/backend /app/backend
COPY --from=builder /opt/adbc/lib/libadbc_driver_postgresql.so /usr/lib/libadbc_driver_postgresql.so

EXPOSE 8080
CMD ["./backend"]