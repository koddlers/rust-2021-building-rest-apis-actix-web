FROM rust:1.79.0-slim

WORKDIR /usr/src/actix-web
COPY . .

RUN apt-get update && apt-get install -y pkg-config && rm -rf /var/lib/apt/lists/*
RUN cargo install --path .
COPY /usr/local/cargo/bin/myapp /usr/local/bin/myapp
CMD ["myapp"]

