FROM node:22-alpine AS web
WORKDIR /app
COPY package.json vite.config.ts tsconfig.json ./
COPY frontend ./frontend
RUN npm install --ignore-scripts && npm run build

FROM rust:1.85-alpine AS build
ARG BUILD_SHA=dev
WORKDIR /app
RUN apk add --no-cache musl-dev pkgconfig
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM alpine:3.21
ARG BUILD_SHA=dev
RUN addgroup -S app && adduser -S app -G app && mkdir -p /data && chown app:app /data
WORKDIR /app
COPY --from=build /app/target/release/integration-changelog-watch /app/server
COPY --from=web /app/dist /app/dist
ENV BUILD_SHA=$BUILD_SHA
USER app
EXPOSE 8080
CMD ["/app/server"]
