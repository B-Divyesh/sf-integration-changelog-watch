# Handoff — independent verification 12

## Outcome: FAIL

Candidate `1d82c9140dcf6937295d57fc96d47c087aa0775a` was verified at
<https://integration-changelog-watch.sociobot.in> on 2026-08-29 UTC. The exact
candidate is live, but it is not release-ready because a rejected watch-file
import can irreversibly delete every existing watch in the private workspace.

No product code was changed. Full evidence and reproduction steps are in
`.factory/verification-12.md`; screenshots, the factory URL-verifier output,
and Lighthouse JSON are in `.factory/qa-artifacts/verification-12/`.

## Release-blocking defect

1. In a fresh private workspace, save a valid watch.
2. Import a schema-valid watch file containing
   `http://127.0.0.1/private`.
3. Confirm **Import 1 watch**.
4. The app sends `DELETE` for the existing watch (`204`) before the rejected
   imported `POST` (`400`). It says the import failed, but the server list is
   empty. Stale old/import-preview rows remain until reload; reload shows no
   watches.

Required repair: validate the whole replacement server-side before mutation,
or replace it atomically in one transaction. Failed import must preserve all
existing watches. Add a real-workspace regression/claim test covering a
server-rejected source and rollback.

## Other defects

- **Medium:** `/demo` has an `h1` → `h3` heading skip. Lighthouse flags
  `heading-order` and scores accessibility 98.
- **Medium:** navigating to Demo focuses its `h1`, but browser Back to `/`
  leaves focus on `<body>` after asynchronous workspace hydration replaces the
  focused heading.

## Verification summary

- All 20 exact `.factory/claims.json` commands: PASS.
- Cold first read and one-click sample demo: PASS at desktop and 390 px.
- `npm ci`: PASS; 60 packages, zero vulnerabilities.
- `npm test`: 6/6; typecheck, lint, production frontend build: PASS.
- `cargo fmt --check`, 18 Rust tests, Clippy with warnings denied, and locked
  release build: PASS.
- Full browser suite locally and live: 59 passed / 1 intentional project skip
  each. Accessibility suite locally and live: 16/16 each; Axe found no
  violations.
- Real Stripe Node feed: save, scan (7 actions), acknowledge, deduplicate,
  edit, and delete: PASS.
- Boundary checks: 120/121-character names, blank/invalid/private input,
  three-watch limit, workspace isolation, and 24 parallel reads behaved as
  documented.
- Rate limit: an 80-request burst returned 40 × `401`, then 40 × `429`; every
  `429` had `Retry-After: 1`. Observed allowance is a 40-request burst with a
  20 requests/second refill.
- Azure reports the candidate image running at 100% traffic with one replica
  and the durable Azure Files `/data` mount.
- Clean crate package/install: PASS; installed CLI ran help/demo, scanned the
  packaged feed without duplication, and persisted acknowledgement.
- Demo privacy: three same-origin static requests, no API or third-party
  requests, and no console/page errors.
- Mobile Lighthouse `/demo`: performance 100, accessibility 98, best practices
  100, SEO 100; LCP 1.0 s, TBT 20 ms, CLS 0, transfer 30 KiB.
- Bundles: JS 19,463 bytes raw / 6,820 gzip; CSS 8,801 / 2,745; hero 58,974
  bytes.

Docker/Podman/Buildah are unavailable in this worker. The exact constituent
production builds and Dockerfile contract tests pass, and the candidate-tagged
container is the healthy live revision. The product has no PWA, sign-in,
runtime AI, billing, or paid-unlock path.
