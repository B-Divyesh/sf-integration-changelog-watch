# Independent verification 15 — PASS

**Candidate:** `34d2c65449dfc26b6d8ae606044bf072fa9b626f`
**Live URL:** https://integration-changelog-watch.sociobot.in
**Date:** 2026-08-29 UTC
**Verdict:** **PASS** — no Critical, High, Medium, or Low product defects found.

## First read and live identity

A cold desktop Chromium visit returned `200`, the title **“Integration Changelog Watch — Assign vendor changes”**, one H1, and no console or page errors. The first screen plainly says it turns vendor changes into assigned action cards, names engineers maintaining payment/auth/analytics/messaging integrations as the audience, and presents **Try it with sample data** with the result stated next to it. The required one-click demo is therefore present and understandable.

`GET /health` on the live URL returned `200` with:

```json
{"build":"34d2c65449dfc26b6d8ae606044bf072fa9b626f","ok":true}
```

The live JavaScript, CSS, and hero WebP SHA-256 values exactly matched the local production `dist/` and source asset. This establishes that the inspected deployment is the candidate.

## Required claims gate

From this clean checkout, after `npm ci` (60 packages; 0 vulnerabilities), `npm run test:claims` ran every literal command in `.factory/claims.json` through the shipped demo/server entry point. Result: **all 21 passed**, with port 8080 bindable before and after every command.

| Claim IDs | Result |
| --- | --- |
| `sample-action-cards`, `csv-export`, `demo-local`, `demo-isolation-transitions`, `workspace-boundary` | PASS |
| `hosted-scan-result`, `hosted-watch-limit`, `keyword-edit`, `requested-scans`, `redirecting-feeds` | PASS |
| `watch-file-portability`, `watch-file-rejection-preserves-watches`, `cli-more-feeds`, `cli-repository-workflow`, `cli-demo-local`, `cli-shipped-mapping-local` | PASS |
| `api-contract`, `container-build-stage`, `database-persistence`, `port-only-startup`, `single-replica-durable-data` | PASS |

The recorded harness ended `All 21 literal claim commands passed without leaking port 8080.` and `__CLAIMS_EXIT=0`.

## Local build and automated quality gates

The following complete sequence exited `0`:

```sh
npm test
npm run typecheck
npm run lint
npm run build
cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked
npm run test:browser
```

Evidence: Vitest **7/7**; Rust **23/23**; full Playwright **63 passed, 3 skipped** (the three skips are deliberately isolated live-rate probes); locked optimized Rust build passed. Vite produced `dist/`: initial JavaScript is 19,478 bytes raw / **6.80 KiB gzip**, CSS 8,903 bytes raw / **2.76 KiB gzip**, well below the static budget. The hero WebP is 58,974 bytes.

`cargo package --locked --allow-dirty --no-verify` packed 19 files (229.4 KiB unpacked, 59.0 KiB compressed). The crate was unpacked into a fresh consumer directory, installed with `cargo install --path ... --locked`, and its installed `--help` and `demo` commands succeeded and printed the shipped Stripe/Auth0 action cards.

Docker, Podman, and Buildah are absent from this verifier image, so an image build could not be run here. The actual web build, locked release build, and the versioned Docker build-stage claim test all passed; this is an environment limitation, not a product defect.

## End-to-end and robustness

On the live site, a fresh private workspace rejected `http://127.0.0.1/internal` with the readable recovery message about blocked private/loopback/link-local addresses and left zero watches. A valid public RSS Board feed with keyword `spacewalk`, owner, dependency version, and command saved successfully; an explicit scan created **2** action cards. A schema-valid import containing a loopback feed was rejected while the existing valid watch remained. This exercises normal flow, a boundary, invalid input, and recovery.

The local concurrency/persistence coverage passed (atomic three-watch limit, parallel authenticated reads, restart persistence, per-client limiter behavior, and durable single-replica topology). On the deployed API, 80 parallel unauthenticated calls with rotating caller-supplied `X-Forwarded-For` prefixes produced exactly **40 × 401** followed by **40 × 429**. Every 429 carried `Retry-After: 1`; `/health` remained `200`. Observed allowance: burst of 40 API requests per client, refilling as implemented by the service.

## Privacy, accessibility, responsive, and delivery checks

- Fresh `/demo` at 390 px made **no `/api/` calls**; all demo requests were same-origin. Acknowledgement wrote only `demo:integration-changelog-watch`; no real workspace/token existed. Reset restored the sample, and starting a private workspace discarded demo storage before creating the real token.
- Fresh landing-page load had no console/page errors. The two console `Failed to load resource: 400` entries observed during the deliberate invalid-address/import probes correspond to the expected, user-visible validation rejections; no exception or unexpected error was emitted.
- Live Axe WCAG 2 A/AA scan of `/demo` had **zero violations** (therefore zero serious/critical findings). Desktop and 390 px screenshots were visually inspected; 390 px had `scrollWidth === clientWidth`. The full browser suite also covers keyboard skip link/Space acknowledgement, visible focus, route focus restoration, 195 px reflow, touch targets, and reduced motion.
- Live response headers include CSP with `frame-ancestors 'none'`, `X-Content-Type-Options: nosniff`, strict referrer policy, HSTS, and permissions policy. HTML is `no-cache`; hashed JS/CSS are `public, max-age=31536000, immutable`; hero WebP is `max-age=604800`. `/privacy`, `/terms`, `robots.txt`, sitemap, and styled 404 all returned as expected.
- No third-party requests, scripts, analytics, advertising, or remote fonts appeared in the demo request log.

## Defects by severity

| Severity | Findings |
| --- | --- |
| Critical | None |
| High | None |
| Medium | None |
| Low | None |

## Reproduce

Run `npm ci && npm run test:claims`, then the full command sequence above. Use `cargo package --locked --allow-dirty --no-verify` followed by an install from the extracted `.crate` for the CLI consumer check. Live checks target the URL and candidate SHA stated at the top of this report.
