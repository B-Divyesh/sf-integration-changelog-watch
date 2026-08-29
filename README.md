# Integration Changelog Watch

Turn vendor changes into assigned action cards. It is for engineers who maintain payment, auth, analytics, or messaging integrations.

Add a public changelog or RSS feed, keywords, an owner, the affected dependency version, and a local check command. Scan when you are ready to review notices and create action cards.

## Run locally

```sh
npm ci
npm run build                 # creates dist/
cargo run                     # serves dist/ and API on http://localhost:8080
```

The container uses durable `/data` by default. On a host without that mount, use `DATABASE_URL='sqlite:changelog-watch.db?mode=rwc' cargo run`.

For frontend development, use `npm run dev`. Run `npm test` and `npm run typecheck` for code checks. After `npm run build`, run `npm run test:browser` for browser coverage. The exact claim commands live in `.factory/claims.json`.

Open `http://localhost:8080/demo` for a one-click sandbox. Demo data uses `demo:integration-changelog-watch` browser storage. **Start a private workspace** discards it.

## Workspace API

The container exposes health, workspace, watch, action, and scan endpoints. Create a workspace first. Send its browser-held bearer token with every workspace request. Source URLs must be public `http` or `https` addresses; private network addresses are rejected.

| Method | Path | Result |
| --- | --- | --- |
| `GET` | `/health` | Service and build identity |
| `POST` | `/api/workspaces` | A new browser-held workspace token |
| `GET`, `POST` | `/api/watches` | Read or create workspace watches |
| `POST` | `/api/watches/import` | Validate and replace one to three workspace watches atomically |
| `PUT`, `DELETE` | `/api/watches/:id` | Update or remove one workspace watch |
| `GET` | `/api/actions` | Read assigned action cards |
| `POST` | `/api/actions/:id` | Acknowledge one action card |
| `POST` | `/api/scan` | Scan saved watches when requested |

## Watch files

Use **Export watch file** to download your watches in the CLI JSON schema. Use **Import watch file** to preview one to three watches. A rejected private-workspace import leaves the current watches unchanged. Demo imports stay in demo storage.

## CLI demo

```sh
cargo run -- demo
cargo run -- --help
cargo run -- scan --config examples/watches.json
# Copy the action ID printed in the new Markdown card, then acknowledge it:
cargo run -- ack --config examples/watches.json --id <action-id>
```

`demo` prints the bundled Markdown action-card sample. `scan --config` reads a JSON watch file and writes Markdown cards under `.integration-changelog-watch/actions/`. It stores hashes and acknowledgement state in `.integration-changelog-watch/state.json`. Each card prints its hash-derived action ID; pass that ID to `ack`. Acknowledgement updates both the state file and the Markdown card. The shipped example uses `examples/sample-feed.xml`, so it works without a network request.

## Hosted workspace scope

The hosted workspace holds up to three watches. Use the local CLI for a four-watch mapping.

## Deploy

Build the repository image with `docker build --build-arg BUILD_SHA=dev -t integration-changelog-watch .`. The build stage uses the official `rust:1-alpine` image. It starts with only `PORT` required (default `8080`). The shipped Container App configuration uses one replica and mounts durable Azure Files at `/data`, where SQLite persists at `/data/changelog-watch.db`. A production topology guard closes workspace APIs and restores that configuration if a generic deploy removes it. SQLite uses Azure Files-compatible dot-file locks, so do not raise the replica count.

See `/privacy` and `/terms` for data handling and source rules.

MIT licensed. Built by Param Factory.
