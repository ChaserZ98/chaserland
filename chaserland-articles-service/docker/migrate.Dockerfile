FROM rust:1.86-alpine AS builder-base

RUN apk update \
    && apk add --no-cache musl-dev protobuf-dev \
    && rm -rf /var/cache/apk/*

FROM builder-base AS builder

WORKDIR /chaserland/chaserland-articles-service/crates/migrate

COPY . /chaserland

RUN cargo build --release

FROM alpine:latest AS runner

WORKDIR /app

COPY --from=builder /chaserland/target/release/chaserland-articles-service-migrate /app/migrate

ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=postgres
ENV POSTGRES_HOST=postgres
ENV POSTGRES_PORT=5432
ENV POSTGRES_DB=chaserland_article

CMD ["./migrate"]
