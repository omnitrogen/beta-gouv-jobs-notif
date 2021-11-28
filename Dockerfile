FROM rust:latest AS builder
COPY . .
RUN cargo install --path .

FROM debian:buster-slim AS runner
RUN apt-get update \
    && apt-get install -y ca-certificates libssl-dev sqlite3 libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates
COPY --from=builder /usr/local/cargo/bin/beta-gouv-jobs-notif /usr/local/bin/beta-gouv-jobs-notif
ENV RUST_LOG=main
CMD ["beta-gouv-jobs-notif"]