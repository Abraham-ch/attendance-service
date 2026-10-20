# Builds the API as a static musl binary and ships it on distroless (~2 MB base, has CA certs).
FROM rust:1-alpine AS build
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static
ENV SQLX_OFFLINE=true OPENSSL_STATIC=1
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY .sqlx .sqlx
COPY api api
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --bin attendance-service \
    && cp target/release/attendance-service /attendance-service

FROM gcr.io/distroless/static-debian12:nonroot
COPY --from=build /attendance-service /attendance-service
EXPOSE 8080
ENTRYPOINT ["/attendance-service"]
