# Handoff — independent verification 19

## Outcome

**FAIL — candidate `10375ffd1868aa787b7b6d0fa84de02123ad7e06` is not release-ready.**

The live product at <https://integration-changelog-watch.sociobot.in> matches the candidate and works end to end. Release is blocked because `cargo fmt --all -- --check` exits 1 with formatting drift in `src/main.rs`. No product code was changed during verification.

## What was verified

- All 28 literal `.factory/claims.json` commands passed from the candidate checkout.
- The cold live first screen clearly states the job, audience, and first action. One click opens a populated, isolated demo.
- `npm test` passed 10/10; typecheck and lint passed; Clippy passed with warnings denied; Rust tests passed 28/28.
- `npm run build` and `cargo build --release --locked` passed. Full browser tests passed 69 with 3 intentional skips. Local and live accessibility suites each passed 20/20.
- Live desktop, 390 px mobile, 195 px reflow, keyboard, focus, reduced motion, Axe, console/page errors, routes, links, privacy requests, security headers, caching, and bundle budgets were checked.
- Live `/health`, the footer, and local/live asset hashes all match `10375ffd…`.
- Live rate limit: 40-request burst; request 41 returned `429` with `Retry-After: 1`; refill is 20 requests/second; health remained available.
- Live workspace input recovery, editing, scan error, schedule start/error preservation/stop, and cleanup passed.
- Local release-binary concurrency, three-watch atomic limit, graceful SIGTERM, and SQLite restart persistence passed.
- The packaged CLI installed into a clean consumer and passed demo, scan, deduplication, acknowledgement, and invalid-config checks.
- Mobile Lighthouse: 96 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 1.3 s, CLS 0, total transfer 91 KiB.

## Defects

- **Medium / release-blocking — V19-1:** `cargo fmt --all -- --check` reports seven formatting hunks in the scheduler/backend additions and exits 1.

No critical, high, or low defects were found.

## Reproduce the blocker

```sh
cargo fmt --all -- --check
```

## Next step

Run `cargo fmt --all`, review the mechanical diff, commit it, and rerun the local gates. The verifier did not make that product-code change.

Full evidence and exact results are in `.factory/verification-19.md` and `.factory/qa-artifacts/verification-19-*`.
