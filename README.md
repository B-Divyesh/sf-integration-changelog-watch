# Integration Changelog Watch

Turn vendor changelog changes into owned integration actions. It is for engineers who maintain payment, auth, analytics, or messaging integrations.

Add an explicit public changelog or RSS feed, matching words, an owner, and the local check command. A scan stores a content hash and creates a concise action card for new matching notices. It does not access private portals or claim to detect undocumented changes.

## Run locally

```sh
npm install
npm run build                 # creates dist/
cargo run                     # serves dist/ and API on http://localhost:8080
```

For frontend development, use `npm run dev`. Run `npm test` for shipped sample and claim checks. Run `cargo test` for server compilation and unit checks.

Open `http://localhost:8080/demo` for a one-click sandbox. Demo data uses the `demo:integration-changelog-watch` browser storage namespace and is discarded by **Start for real**.

## API

The container exposes `GET /health`, `GET|POST /api/watches`, `GET /api/actions`, `POST /api/actions/:id`, and `POST /api/scan`. Scans respect the configured public source URL; use only feeds whose terms permit access. The free server API accepts three watches.

## Deploy

Build the repository image with `docker build --build-arg BUILD_SHA=dev -t integration-changelog-watch .`. It starts with only `PORT` required (default `8080`) and persists its SQLite database at `/data/changelog-watch.db` when that directory is mounted.

The optional $39 one-time team tier uses the Sociobot hosted checkout and license verification. Sociobot/Dodo is merchant of record. See `/privacy` and `/terms`.

MIT licensed. Built by Param Factory.
