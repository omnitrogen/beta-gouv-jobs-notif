FROM rust:latest AS builder
COPY . .
RUN cargo install --path .

FROM debian:buster-slim AS runner
RUN apt-get update && apt-get install -y \
    libssl-dev
COPY --from=builder /usr/local/cargo/bin/beta-gouv-jobs-notif /usr/local/bin/beta-gouv-jobs-notif
ENV RUST_LOG=main
CMD ["beta-gouv-jobs-notif"]