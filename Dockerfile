FROM rust as builder
WORKDIR /usr/src/journali-api
COPY . .

# Set version environment variable
ARG RUST_APP_VERSION="development"

RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
CMD ["server"]
