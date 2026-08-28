# Handoff — Integration Changelog Watch

## What shipped

- A Rust/axum container on `PORT` with SQLite persistence, `/health`, watch CRUD, action acknowledgement, and explicit-feed scanning. Each scan hashes feed content and creates action cards only for configured keyword matches.
- A Vite + TypeScript dashboard with an owned action queue, CSV export, mobile layout, visible keyboard focus, offline/error copy, and a separate `/demo` workspace.
- The requested paper-cut diorama identity and original generated hero asset. Source PNG and generation sidecar live in `assets/src/`; the shipped WebP is 58 KB.
- A three-watch free limit and Sociobot checkout, restore, optimistic license storage, and daily verification path for the $39 one-time team tier.
- Privacy, terms, sitemap, robots, favicon, security headers, metadata, and a styled 404 document.

## Run and verify

```sh
npm install
npm test
npm run build
cargo test
cargo run
```

Then open `http://localhost:8080/demo`. The exact production build command is `npm run build`; it writes `dist/index.html`. The container build command is `docker build --build-arg BUILD_SHA=dev -t integration-changelog-watch .`.

Verified in this work order:

- `npm test` — 2 passing claim-tagged tests.
- `npm run build` — passes; initial JavaScript is 5.15 KB gzip and CSS is 2.23 KB gzip.
- `cargo test` — passes, including RSS item parsing.
- A local server smoke verified `GET /health`, `GET /demo` (200), and `POST /api/watches` (201).

Lighthouse and axe were not measured in this container because no browser binary is available. The markup and CSS were checked against the accessibility baseline; run those two browser checks in CI before release.

## Known limits and next steps

- The free dashboard permits three server-side watches. The UI exposes the hosted team-tier purchase but the product must be registered by the factory before checkout can complete.
- Feed parsing intentionally supports common RSS/Atom title and description markup. It does not crawl authenticated or JavaScript-only changelogs, and fetch errors currently produce no action card.
- The server rate limiter uses the first trusted `X-Forwarded-For` value through `SmartIpKeyExtractor`; factory ingress must continue to overwrite that header from the client connection.
