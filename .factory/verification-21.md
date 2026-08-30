# Independent verification 21 — PASS

Verified independently on 2026-08-30 UTC.

- Candidate: `2e443911e1e63e4d998310c84df07d1bda558630`
- Live URL: <https://integration-changelog-watch.sociobot.in>
- Work order: `integration-changelog-watch-verify-21`
- Result: **PASS — the candidate is release-ready.**

Fresh evidence confirms that the candidate is deployed, all declared claims
pass, and both the hosted and repository-owned workflows complete end to end.
No product code was changed during verification.

## Mandatory first gates

### Claims manifest — PASS

`.factory/claims.json` exists with 28 entries. After `npm ci` from the clean
candidate checkout, every literal `test` command was run separately. All 28
exited 0:

`sample-action-cards`, `csv-export`, `demo-local`,
`demo-isolation-transitions`, `workspace-boundary`, `hosted-scan-result`,
`hosted-watch-limit`, `keyword-edit`, `requested-scans`,
`online-feed-scans`, `no-account-or-payment`, `redirecting-feeds`,
`watch-file-portability`, `watch-file-rejection-preserves-watches`,
`cli-more-feeds`, `cli-repository-workflow`, `cli-demo-local`,
`cli-shipped-mapping-local`, `api-contract`, `container-build-stage`,
`database-persistence`, `port-only-startup`, `azure-files-dotfile-locking`,
`single-replica-durable-data`, `scheduled-scan-consent`,
`scheduled-scan-deduplication`, `scheduled-run-status`, and
`scheduled-notification-destination`.

Landing, legal, README, demo, deployment, and CLI statements were
cross-checked against the manifest. No unlisted user-facing capability claim
was found.

### Cold first read — PASS

A fresh 1440 × 900 Chromium context with empty storage opened the live home
page without interaction. The first screen says:

- What: **“Turn vendor changes into assigned action cards.”**
- Who: **“For engineers who maintain payment, authentication, analytics, or
  messaging integrations.”**
- First action: **“Try it with sample data,”** beside **“See matched notices,
  owners, versions, and checks.”**

The first action opens the sample in one click. At 390 × 844 the first demo
viewport already shows a matched Stripe notice, owner `Maya · Payments`,
dependency `stripe-node 16.2`, and check `pnpm test:stripe`. Its persistent
banner provides **Reset demo** and **Start a private workspace**.

## Candidate and deployment identity

- `git rev-parse HEAD`, live `/health`, the SPA footer, and the 404 footer all
  report `2e443911e1e63e4d998310c84df07d1bda558630`.
- The locally rendered `index.html` and `404.html` are byte-identical to live.
- SHA-256 hashes match for live/local JS, CSS, hero, social card, favicon,
  touch icon, `robots.txt`, `sitemap.xml`, and 404 stylesheet.
- `/`, `/demo`, `/privacy`, and `/terms` return 200. A missing route returns
  the styled 404. Every discovered internal link resolves; both sample vendor
  links return 200.

This is fresh evidence and does not rely on an earlier deployment report.

## Local quality gates

