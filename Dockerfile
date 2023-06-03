FROM node:alpine as web-builder
WORKDIR /usr/src
COPY client/package.json .
COPY client/package-lock.json .
RUN npm install
COPY client/ .
RUN npm run build

FROM rust:alpine as builder
WORKDIR /usr/src/api-service
RUN apk add --no-cache musl-dev

# Update this to whatever database provider you use
RUN apk add --no-cache postgresql-dev postgresql
ENV RUSTFLAGS="-C target-feature=-crt-static" 
RUN cargo install diesel_cli --no-default-features --features sqlite

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

# You don't want to run migrations here in prod. This is here for a working example app
COPY migrations/ migrations
ENV DATABASE_URL=postgres://db_admin_user:password123@127.0.0.1/movierama
RUN diesel database setup

COPY src/ src
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
RUN apk add --no-cache postgresql-dev postgresql libgcc ca-certificates 
COPY --from=builder /usr/src/api-service/target/release/actix_svelte_template /app
# COPY --from=builder /usr/src/api-service/db.sqlite3 /app/db.sqlite3

COPY --from=web-builder /usr/src/build /app/static

ENV STATIC_FILE_PATH=/app/static PORT=8080
# Replace with your database connection string if not using sqlite
ENV DATABASE_URL=postgres://db_admin_user:password123@127.0.0.1/movierama
EXPOSE 8080
CMD ["/app/actix_svelte_template"]
