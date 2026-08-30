FROM node:22-alpine AS web
WORKDIR /app
COPY package.json package-lock.json vite.config.ts tsconfig.json ./
COPY frontend ./frontend
RUN npm ci --ignore-scripts && npm run build

FROM rust:1-alpine AS build
ARG BUILD_SHA=dev
WORKDIR /app
RUN apk add --no-cache musl-dev pkgconfig
COPY Cargo.toml Cargo.lock ./
COPY src ./src
# The floating official major tag keeps pace with dependency MSRV changes.
RUN cargo build --release --locked
RUN printf '%s' "$BUILD_SHA" > /build-sha

FROM alpine:3.21
ARG BUILD_SHA=dev
RUN addgroup -S app && adduser -S app -G app && mkdir -p /data && chown app:app /data
WORKDIR /app
COPY --from=build /app/target/release/integration-changelog-watch /app/server
COPY --from=build /build-sha /app/build-sha
COPY --from=web /app/dist /app/dist
ENV BUILD_SHA=$BUILD_SHA
USER app
EXPOSE 8080
CMD ["/app/server"]
