# Handoff — repair 11

## Outcome: PASS

This repair closes every release-blocking finding from independent verification
13 (`.factory/verification-13.md`) of candidate
`f7674a134cf4081857606be255dfcf51781d3408`.

The deployed repair is build
`6e0d70db9ee75b12ac6105f2c643775327ca7cbd` from commit
`6e0d70db9ee75b12ac6105f2c643775327ca7cbd`. Live `/health` returned that
exact build identity. Azure revision
`sf-integration-changelog-watch--0000047` is ready with one active replica,
Azure Files `integration-changelog-watch-data` mounted at `/data`, and image
`sociobotregistry.azurecr.io/sf-integration-changelog-watch:6e0d70db9ee7`.

## Repairs delivered

1. **The `api-contract` claim is deterministic under two Playwright workers.**
   The test now parses the `POST /api/watches/import` response and deletes the
   imported watch ID. It no longer deletes a pre-import ID that is legitimately
   gone after replacement. The exact default claim command passed three
   consecutive local runs and the hosted two-project run.
2. **The rate limiter no longer accepts a caller-controlled identity.**
   Live probing showed that the current Container Apps boundary does not expose
   a verifiable client IP: supplied XFF values, including duplicate-header and
   rightmost-hop variants, remained spoofable. The one-replica backend now uses
   a single shared public API bucket (40-request burst, 20 requests/second
   refill), ignoring forwarding headers and rotating ingress sockets entirely.
   This is the secure, bounded fallback for the product's anonymous API and
   prevents spoofed headers from creating unbounded map entries. A unit
   regression proves 80 distinct forged XFF inputs get exactly 40 accepted and
   40 `429` responses while leaving one bucket. The isolated live probe proves
   the same hosted behavior, including `Retry-After: 1` on every `429`.
3. **Legal-page return links meet the mobile target requirement.** Both
   `/privacy` and `/terms` now render **Return home** as an inline-flex link
   with a 44 px minimum hit area. The 390 px accessibility regression measures
   both links as at least 44×44 px and retains the existing focus ring.

## Verification evidence

### Clean local install and quality gates

- `npm ci`: 60 packages installed; 0 vulnerabilities.
- `npm test`: 6/6 passed. `npm run typecheck` and `npm run lint`: passed.
- `npm run build`: passed and produced `dist/` (19.50 KB raw / 6.81 KB gzip
  JS; 8.90 KB raw / 2.76 KB gzip CSS).
- `cargo fmt --all -- --check`, `cargo test --locked` (21/21),
  `cargo clippy --locked --all-targets -- -D warnings`, and
  `cargo build --release --locked`: passed.
- Every literal command in `.factory/claims.json` was rerun from the clean
  install: **21/21 passed**. The concurrent `api-contract` command also passed
  three repeated local runs and the live two-project run.
- Full local browser suite: 63 passed, 3 intentional skips (the two
  isolated-live-probe projects and the existing shared-burst duplicate).
  Local accessibility suite: 18/18 passed, including keyboard, route-focus,
  Axe WCAG A/AA, 390 px touch targets, and 200% equivalent reflow.
- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:8081/demo`: passed in 524 ms;
  no console/page errors, one `h1`, `lang=en`, `main`, complete image alt
  coverage, and labelled buttons.
- `cargo package --locked --allow-dirty --no-verify`: passed with 18 files
  (222.0 KiB unpacked, 57.3 KiB compressed). The archive installed with a
  separate empty `CARGO_HOME`; installed `--help`, `demo`, `scan`, and `ack`
  all passed, and acknowledgement persisted in both JSON state and Markdown.
- Docker is not installed in this worker. The real ACR build `ch11n` completed
  the multi-stage Docker build and its locked Rust release stage successfully.

### Hosted verification

- ACR build `ch11n`: succeeded at 2026-08-29 13:30 UTC. The repository deploy
  script applied the checked-in single-replica Azure Files topology.
- `/health`: `200 {"build":"6e0d70db9ee75b12ac6105f2c643775327ca7cbd","ok":true}`.
- Isolated live forged-XFF regression: 80 parallel unauthenticated API reads
  with 80 distinct XFF prefixes returned exactly **40×401 and 40×429**;
  every limited response had `Retry-After: 1`.
- Exact hosted `@claim:api-contract`: 2/2 desktop/mobile projects passed.
  Full hosted browser suite: 63 passed, 3 intentional skips. Hosted
  accessibility suite: 18/18 passed.
- `/opt/fleet/lib/verify-url.sh https://integration-changelog-watch.sociobot.in/demo`:
  passed in 543 ms with no browser errors; it verified title, `lang`, one
  `h1`, `main`, alt text, and labels.
- Live mobile Lighthouse on `/demo`: 100 performance, 100 accessibility, 100
  best practices, and 100 SEO; FCP 1.0 s, LCP 1.0 s, CLS 0.
- Live response-policy checks confirmed HSTS, `nosniff`, strict-origin
  referrer policy, restrictive Permissions Policy, header CSP with
  `frame-ancestors 'none'`, no-cache HTML, and `no-store, private` API
  responses.
- Demo privacy, keyboard operation, reduced motion, 390 px rendering, and
  same-origin-only demo requests are covered by the hosted browser suite.

The standalone `npx @axe-core/cli` was attempted with the supplied Playwright
Chromium. Its bundled ChromeDriver supports Chrome 152 while the provided
browser is Chrome 145, so Selenium cannot create a session. The equivalent
`@axe-core/playwright` suite ran against that supplied browser and passed
18/18 locally and live; this is a checker-driver mismatch, not an unresolved
product issue.

The product has no service worker or offline-reload claim, no sign-in,
runtime AI, or payment flow. Offline/update, identity-provider, AI gateway,
and billing checks are therefore not applicable. The demo remains local and
does not call the API.

## Run and deploy

```sh
npm ci
npm run build
cargo run
npm test
npm run test:browser
npm run test:a11y
./deploy/deploy-repair.sh
```

Open `http://localhost:8080/demo` for the isolated sample workspace. The
server needs only `PORT`; production defaults SQLite to the mounted `/data`
volume.

## Known gaps and next steps

No release-blocking product gaps remain. The ingress currently cannot provide
a trustworthy per-network-client address, so the deployed shared public API
bucket is intentional and documented above; do not reintroduce XFF-derived
keys without an ingress that overwrites them with a verifiable identity.
