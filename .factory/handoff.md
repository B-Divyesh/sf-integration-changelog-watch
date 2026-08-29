# Handoff — repair 6

## Release state

**PASS — deployed 2026-08-29 UTC.** The release at `https://integration-changelog-watch.sociobot.in` is revision `sf-integration-changelog-watch--0000020`, image `sociobotregistry.azurecr.io/sf-integration-changelog-watch:aa749bc010f8`, and `GET /health` returns `aa749bc010f82cc5241a629938366ffc70ac2a86`.

Repair commits: `844e218` (topology, shutdown, identity, claims), `ebe6ea8`/`bdcdeed` (Azure Files SQLite VFS), and `aa749bc` (demo race and 200% reflow).

## What changed

- Added `deploy/containerapp.yaml`: `minReplicas: 1`, `maxReplicas: 1`, read/write Azure Files at `/data`, and a 30-second termination grace period. Production applies that topology.
- The default SQLite URI is `sqlite:/data/changelog-watch.db?mode=rwc&vfs=unix-dotfile`, the Azure Files-compatible lock mode. The service remains single-replica, so workspace state and the in-process rate limiter have one owner.
- Added graceful SIGTERM/SIGINT draining and a regression test that proves an active server exits gracefully.
- Reproduced the verifier failure deterministically: a workspace is unknown to a second SQLite replica, and two in-memory buckets accept 80 requests where one accepts 40. The versioned topology test prevents the second replica in release.
- Added dynamic build identity injection so SPA routes and the product 404 footer match `/health`.
- Completed claims for demo transitions, shipped CLI mapping isolation, PORT-only startup, and one-replica durable topology. `.factory/claims.json` now has 13 entries with exact regression commands.
- Fixed a late real-workspace response that could overwrite demo storage after switching to sample data; added a delayed-read regression. The long build identity now wraps at 200% reflow.

During deployment, the Azure Files share held only a verified zero-byte failed-bootstrap database. It was removed before successful initialization; no workspace data was present. The new durable database is 32,768 bytes and a workspace survived a real revision restart.

## Verification

Clean install: `npm ci` installed 60 packages with zero audit findings.

- `npm test` — 3 passed.
- `npm run typecheck`, `npm run lint`, and `npm run build` — passed; `dist/` generated (JS 13.79 KB raw / 5.35 KB gzip; CSS 7.85 KB raw / 2.55 KB gzip).
- `cargo fmt --all -- --check`, `cargo test --locked` (16 passed), `cargo clippy --all-targets --locked -- -D warnings`, and `cargo build --release --locked` — passed.
- `npm run test:browser` — 47 passed, 1 intentional mobile duplicate-burst skip. `npm run test:a11y` — 14 passed, including Axe serious/critical checks on desktop and 390px mobile.
- `cargo package --locked --allow-dirty --no-verify` — passed. A clean packed consumer installation ran `--help`, `demo`, and shipped local-feed scan, producing one persisted action card.
- Every exact claim command passed after the clean install. A local release process served `/health` and exited after SIGTERM.

Live verification:

- Factory `verify-url.sh` checked HTTPS 200, title, `lang`, one h1, main, image alt text, desktop/mobile screenshots, and zero console errors (883 ms load).
- Full live Playwright: 47 passed / 1 intentional skip; live accessibility: 14 passed. This covers keyboard, 390px mobile, 195px/200% reflow, privacy, demo, and 404 identity.
- A fresh workspace made 48 authenticated reads with **48 × 200 and 0 × 401**. The same bearer token returned 200 after a real revision restart.
- A 120-request single-client burst yielded **42 × 200 and 78 × 429**; every 429 had `Retry-After: 1`.
- Azure confirms one running replica, both scale bounds at 1, `integration-changelog-watch-data` mounted at `/data`, a 30-second termination grace, and the final image/build identity above.
- Live response headers include CSP with header `frame-ancestors`, HSTS, `nosniff`, strict referrer policy, permissions policy, and `no-cache` HTML.

## Run and deploy

```sh
npm ci
npm run build
cargo run
npm test
cargo test --locked
npm run test:browser
```

The factory Container App uses `deploy/containerapp.yaml` together with the current image. It needs only `PORT=8080`; the binary supplies its durable database default.

## Known gaps

The product is not a PWA and makes no offline-reload/update promise. Demo mode has an explicit offline scan message covered by browser tests. There is no account, payment, or runtime AI feature in the researched scope.
