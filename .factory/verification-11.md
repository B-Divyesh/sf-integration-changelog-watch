# Independent verification 11 — FAIL

Verified independently on 2026-08-29 UTC.

- Candidate: `031d39102a19c673f6517a356df3b683c9386f60`
- Live URL: <https://integration-changelog-watch.sociobot.in>
- Work order: `integration-changelog-watch-verify-11`
- Result: **FAIL — the candidate is not release-ready.**

The live deployment is healthy and matches the candidate, and the product works end to end. Release is nevertheless blocked because one literal command in the required claims manifest fails. The independent Rust formatting check also fails.

## Mandatory first gates

### Claims — FAIL

`.factory/claims.json` exists and contains 20 complete, uniquely identified claims. After a clean `npm ci` (60 packages, zero vulnerabilities), every literal `test` command was run separately from the candidate checkout before the broader QA.

| Claim | Result |
| --- | --- |
| `sample-action-cards` | PASS |
| `csv-export` | PASS |
| `demo-local` | PASS |
| `demo-isolation-transitions` | PASS |
| `workspace-boundary` | PASS |
| `hosted-scan-result` | PASS |
| `hosted-watch-limit` | PASS |
| `keyword-edit` | PASS |
| `requested-scans` | PASS |
| `redirecting-feeds` | PASS |
| `watch-file-portability` | PASS |
| `cli-more-feeds` | PASS |
| `cli-repository-workflow` | PASS |
| `cli-demo-local` | PASS |
| `cli-shipped-mapping-local` | PASS |
| `api-contract` | PASS |
| `container-build-stage` | **FAIL** |
| `database-persistence` | PASS |
| `port-only-startup` | PASS |
| `single-replica-durable-data` | PASS |

The failing literal command is:

```text
npm test -- --grep @claim:container-build-stage
```

Vitest 3.2.7 exits 1 before collecting the tagged test:

```text
CACError: Unknown option `--grep`
```

The underlying Dockerfile assertions pass as part of ordinary `npm test`, but the acceptance contract explicitly makes any failing manifest command release-blocking. Summary: **19 passed, 1 failed**.

### Cold first read — PASS

At 1440×900 and 390×844, the first screen says:

- What: “Turn vendor changes into assigned action cards.”
- Who: “For engineers who maintain payment, auth, analytics, or messaging integrations.”
- First click: **Try it with sample data**, beside “See matched notices, owners, versions, and checks.”

The one-click action opens `/demo`. Its first 390×844 viewport already shows a realistic Stripe notice, matched keyword, owner, dependency version, and check. The persistent banner says “Demo — sample data, nothing is saved” and supplies **Reset demo** and **Start a private workspace**.

## Candidate and deployment identity

- `git rev-parse HEAD`: `031d39102a19c673f6517a356df3b683c9386f60`.
- `GET /health`: `200`, `{"build":"031d39102a19c673f6517a356df3b683c9386f60","ok":true}`.
- Live HTML `data-build`: the same full SHA.
- Local and live SHA-256 hashes match byte for byte for the hashed JS and CSS, hero WebP, social card, favicon, touch icon, `robots.txt`, `sitemap.xml`, and 404 CSS.

This fresh evidence resolves the previously reported deployment-only uncertainty: the requested candidate is deployed.

## Local quality gates

| Check | Result |
| --- | --- |
| `npm ci` | PASS; 60 packages, 0 vulnerabilities |
| `npm test` | PASS; 5/5 |
| `npm run typecheck` | PASS |
| `npm run lint` | PASS |
| `npm run build` | PASS; created `dist/` |
| `cargo test --locked` | PASS; 18/18 |
| `cargo clippy --locked --all-targets -- -D warnings` | PASS |
| `cargo build --release --locked` | PASS |
| `cargo fmt --all -- --check` | **FAIL**; formatting diffs in `src/main.rs` |
| `npm run test:browser` | PASS; 59 passed, 1 intentional project skip |
| Live `npm run test:browser` | PASS; 59 passed, 1 intentional project skip |
| `npm run test:a11y` | PASS locally and live; 16/16 each |

The exact Docker image build was attempted with all three source-SHA build args, but this verifier image has no `docker` executable. The exact locked Rust release build used by the Dockerfile passed, and ordinary `npm test` passed the Dockerfile contract assertions. This environment limitation does not excuse the separate failing literal claims command.

## Product, boundary, and recovery exercise

