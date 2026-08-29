# Independent verification 10 — PASS

Verified independently on 2026-08-29 UTC.

- Candidate: `99f0ca341a13140030b4f50272b4b399c54cbd57`
- Live URL: `https://integration-changelog-watch.sociobot.in`
- Work order: `integration-changelog-watch-verify-10`
- Result: **PASS — candidate is release-ready.**

## Candidate and deployment identity

The clean checkout was detached at the requested candidate (`git rev-parse HEAD` = `99f0ca341a13140030b4f50272b4b399c54cbd57`). The live footer and `GET /health` both report that exact full SHA. This fresh evidence resolves the earlier report's unavailable, different SHA; it is not a deployment-only failure.

## Mandatory first gates

### Claims — PASS

`.factory/claims.json` exists and has 13 claims. After `npm ci` (60 packages, zero vulnerabilities), every literal listed command passed serially from the candidate checkout:

`sample-action-cards`, `csv-export`, `demo-local`, `workspace-boundary`, `redirecting-feeds`, `requested-scans`, `cli-repository-workflow`, `cli-demo-local`, `database-persistence`, `demo-isolation-transitions`, `cli-shipped-mapping-local`, `port-only-startup`, and `single-replica-durable-data`.

### Cold first read — PASS

The first screen says what it does—“Turn vendor changes into owned actions”—and names its audience: engineers maintaining payment, auth, analytics, or messaging integrations. Its first action is the one-click **Try it with sample data**, beside the plain result “See matched notices, owners, versions, and checks.” Clicking it opens `/demo` with realistic action cards, a persistent “Demo — sample data, nothing is saved” banner, Reset demo, and Start for real.

## Local quality evidence

| Check | Result |
| --- | --- |
| `npm test` | PASS, 5/5 |
| `npm run typecheck`; `npm run lint` | PASS |
| `npm run build` | PASS; produced `dist/` |
| `cargo fmt --all -- --check` | PASS |
| `cargo test --locked` | PASS, 17/17 |
| `cargo clippy --all-targets --locked -- -D warnings` | PASS |
| `cargo build --release --locked`; `npm run test:container` | PASS |
| `npm run test:browser` | PASS, 47 passed / 1 intentional duplicate-burst skip |
| `PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run test:browser` | PASS, 47 passed / 1 intentional duplicate-burst skip |
| `cargo package --locked --allow-dirty --no-verify` | PASS; clean extracted consumer ran `--help` and `demo` |

The exact Docker image build was attempted but cannot run in this worker because `docker`, `podman`, and `buildah` are absent. The locked release build that the Dockerfile performs passed. This is an environment limitation, not a source/build failure.

## Product and backend exercise

- Demo end to end: sample action cards show matched notice, owner, dependency version, and check; keyboard Space acknowledges an action; Reset demo restores the sample; CSV export claim passed.
- Live workspace: `POST /api/workspaces` returned `201`; a public GitHub feed saved with `201`; loopback input returned the readable `400` private-network error; the fourth watch returned `409` with the three-watch recovery instruction; an explicit scan returned its two redirecting-feed errors; all three test watches were deleted with `204`.
- Live concurrency coverage passed in the browser suite (24 parallel authenticated reads all `200`).
- Rate limit: after refill, one client sent 80 simultaneous `GET /api/watches` requests. The result was exactly 40 `401` followed by 40 `429`; each limited response carried `Retry-After: 1`. Observed allowance: 40-request burst, then 20 requests/second refill.
- The product has no sign-in, PWA/service worker, runtime AI, billing, or unlock flow. Entra, service-worker update, AI gateway, and payment checks are therefore not applicable.

## Live privacy, accessibility, and performance evidence

- A fresh `/demo` request log contained only the document, local JS, local CSS, and local hero WebP: four same-origin requests and **no API call**, analytics, advertising, remote font, or third-party request. Acknowledging demo data remained local to the demo namespace.
- Live HTML/API headers include HSTS, `X-Content-Type-Options: nosniff`, `Referrer-Policy: strict-origin-when-cross-origin`, restrictive Permissions Policy, and a header-delivered CSP with `frame-ancestors 'none'`. HTML is `no-cache`; hashed JS is `public, max-age=31536000, immutable`; API responses are `no-store, private`.
- Factory `verify-url.sh` passed: HTTP 200, 801 ms network-idle load, no console/page errors, title/lang/one `h1`/`main`, no missing image alt text, and no unnamed buttons. Evidence: `.factory/qa-artifacts/verification-10/`.
- Independent Playwright + Axe found zero serious/critical WCAG A/AA issues. Desktop keyboard starts on Skip to content with a visible 3px dashed focus ring and Enter moves focus to `main`; 390px has no horizontal overflow; reduced-motion computes to no transition and no transform.
- Production assets are within budget: JS 14,416 bytes raw / 5,538 bytes gzip; CSS 7,942 bytes raw / 2,562 bytes gzip; hero WebP 58,974 bytes. No Lighthouse binary is installed in this worker; the live browser suite and factory verifier supplied the Lighthouse-class checks above.

## Defects by severity

- **Critical / high / medium:** none.
- **Minor:** Axe reports one moderate `landmark-complementary-is-top-level` node on `/demo`. It is outside the required serious/critical gate and does not block release; make that nested complementary region a non-landmark or top-level landmark in a future cleanup.

## Release decision

**PASS.** The stated candidate is live, identity-matched, and passes the mandatory claims, functional, privacy, accessibility, backend, and local quality gates.
