# Independent verification 4 — FAIL

Verified independently on 2026-08-28 UTC.

- Candidate commit: `fabea5e036adfc5bf820e719083766e80902e2ce`
- Live URL: `https://integration-changelog-watch.sociobot.in`
- Work order: `integration-changelog-watch-verify-4`
- Result: **FAIL — do not release**

The live `GET /health` returned `200 {"build":"fabea5e036adfc5bf820e719083766e80902e2ce","ok":true}`. Deployed asset hashes match the clean local production build (`index-C8qgZKOV.js`, `index-Bc4ZUE6J.css`). No product code was changed during this verification.

## Mandatory gates

### First read — PASS

A cold `/` visit plainly says what (“Turn vendor changes into owned actions”), for whom (“engineers who maintain payment, auth, analytics, or messaging integrations”), and what to click first (“Try it with sample data”, followed by “See matched notices, owners, and checks”). At 390px, the action was inside the first viewport (y=373.94px, 241.67 × 46px), opens `/demo` in one click, and shows two realistic cards, owners, checks, three watches, and a persistent sample-data banner with Reset demo and Start for real.

### Claims registry and exact commands — PASS

`.factory/claims.json` exists with seven claims. From a clean checkout after `npm ci`, each exact command passed:

| Claim | Result |
| --- | --- |
| `sample-action-cards` | PASS, 2 Playwright projects |
| `csv-export` | PASS, 2 Playwright projects |
| `demo-local` | PASS, 2 Playwright projects |
| `workspace-boundary` | PASS, 2 Playwright projects |
| `requested-scans` | PASS, 2 Playwright projects |
| `cli-repository-workflow` | PASS, 2 Playwright projects |
| `database-persistence` | PASS, 1 Rust test |

In a fresh `/demo` browser context, every request was same-origin: `/demo`, the hashed JS/CSS assets, and `/paper-cut-hero.webp`. No analytics, advertising, third-party-font, or external request occurred.

### Clean local gates — PASS

| Check | Result |
| --- | --- |
| `npm ci` | PASS; 60 packages, 0 vulnerabilities reported |
| `npm test` | PASS, 3/3 |
| `npm run typecheck` / `npm run lint` | PASS |
| `npm run build` | PASS; `dist/` generated |
| `cargo fmt --all -- --check` | PASS |
| `cargo test --locked` | PASS, 8/8 |
| `npm run test:container` (`cargo build --release --locked`) | PASS |
| `npm run test:a11y` | PASS, 8/8 |
| `npm run test:browser` | PASS, 32/32 (desktop and mobile) |
| `cargo clippy --all-targets --locked -- -D warnings` | PASS |
| `cargo package --allow-dirty` | PASS |

The packed crate was extracted into a new temporary consumer and installed with a separate empty `CARGO_HOME`. Its installed binary ran `demo`, `scan --config examples/watches.json`, and `ack --config … --id 464f8e41f622`. The scan created a Markdown action card and state file; acknowledgement persisted `acknowledged: true`.

Docker, Podman, and Buildah were unavailable, so a Docker image could not be built/run. The exact locked release-binary build passed.

## Live functional evidence

A fresh live workspace ran: create workspace (201), add permitted public RSS watch (201), scan (200, `new_actions: 1`), list the owned action, acknowledge (200), rescan (200, `new_actions: 0`), and delete the temporary watch (204). `http://127.0.0.1/private` correctly returned 400 with the private/loopback/link-local explanation. This covers token isolation, explicit scan, acknowledgement, deduplication, deletion, and invalid-input recovery.

The demo's first Tab focuses the visible dashed Skip to content link. Acknowledgement changes card state, Reset restores the sample, and reduced-motion transition duration is `0s`. At 390px, `scrollWidth === clientWidth === 390`; normal home/demo loads had no page errors. `/privacy` has an independent title/h1 and an unknown route returns styled 404 with 404 status.

Live axe on the desktop demo found zero serious/critical violations. Headers include restrictive CSP with `frame-ancestors 'none'`, HSTS, `X-Content-Type-Options: nosniff`, strict-origin referrer policy, and restrictive Permissions-Policy. Hashed JS is immutable for a year; the 58,974-byte WebP is cached for a week. Local production output is 13.69kB JS / 5.32kB gzip and 7.69kB CSS / 2.52kB gzip, below the 200kB JS and 50kB CSS budgets.

The product is not a PWA and makes no offline-reload claim; no service worker is shipped. It has no sign-in or paid unlock flow, so Entra and billing checks are not applicable.

## Release-blocking findings

### Major — rate limiter violates the mandatory ingress-client contract

The backend contract requires rate limiting by the first `X-Forwarded-For` hop behind factory ingress. `src/main.rs` instead says it deliberately ignores `X-Forwarded-For` and keys buckets by TCP peer. That does not identify the actual client behind ingress.

Live evidence also exceeded the intended 40-request burst: 70 concurrent unauthenticated `GET /api/watches` calls all returned 401; a subsequent serial burst of 100 got 94 × 401 before 6 × 429. The 429 did have `Retry-After: 1`, but the observed client allowance was 94 rapid requests rather than the configured burst of 40. This is release-blocking.

**Required repair:** derive the trusted, ingress-sanitized first `X-Forwarded-For` value; test one client and spoof attempts; document/enforce the allowance (e.g. 20 req/s, burst 40); preserve calculated `Retry-After`.

### Major — standard RSS CDATA titles remain literal markup

The brief's core job is watching public changelog/RSS URLs and turning matched notices into owned action cards. A normal live public RSS scan stored the standard RSS title as:

```text
<![CDATA[Unix V4 Workshop at Low Resource Computing]]>
```

The excerpt retained its closing `]]>`. As the UI escapes it, users see literal markup rather than readable notice text. Matching, ownership, local check, acknowledgement, and deduplication worked, but common RSS input produces malformed cards. This is core-format interoperability failure.

**Required repair:** decode CDATA/XML text when parsing RSS titles/descriptions, with an end-to-end fixture that asserts the rendered card text.

## Notes

- No `verify-url.sh` is present, so its requested check was unavailable; equivalent title/lang/main/alt/console checks were performed via Playwright and the a11y suite.
- Normal demo/home loads had no console/page errors. The deliberate 404 route emits the expected browser failed-resource console entry for its 404 document only.
- The CLI example's acknowledgement ID is hash-derived: `ack --id 1` correctly fails, while the action/state ID succeeds. Documentation should state where to obtain it.

## Release decision

**FAIL.** Do not release until both major findings are repaired and independently reverified live.
