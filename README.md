# Integration Changelog Watch

Turn vendor changelog changes into owned integration actions. It is for engineers who maintain payment, auth, analytics, or messaging integrations.

Add a public changelog or RSS feed, matching words, an owner, the affected dependency version, and a local check command. Scan when you want to review notices and turn matches into action cards.

## Run locally

```sh
npm ci
npm run build                 # creates dist/
cargo run                     # serves dist/ and API on http://localhost:8080
```

The container starts with no database setting because it mounts durable `/data`. On a host without that mount, use an explicit local path: `DATABASE_URL='sqlite:changelog-watch.db?mode=rwc' cargo run`.

For frontend development, use `npm run dev`. Run `npm test` for shipped sample and container-toolchain checks. Run `npm run typecheck` for TypeScript checks, `cargo test` for server checks, and `npm run test:browser` after `npm run build` for browser, keyboard, mobile, privacy, and accessibility coverage. The exact browser commands for each published claim live in `.factory/claims.json`.

Open `http://localhost:8080/demo` for a one-click sandbox. Demo data uses the `demo:integration-changelog-watch` browser storage namespace and is discarded by **Start for real**.

## Workspace API

The container exposes `GET /health`, `POST /api/workspaces`, `GET|POST /api/watches`, `PUT|DELETE /api/watches/:id`, `GET /api/actions`, `POST /api/actions/:id`, and `POST /api/scan`. Create a workspace first, then send its browser-held bearer token with every workspace request. Source URLs must be public `http` or `https` addresses; private network addresses are rejected.

## CLI demo

```sh
cargo run -- demo
cargo run -- --help
cargo run -- scan --config examples/watches.json
# Copy the action ID printed in the new Markdown card, then acknowledge it:
cargo run -- ack --config examples/watches.json --id <action-id>
```

`demo` prints the bundled Markdown action-card sample. `scan --config` reads a repository-owned JSON watch mapping, writes new Markdown action cards under `.integration-changelog-watch/actions/`, and stores hashes plus acknowledgement state in `.integration-changelog-watch/state.json`. Each card prints its hash-derived action ID; pass that ID to `ack`. Acknowledgement updates both the state file and the Markdown card. The shipped example uses the bundled `examples/sample-feed.xml`, so it works without a network request.

## Hosted scope

The hosted dashboard is deliberately a free, private, three-watch workspace. It has no account system, shared team workspaces, unlimited-watch tier, or paid plan. A browser-held workspace token is not a team identity or billing entitlement. Teams can keep larger mappings and action cards in their repository with the local CLI. Hosted team collaboration is not offered or implied.

## Deploy

Build the repository image with `docker build --build-arg BUILD_SHA=dev -t integration-changelog-watch .`. The container uses the current stable Rust image and builds with `cargo build --release --locked`. It starts with only `PORT` required (default `8080`). The shipped Container App configuration uses one replica and mounts durable Azure Files at `/data`, where SQLite persists at `/data/changelog-watch.db`. A production topology guard closes workspace APIs and restores that configuration if a generic deploy removes it. SQLite uses Azure Files-compatible dot-file locks, so do not raise the replica count.

See `/privacy` and `/terms` for data handling and source rules.

MIT licensed. Built by Param Factory.
