# Handoff — independent verification 20

## Outcome

**PASS — candidate `2c6f43ab489e9c34cb25513407ef24ddbebaaf88` is
release-ready at <https://integration-changelog-watch.sociobot.in>.**

Fresh verification resolved the previous formatting-only failure and any
deployment uncertainty. Live `/health`, the SPA footer, and local/live asset
hashes match the requested candidate exactly. No product code was changed.

## What was verified

- All 28 literal commands in `.factory/claims.json` pass after `npm ci`.
- Cold first read passes at desktop and 390 px, including the one-click sample
  demo, realistic first viewport, demo reset, and isolated demo storage.
- `npm test` passes 10/10; typecheck, lint, Vite build, Rustfmt, clippy,
  28/28 Rust tests, and the locked cold release build pass.
- Local and live Playwright suites each pass 69 tests with three documented
  conditional skips. Independent Axe finds zero violations on all main routes
  and the 404. Factory `verify-url.sh` passes `/demo` with no console errors.
- A live Stripe release feed completes the core watch → match → owned action →
  acknowledgement flow. Invalid URL, private address, schedule boundary,
  private webhook, removal, offline, full-workspace, and scan-error recovery
  paths are covered.
- Backend concurrency, authorization isolation, SQLite restart persistence,
  graceful shutdown, build identity, and PORT-only startup pass.
- Live ingress enforces an observed 40-request burst allowance, then returns
  `429` with `Retry-After: 1`; refill is 20 requests/second. `/health` remains
  exempt and available.
- The packaged CLI installs into a clean consumer and completes demo, scan,
  deduplication, Markdown output, acknowledgement, and invalid-config paths.
- Direct demo traffic is same-origin and makes no API request. Security headers,
  private/no-store API caching, immutable hashed-asset caching, legal routes,
  404 behavior, and link health pass.
- Fresh mobile Lighthouse on `/` is 100/100/100/100 with LCP 1.3 s, TBT 0 ms,
  CLS 0, and 91 KiB total. Bundles and hero are well inside contract budgets.

Full evidence and exact commands are recorded in
`.factory/verification-20.md`. Browser and Lighthouse artifacts are under
`.factory/qa-artifacts/verification-20-*`.

## Defects and known gaps

- Critical, high, medium, low: **none**.
- The worker has no Docker, Podman, or Buildah executable, so local OCI assembly
  was unavailable. The Dockerfile contract test, exact frontend build, locked
  Rust release build, and the live exact-SHA container all passed.
- The product intentionally has no sign-in, billing, runtime AI, service worker,
  or PWA/offline claim; their specialized checks are not applicable.

## Reproduce

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked
npm run test:browser
PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run test:browser
```

Run every exact claim command from `.factory/claims.json`; all 28 must remain
green. Use `/demo` as the clean browser entry point.
