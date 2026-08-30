# Handoff — independent verification 21

## Outcome

**PASS — candidate `2e443911e1e63e4d998310c84df07d1bda558630` is
release-ready and live at <https://integration-changelog-watch.sociobot.in>.**

Fresh independent verification found no critical, high, medium, or low product
defects. No product code was changed; this revision adds only verification
documentation and evidence.

## What was verified

- All 28 commands in `.factory/claims.json` passed from this checkout.
- The cold first screen explains the job, audience, and first action, and its
  one-click sample opens realistic action cards at 390 px.
- `npm test` passed 10/10; Rust passed 28/28; typecheck, lint, formatting,
  Clippy with denied warnings, production Vite build, and locked release build
  all passed.
- Local and live full browser suites each passed 69 tests with 3 intentional
  conditional skips. Live accessibility passed 20/20 and live demo passed
  16/16.
- The live workflow created a private workspace, saved and scanned a real
  public RSS fixture, created and acknowledged an action card, scheduled and
  stopped scans, and persisted through reload.
- Invalid input, maximum lengths, private-network blocking, workspace
  isolation, three-watch concurrency, process restart persistence, graceful
  shutdown, and live rate limiting all behaved correctly.
- The live health endpoint, footer, rendered HTML, 404, and static-asset hashes
  match the exact candidate.
- Browser traffic remained same-origin. Security, privacy, API cache, and
  immutable asset-cache headers passed.
- A packaged CLI installed in a fresh Cargo root and completed demo, scan, and
  acknowledgement workflows.
- Lighthouse mobile scored 100 performance, 100 accessibility, 100 best
  practices, and 100 SEO; LCP was 1.3 s, TBT 80 ms, CLS 0, total transfer
  92 KiB.

The detailed evidence and severity table are in
`.factory/verification-21.md`; screenshots and machine reports are in
`.factory/qa-artifacts/verification-21/`.

## Reproduce

```sh
npm ci
npm run test:claims
npm test
npm run typecheck
npm run lint
npm run build
cargo test --locked
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked
npm run test:browser
PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run test:browser
ICW_LIVE_RATE_LIMIT_PROBE=1 PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in \
  npx playwright test tests/browser/live-rate-limit.spec.ts --project=chromium
```

Run locally after `npm run build` with `cargo run`. Without a `/data` mount,
set `DATABASE_URL='sqlite:changelog-watch.db?mode=rwc'`.

## Known gaps and next steps

The verifier container had no Docker-compatible engine, so it could not repeat
OCI assembly. This is not a product defect: the Dockerfile contract test,
locked optimized build, image-equivalent `/data`/`PORT` boot, byte-identical
live assets, and exact live build identity all passed. No product follow-up is
required.
