# Independent verification 20 — PASS

Verified independently on 2026-08-29 UTC.

- Candidate: `2c6f43ab489e9c34cb25513407ef24ddbebaaf88`
- Live URL: <https://integration-changelog-watch.sociobot.in>
- Work order: `integration-changelog-watch-verify-20`
- Result: **PASS — the candidate is release-ready.**

The earlier formatting-only failure is repaired. Fresh evidence shows the exact
candidate is deployed, every declared claim passes, and the product completes
the real hosted and repository-owned changelog workflows.

## Mandatory first gates

### Claims manifest — PASS

`.factory/claims.json` exists with 28 entries. After a lockfile install (`npm
ci`: 60 packages, zero vulnerabilities), every literal `test` command was run
independently from the candidate checkout. All 28 exited 0:

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

The landing page, legal copy, README, demo documentation, and CLI help were
cross-checked against the manifest. No unlisted user-facing capability claim
was found.

### Cold first read — PASS

A new 1440 × 900 Chromium context with empty storage opened the live home page.
The first screen says:

- What: **“Turn vendor changes into assigned action cards.”**
- Who: **“For engineers who maintain payment, authentication, analytics, or
  messaging integrations.”**
- First action: **“Try it with sample data,”** beside **“See matched notices,
  owners, versions, and checks.”**

The button opens `/demo` in one click. At 390 × 844 the first demo viewport
already shows a Stripe notice, owner `Maya · Payments`, dependency
`stripe-node 16.2`, and check `pnpm test:stripe`. The demo banner provides
Reset demo and Start a private workspace; acknowledgement and reset both work.

## Candidate and deployment identity

- `git rev-parse HEAD`, live `/health`, the SPA footer, and the styled 404
  footer all report the exact full candidate SHA.
- Local and live SHA-256 hashes match byte for byte for hashed JS and CSS, the
  hero WebP, social image, favicon, touch icon, `robots.txt`, `sitemap.xml`,
  and 404 stylesheet.
- `/`, `/demo`, `/privacy`, and `/terms` return 200. A missing route returns the
  styled 404. All discovered internal links and both sample vendor links return
  200; in-page targets resolve through the browser.

This is fresh evidence that resolves any prior deployment-only uncertainty.

## Clean local quality gates

| Check | Result |
| --- | --- |
| `npm ci` | PASS — 60 packages; zero audit vulnerabilities |
| `npm test` | PASS — 10/10 |
| `npm run typecheck`; `npm run lint` | PASS |
| `npm run build` | PASS — produced `dist/` |
| `cargo fmt --all -- --check` | PASS — prior V19 blocker repaired |
| `cargo test --locked` | PASS — 28/28 |
| `cargo clippy --locked --all-targets -- -D warnings` | PASS |
| `cargo build --release --locked` | PASS — cold release build |
| `npm run test:browser` | PASS — 69 passed, 3 documented conditional skips |
| Live full browser suite | PASS — 69 passed, 3 documented conditional skips |
| Factory `verify-url.sh` | PASS — 200, 580 ms, no console errors, complete semantic checks |

The three browser skips are intentional guards around the isolated hosted rate
probe and one duplicate shared-backend burst. The live limiter was exercised
independently below. This worker has no Docker, Podman, or Buildah executable,
so OCI assembly could not be repeated locally. The Dockerfile contract test,
exact Vite build, locked Rust release build, and exact live build identity all
pass.

## End-to-end product exercise

- A real live workspace was created without an account. A malformed URL and a
  loopback URL returned readable `400` recovery messages, while a second token
  saw zero records from the first workspace.
- Stripe's public `stripe-node` release feed saved and scanned successfully.
  The scan created 10 current action cards with vendor permalinks; each carried
  the configured owner, dependency version, and check command. An action
  acknowledged successfully.
- Schedule value `14` was rejected with the documented 15-minute lower bound.
  A private webhook was rejected. A 60-minute schedule saved a next-run time,
  then stopped successfully. Removing the watch left the workspace empty.
- The real 390 px UI showed the invalid-URL error, recovered by saving a valid
  watch, and exposed a successful remove/recovery message.
- Claim coverage additionally verified the fourth-watch `409`, atomic rejected
  imports, keyword edits after reload, scan error retention, offline guidance,
  CSV/watch-file exports, and every documented API method.

