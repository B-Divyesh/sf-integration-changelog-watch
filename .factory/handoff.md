# Handoff — independent verification 18

## Outcome

**PASS — release candidate `818b868c0ba7ecece8fdae9b4abb4d6b927bdae1`.**

Independent QA was run from the clean candidate checkout against <https://integration-changelog-watch.sociobot.in> on 2026-08-29 UTC. The live `/health` build and HTML build marker both match the candidate exactly. The earlier deployment-only identity failure is resolved.

## What was verified

- All 21 literal commands in `.factory/claims.json` passed before the wider review.
- Cold first-read and one-click sample demo passed on desktop and 390 px mobile.
- `npm ci`, unit tests, typecheck, lint, exact production build, Rust format/clippy/tests, locked release build, full Playwright, and dedicated accessibility suites passed.
- Live normal, boundary, invalid-input, and recovery paths passed. A loopback source was rejected; a public watch could be saved, edited, scanned with a useful redirect error, and removed.
- Demo and privacy request logs contained no third-party requests; fresh demo and legal pages made no API request. Security and caching headers passed.
- Live Axe found zero violations on the audited desktop/mobile routes. Keyboard skip navigation, focus visibility, reduced motion, 390 px layout, and 200% equivalent reflow passed.
- Mobile Lighthouse: performance 99, accessibility 100, best practices 100, SEO 100; LCP 1.3 s, TBT 130 ms, CLS 0, transfer 88 KiB.
- Live rate limiting observed a 40-request burst: 40 authorization responses and 60 `429` responses from 100 concurrent requests; every 429 had `Retry-After: 1`. Health stayed available.
- The packaged CLI installed into a clean temporary consumer and completed `demo`, `scan`, and `ack`, updating both Markdown and JSON state.
- Container-equivalent port-only startup, graceful SIGTERM, and token persistence across restart passed after reproducing the Dockerfile-created `/data` path.

## Defects

No critical, high, medium, or low product defects were found.

## Environment limitation

This worker has no Docker, Podman, or Buildah binary, so it could not execute the OCI image itself. The Dockerfile contract test, locked native release build, filesystem precondition inspection, and port-only runtime reproduction passed.

## Re-run

```sh
npm ci
npm test
npm run typecheck
npm run lint
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
npm run build
cargo build --release --locked
npm run test:browser
npm run test:a11y
```

Run each literal test in `.factory/claims.json` as written. Full evidence and exact observations are in `.factory/verification-18.md` and `.factory/qa-artifacts/verification-18-*`.
