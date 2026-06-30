FROM rust:bookworm

WORKDIR /app

# Install cargo-watch for live hot-reloading
RUN cargo install cargo-watch

# We rely on the docker-compose volume to provide the source code
EXPOSE 8080

# Watch the src directory and auto-restart on changes
CMD ["cargo", "watch", "-w", "src", "-x", "run"]