## Backend concurrency, persistence, and rate limits

- A local release server returned the candidate SHA from `/health`. With one
  existing watch, six concurrent creates produced exactly two `201` and four
  `409` responses, preserving the three-watch maximum. Twenty-four parallel
  authenticated reads all returned `200`.
- The token and all three watches survived `SIGTERM`, graceful drain, and a
  reconnect to the same SQLite file. Both server stops logged graceful
  completion.
- The release binary also started successfully in a container-equivalent `/data`
  setup with an otherwise empty environment containing only `PORT`; `/health`
  returned 200.
- The complete `/api` router is behind the per-client limiter; `/health` is the
  documented exemption. After refill, a 100-request same-client live ingress
  burst produced exactly 40 authorization responses and 60 `429` responses.
  Every `429` carried `Retry-After: 1`, `Cache-Control: no-store, private`, and
  the matching one-second recovery message. Observed allowance: burst 40 with
  20 requests/second refill. Health remained 200.

## CLI package and clean consumer

`cargo package --locked --allow-dirty --no-verify` produced a 20-file crate,
251.4 KiB unpacked / 62.9 KiB compressed. It installed into a fresh temporary
Cargo root. The installed binary:

- printed public help and the two-card bundled demo;
- scanned the shipped local mapping into action `464f8e41f622`;
- reported no duplicate on a second scan;
- acknowledged the action in both JSON state and its Markdown card; and
- rejected `missing.json` with exit 1 and a readable error.

## Privacy, security headers, and caching

- A fresh direct `/demo` load requested only the document and same-origin JS
  and CSS. It made no API, analytics, advertising, font-CDN, AI, billing, or
  other third-party request. Demo actions remain in the demo storage namespace.
- Home requests were also same-origin only: shell assets plus private workspace
  creation and reads. Privacy and Terms make no workspace/API request.
- HTML and API responses send HSTS, `nosniff`, strict-origin referrer policy,
  restrictive permissions policy, and a header CSP with
  `frame-ancestors 'none'`. API and limited responses are `no-store, private`.
- HTML and health use `no-cache`; hashed JS/CSS use one-year immutable caching;
  the hero uses a seven-day public cache.
- The product has no sign-in, payment/unlock, runtime AI, service worker, or PWA
  claim. Entra, billing, AI-gateway, service-worker update, and offline-reload
  checks are therefore not applicable. Import/export is present; the brief
  explicitly excludes LLM summaries.

## Accessibility, responsive behavior, and performance

- Independent Playwright + Axe WCAG 2 A/AA checks found zero violations on
  home, demo, Privacy, Terms, and the 404: therefore zero serious/critical
  findings. The full repository Axe suite also passed on desktop and mobile.
- The first Tab focuses Skip to content with a visible 3 px dashed indigo ring;
  Enter focuses `main`. Space acknowledges an action and focus moves to that
  action card. There are no keyboard traps.
- At 390 px there is no horizontal overflow; active touch targets meet 44 px.
  The 195 px/200%-equivalent reflow check passes. Reduced-motion emulation
  leaves no active transition or animation.
- Semantic checks pass: route-specific title, `lang=en`, one H1, one main,
  ordered headings, alt text, named controls, and route focus announcements.
- Build output is 22,558 bytes JS raw / 7.65 kB gzip, 9,025 bytes CSS raw /
  2.79 kB gzip, and 58,974 bytes for the hero WebP.
- Fresh mobile Lighthouse on `/` scored **100 performance, 100 accessibility,
  100 best practices, 100 SEO**: FCP 1.0 s, LCP 1.3 s, TBT 0 ms, CLS 0,
  91 KiB total. `/demo` scored 99/100/100/100 with LCP 1.2 s, TBT 120 ms,
  CLS 0, and 33 KiB total.

Evidence is in `.factory/qa-artifacts/verification-20-verify-url/`,
`.factory/qa-artifacts/verification-20-lighthouse-home.json`, and
`.factory/qa-artifacts/verification-20-lighthouse.json`.

## Defects by severity

| Severity | Findings |
| --- | --- |
| Critical | None |
| High | None |
| Medium | None |
| Low | None |

## Release decision

**PASS.** Candidate `2c6f43ab489e9c34cb25513407ef24ddbebaaf88` is live,
identity-matched, and passes the acceptance contract.
