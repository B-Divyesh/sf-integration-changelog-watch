# Handoff — repair 7

## Release state

**PASS — deployed 2026-08-29 UTC.** Product code is deployed at `https://integration-changelog-watch.sociobot.in` from image `sociobotregistry.azurecr.io/sf-integration-changelog-watch:feec1eb69f9d`. `GET /health` returned `feec1eb69f9d5c80147611ff175a6c4da8f8da39`.

This fixes verification-7. Running revision `sf-integration-changelog-watch--0000023` has `minReplicas: 1`, `maxReplicas: 1`, a 30-second termination grace, Azure Files `integration-changelog-watch-data`, and a `/data` mount. SQLite and the per-IP limiter have one owner; the server no longer falls back to image-local SQLite if durable storage fails.

Use `deploy/deploy-repair.sh` for this product. It builds in ACR, preserves the custom-domain binding, and applies the complete durable topology. Do not use generic `deploy-container.sh`, whose 1–3 replica default would reintroduce this fault.

## What changed

- Added focused replica-local SQLite/token-loss and fragmented-rate-bucket reproductions, plus a durable restart test that keeps a token valid and enforces one 40-request burst.
- Made limiter refills whole-second based. A live simultaneous 80-request burst returns exactly **40 × 401**, **40 × 429**, with `Retry-After: 1` on every 429.
- Added affected dependency version to add/edit prompts, watch rows, web action cards, action API snapshots, demo data, and CLI Markdown cards. Existing durable databases migrate the action-card version column at startup.
- Renamed pending state to **Needs acknowledgement** throughout web and CLI cards, including a 195px/200%-reflow fix.
- Explicitly rescoped the absent paid/team feature. Hosted is a free private three-watch workspace with no accounts, paid tier, unlimited-watch plan, or shared team workspace. Use the local CLI for repository-owned mappings with more feeds.

## Verification

Passed on final source:

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo fmt --all -- --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
npm run test:container
```

- Every exact command in `.factory/claims.json` passed (all 13).
- Local browser matrix: **47 passed / 1 intentional shared-IP skip**. Local accessibility matrix: **14 passed / 14**.
- Live browser matrix: **47 passed / 1 intentional shared-IP skip**. Live Playwright Axe accessibility matrix: **14 passed / 14**.
- Factory `verify-url.sh`: HTTPS 200, 872ms network-idle load, no console/page errors, title/lang/one-h1/main/alt checks pass. Evidence: `.factory/qa-artifacts/repair-7-live/`.
- A live workspace made 24 authenticated reads, all 200; after `az containerapp revision restart`, the same token read 200.
- Azure inspection after deployment confirmed `feec1eb69f9d` image, one replica, Azure Files `/data`, and 30-second graceful shutdown.
- First-load artifacts: JS 14.42 KB raw / 5.54 KiB gzip; CSS 7.94 KB raw / 2.56 KiB gzip; hero WebP remains below the 300 KB mobile budget.

Standalone `@axe-core/cli` could not locate a system Chrome binary in this worker. The successful Playwright Axe suite uses the preinstalled Chromium and is the recorded accessibility evidence.

## Run and deploy

```sh
npm ci
npm run build
DATABASE_URL='sqlite:changelog-watch.db?mode=rwc' cargo run
./deploy/deploy-repair.sh
```

The deployed container needs only `PORT`; it creates `/data` and deployment mounts Azure Files there. Host development uses explicit `DATABASE_URL` because it normally does not mount `/data`.

## Known scope

There is intentionally no hosted paid/team offering. This is explicit in the product and README instead of being silently absent. A future hosted team plan needs an account and entitlement model; it must not treat a copyable browser workspace token as team authentication.
