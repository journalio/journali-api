FROM rust as builder
WORKDIR /usr/src/journali-api
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
CMD ["server"]