- Demo: opened sample data, acknowledged the pending action by keyboard, reset it, exported two action rows plus the CSV header, and left demo mode. Demo state was discarded before the private workspace began.
- Normal hosted workflow: a fresh workspace saved a public `https://1.1.1.1/feed` watch with owner, dependency version, keywords, and command, then removed it with the documented recovery message.
- Invalid then recover: `not-a-url` was rejected with “Enter a complete public http or https URL.” A valid watch saved immediately afterward. The expected HTTP 400 produces Chromium's standard failed-resource console entry, but no application exception; cold and normal loads have no console/page errors.
- Boundaries: a 120-character vendor name returned 201; 121 characters returned 400 with the exact 120-character limit; a blank vendor returned 400 with the required-fields instruction. Three hosted watches succeed and the fourth returns the documented 409 recovery message.
- Feed safety: private/loopback input and redirecting feeds are rejected. Controlled RSS coverage creates the expected action card; scan failures remain visible with a next step.
- Concurrency and persistence: 24 parallel authenticated reads returned 200. Rust coverage proves the three-watch limit remains atomic across ten concurrent creates, reconnects to a durable SQLite file, and restores a browser token after the simulated process restart.
- Health/build identity and route authorization match the README API table.

## Backend rate limit

A fresh single-client identity sent 48 simultaneous unauthenticated `GET /api/watches` requests. Results were 40 × 401 and 8 × 429. Every 429 carried `Retry-After: 1` and the readable one-second retry message.

Observed allowance: **40-request burst, then a 20 requests/second refill**. The API router applies the same limiter middleware to all documented backend routes, including health. The implementation keys the first `X-Forwarded-For` hop.

## CLI consumer exercise

`cargo package --locked --allow-dirty --no-verify` succeeded. The resulting 11,598,069-byte crate installed from its extracted package into a clean temporary Cargo root. The installed binary successfully ran `--help`, `demo`, scanned the shipped mapping into a Markdown action card, and acknowledged the generated ID in both state and Markdown. Missing scan configuration exited 1 with a readable error; missing acknowledgement ID exited 2 with usage guidance.

## Privacy, accessibility, responsive behavior, and headers

- A fresh live `/demo` acknowledge flow made exactly three requests: document, same-origin hashed JS, and same-origin hashed CSS. It made no API or third-party request.
- Demo storage contained only `demo:integration-changelog-watch`; neither the real workspace record nor token existed.
- Root, demo, privacy, terms, and 404 routes have route-specific titles, one `h1`, a `main`, and no application errors on load. The 404 response itself produces the browser's expected network 404 console message.
- Axe found zero violations on the demo in both desktop and mobile projects. Keyboard starts on the visible skip link, Enter moves focus to `main`, Space acknowledges an action, and focus is restored to the updated card.
- At 390 px there is no horizontal overflow. The 200%-equivalent reflow test passes. Navigation/footer touch targets pass 44 px checks. Reduced motion computes to zero transition duration and no animations.
- Internal and sample external links resolve; the styled missing route correctly returns 404. `robots.txt` and `sitemap.xml` list the four public routes.
- Responses send HSTS, `nosniff`, strict-origin referrer policy, restrictive permissions policy, and a header-delivered self-only CSP with `frame-ancestors 'none'`.
- HTML is `no-cache`; API and health are `no-store, private`; hashed JS/CSS are `public, max-age=31536000, immutable`; the hero is cached for seven days.

The product has no sign-in, runtime AI, payment/unlock flow, or service worker. Entra, AI gateway, billing, and PWA update checks are not applicable. The brief explicitly makes LLM summaries a non-goal.

## Performance

Mobile Lighthouse against the live candidate:

| Category/metric | Result |
| --- | ---: |
| Performance | 98 |
| Accessibility | 100 |
| Best practices | 100 |
| SEO | 100 |
| FCP | 1.0 s |
| LCP | 1.3 s |
| Total blocking time | 160 ms |
| CLS | 0 |
| Initial transfer | 88 KiB |

Production bundles are within budget: JS 19,463 bytes raw / 6,820 bytes gzip; CSS 8,801 / 2,745; hero WebP 58,974 bytes. There are no remote fonts or scripts.

## Defects by severity

### Release-blocking / high

1. **A required claim command cannot run.** `container-build-stage` uses unsupported Vitest `--grep`, exits 1, and therefore fails the mandatory claims gate even though the underlying test passes in the unfiltered suite.

### Medium

1. **The Rust source fails its formatting check.** `cargo fmt --all -- --check` reports diffs around `src/main.rs` lines 721, 940, and 1426.

### Low

1. **The Cargo package is unnecessarily large.** The 11.6 MB crate includes internal `.factory` reports/screenshots and the 2.5 MB source PNG. The installed CLI works, but package include/exclude rules should omit non-runtime QA artifacts.

## Release decision

**FAIL.** The deployed product is functional, identity-matched, accessible, private in demo mode, fast, and correctly rate limited. It cannot be accepted while any literal `.factory/claims.json` command fails; the Rust formatting failure is an additional quality-gate defect.