| Check | Result |
| --- | --- |
| `npm ci` | PASS — 60 packages; zero audit vulnerabilities |
| All 28 literal claim commands | PASS |
| `npm test` | PASS — 10/10 |
| `npm run typecheck`; `npm run lint` | PASS |
| `npm run build` | PASS — produced `dist/` |
| `cargo test --locked` | PASS — 28/28 |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --locked --all-targets -- -D warnings` | PASS |
| `cargo build --release --locked` | PASS |
| Local `npm run test:browser` | PASS — 69 passed, 3 intentional skips |
| Live `npm run test:browser` | PASS — 69 passed, 3 intentional skips |
| Live `npm run test:a11y` | PASS — 20/20 |
| Live demo suite | PASS — 16/16 |
| Factory `verify-url.sh`, home and demo | PASS — no errors; semantic checks pass |

The three browser skips are deliberate guards for the separately invoked live
rate probe and one duplicate shared-backend burst. The verifier image has no
Docker, Podman, Buildah, or Nerdctl executable, so OCI assembly could not be
repeated. The versioned Dockerfile test, exact Vite build, locked Rust release
build, container-equivalent runtime boot, and exact live image identity all
pass.

## End-to-end product exercise

The live UI completed this flow:

1. Open the one-click sample and inspect two realistic action cards.
2. Discard it and create an isolated private workspace without an account.
3. Add the candidate's public RSS fixture through the UI.
4. Scan it and create `Webhook delivery format changes` with the configured
   owner, dependency, command, keyword, excerpt, and vendor link.
5. Acknowledge the action, turn on a 15-minute schedule, stop the schedule,
   reload, and recover the same workspace and watch.

The live API returned readable recovery responses for an empty vendor,
`ftp:` URL, loopback feed, 121-character vendor, 14-minute and 10,081-minute
schedules, private webhook, unknown action, and missing authorization. The
120-character vendor boundary succeeded. All tested API responses carried
`Cache-Control: no-store, private`.

Ten simultaneous watch creates in a fresh workspace produced exactly three
`201` responses and seven `409` responses, leaving three stored watches.

## Backend persistence, identity, and rate limiting

- A release server started with the image's declared `/data` precondition and
  no runtime app configuration beyond `PORT`; `/health` returned the candidate
  SHA and the process drained cleanly on shutdown.
- A separate SQLite-backed release server created a token and watch, stopped,
  restarted against the same file, and returned the same authorized watch.
- The isolated live rate-limit Playwright probe passed. After refill, a fresh
  100-request same-client burst produced exactly 40 authorization responses
  and 60 `429` responses. Every limited response had `Retry-After: 1` and
  `Cache-Control: no-store, private`; `/health` remained 200.
- Enforced allowance: 40-request burst with 20 requests/second refill. Health
  is the documented exemption.

## CLI package and clean consumer

`cargo package --locked --allow-dirty` produced a verified 20-file crate,
251.4 KiB unpacked / 62.9 KiB compressed. It installed into a fresh temporary
Cargo root. The installed binary printed its help and bundled demo, scanned
the shipped local feed into action `464f8e41f622`, and acknowledged that action
in both JSON state and the generated Markdown card.

## Privacy, headers, caching, and scope

- Cold home and demo loads use only the product origin. There are no analytics,
  advertising, third-party fonts/scripts, AI, billing, or other browser-side
  requests. The real browser workflow also contacted only the product origin.
- The cold landing made no API request and created no workspace. Demo changes
  use only the demo namespace; private workspaces use a 64-character
  browser-held token.
- HTML sends HSTS, `nosniff`, strict-origin referrer policy, restrictive
  permissions policy, and a response-header CSP with `frame-ancestors 'none'`.
- HTML and health use `no-cache`; API uses `no-store, private`; hashed JS/CSS
  use one-year immutable caching; the hero uses a seven-day public cache.
- The product has no sign-in, payment/unlock, runtime AI, service worker, PWA,
  or offline-reload claim. Entra, billing, AI gateway, and PWA update checks
  are therefore not applicable. The brief explicitly excludes LLM summaries.

## Accessibility, mobile, and performance

- Live Playwright + Axe found zero WCAG A/AA violations, hence zero
  serious/critical findings, across home, demo, Privacy, Terms, and 404.
- Keyboard traversal exposes a visible 3 px dashed focus ring. Skip-to-content,
  route focus, Space acknowledgement, and focus recovery all work without a
  trap. Sampled mobile controls are at least 44 px.
- The 390 px layout has no horizontal overflow. The 195 px/200%-equivalent
  reflow test passes. Reduced-motion emulation yields no transitions or
  animations.
- Semantics pass: route title, `lang=en`, one H1, main landmark, ordered
  headings, alt text, named controls, live regions, and route announcements.
- Production output: JS 23.28 kB raw / 7.84 kB gzip; CSS 9.03 kB raw / 2.79
  kB gzip; hero WebP 58,974 bytes; no web fonts.
- Fresh mobile Lighthouse: **100 performance, 100 accessibility, 100 best
  practices, 100 SEO**; FCP 1.0 s, LCP 1.3 s, TBT 80 ms, CLS 0, total 92 KiB.
- Direct interaction checks rendered the demo in 106 ms and acknowledgement
  in 40 ms.

Evidence is under `.factory/qa-artifacts/verification-21/`.

## Defects by severity

| Severity | Findings |
| --- | --- |
| Critical | None |
| High | None |
| Medium | None |
| Low | None |

## Release decision

**PASS.** Candidate `2e443911e1e63e4d998310c84df07d1bda558630` is live,
identity-matched, and satisfies the acceptance contract.
