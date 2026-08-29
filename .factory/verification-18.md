# Independent verification 18 — PASS

**Candidate:** `818b868c0ba7ecece8fdae9b4abb4d6b927bdae1`

**Live URL:** <https://integration-changelog-watch.sociobot.in>

**Date:** 2026-08-29 UTC

**Verdict:** **PASS — release the candidate.**

No release-blocking product defect was found. This is fresh evidence; the earlier deployment-identity failure is resolved. Both `GET /health` and the live HTML `data-build` marker returned the exact candidate SHA above.

## Mandatory first-read test — PASS

A new desktop Chromium context opened the live home page with empty browser storage. The first screen states:

- What it does: **“Turn vendor changes into assigned action cards.”**
- Who it is for: **“For engineers who maintain payment, authentication, analytics, or messaging integrations.”**
- What to click first: **“Try it with sample data,”** followed by **“See matched notices, owners, versions, and checks.”**

One click opened `/demo`. The resulting first screen already showed the persistent **“Demo — sample data, nothing is saved”** banner and a realistic Stripe action with its owner, dependency version, and local check. Evidence: `qa-artifacts/live-cold-desktop.png` and `qa-artifacts/verification-18-live-demo-mobile.png`.

## Mandatory claims gate — PASS

`.factory/claims.json` exists and contains 21 claims. After `npm ci`, every literal `test` command was run separately from this clean candidate checkout. Every command exited 0. Browser claim commands ran against both configured projects; Rust claim commands ran the named backend test.

| Claim | Result | Observed evidence |
| --- | --- | --- |
| `sample-action-cards` | PASS | One-click 390 px demo exposed the matched notice, owner, version, and check. |
| `csv-export` | PASS | Download contained a header and one row per sample action. |
| `demo-local` | PASS | Demo used its own storage and only same-origin resources. |
| `demo-isolation-transitions` | PASS | No demo API call; reset restored the seed; leaving discarded demo state. |
| `workspace-boundary` | PASS | Anonymous access failed, tokens isolated rows, and loopback input was rejected. |
| `hosted-scan-result` | PASS | Controlled RSS match created the complete hosted action-card record. |
| `hosted-watch-limit` | PASS | Three watches succeeded; the fourth produced the documented readable limit. |
| `keyword-edit` | PASS | Edited keywords survived a reload. |
| `requested-scans` | PASS | The scan route was called only after the explicit button action. |
| `redirecting-feeds` | PASS | Redirect responses were rejected with the final-address instruction. |
| `watch-file-portability` | PASS | CLI-schema export/import worked and malformed JSON was rejected. |
| `watch-file-rejection-preserves-watches` | PASS | A private-address import failed without replacing the saved watch. |
| `cli-more-feeds` | PASS | Four repository mappings produced four local action records. |
| `cli-repository-workflow` | PASS | Scan deduplicated cards; acknowledgement updated state and Markdown. |
| `cli-demo-local` | PASS | Demo printed bundled cards without contacting the recording proxy. |
| `cli-shipped-mapping-local` | PASS | The shipped local feed scanned without a network request. |
| `api-contract` | PASS | Every documented method plus authorization/error paths returned the expected status. |
| `container-build-stage` | PASS | Dockerfile uses `rust:1-alpine` and a locked release build. |
| `database-persistence` | PASS | Workspace state survived a SQLite reconnect at the mounted-data equivalent path. |
| `port-only-startup` | PASS | Runtime defaults require only `PORT`; the container filesystem provides `/data`. |
| `single-replica-durable-data` | PASS | Deployment guard restored one replica and the durable `/data` mount. |

Landing, legal, README, CLI-help, and demo documentation claim-like statements were cross-checked against the manifest. No unlisted user-facing claim was found.

## Clean local quality gates — PASS

| Command | Result |
| --- | --- |
| `npm ci` | 60 packages installed; 0 audit vulnerabilities |
| `npm test` | 9/9 Vitest tests passed |
| `npm run typecheck` | PASS |
| `npm run lint` | PASS |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --locked --all-targets -- -D warnings` | PASS |
| `cargo test --locked` | 23/23 passed |
| `npm run build` | PASS; generated `dist/` |
| `cargo build --release --locked` | PASS |
| `npm run test:browser` | 65 passed; 3 deliberate hosted-live probes skipped |
| `npm run test:a11y` | 20/20 passed |

The three skipped full-suite cases are intentional project/environment guards, not missing coverage: the destructive hosted rate-limit probe is separately executed below, and shared backend bursts run only in the desktop project to avoid racing one client bucket.

## End-to-end product behavior — PASS

- Demo: loaded two realistic action cards and three watches; acknowledged the pending card; reset restored it; CSV and watch-file paths passed their claims.
- Live workspace: a loopback feed was rejected with **“Use a public internet address; private, loopback, and link-local networks are blocked.”** The same session then saved a public watch, edited keywords/version/check, scanned it, showed a useful redirect error, and removed it. API statuses were `400`, `201`, `200`, `200`, and `204` on those respective writes. Evidence: `qa-artifacts/verification-18-live-workspace-recovery.png`.
- Boundary and recovery: three-watch atomic limit, rejected-import preservation, oversized-feed rejection, deduplication, missing-action response, concurrent creates, and scan error retention all passed the browser/Rust suites.
- CLI consumer: `cargo package --allow-dirty --no-verify` produced 20 files / 231.5 KiB source / 59.7 KiB compressed. Installing that package into a fresh temporary root succeeded. Its `demo` printed both sample cards; `scan` created action `464f8e41f622`; `ack` changed both Markdown and JSON state to acknowledged.

## Privacy, security, and delivery — PASS

- Cold live home requests were same-origin only: document, hashed JS/CSS, image, workspace creation, and workspace reads. There were no analytics, ads, CDN fonts, or third-party scripts.
- A fresh live `/demo` requested only its document and the same-origin JS/CSS. It made no `/api` request. A fresh `/privacy` also made no API request.
- Cold, valid, demo, and legal flows had no console or page errors. Chrome logged one expected failed-resource message for the deliberately exercised `400` loopback rejection; no application exception followed, and recovery succeeded.
- Security headers include CSP with `frame-ancestors 'none'`, HSTS, `nosniff`, strict-origin referrer policy, and a restrictive permissions policy.
- HTML and health use `Cache-Control: no-cache`; hashed JS/CSS use `public, max-age=31536000, immutable`; the hero uses a seven-day public cache.
- `/`, `/demo`, `/privacy`, and `/terms` return 200 with route-specific metadata. The designed missing route returns 404. All navigational and sample vendor links resolved; `robots.txt` and `sitemap.xml` are present.
- No service worker or offline/PWA claim is shipped, so update/offline-reload testing is not applicable. The demo's deliberate offline scan recovery passed locally.
- No sign-in, AI action, or paid unlock is present. Entra, AI-gateway, and billing checks are therefore not applicable. The brief explicitly excludes LLM summaries; watch-file import/export supplies the obvious portability feature.

## Accessibility and responsive behavior — PASS

- Fresh live Axe WCAG 2 A/AA scans found zero violations on desktop home, 390 px demo, and 390 px privacy; therefore zero serious/critical findings.
- The first Tab focuses **Skip to content** with a visible `3px` dashed indigo ring; Enter moves focus to `main`. Keyboard Space acknowledges a demo action in the repository suite.
- `lang="en"`, route titles, one H1, main landmark, heading order, alt text, named controls, live status text, route focus restoration, and the styled 404 all passed.
- The demo has no horizontal overflow at 390 px or the 195 px 200%-text equivalent. The important first action, owner, dependency, and check all fit in the first 390 × 844 viewport.
- Visible navigation and app controls meet the 44 px touch-target baseline. Reduced-motion emulation left no active transition or animation.
- `/opt/fleet/lib/verify-url.sh` passed the live URL: load 814 ms, no errors, title/lang/one-H1/main/alt/control-name checks all good. Evidence: `qa-artifacts/verification-18-verify-url/verify.json` and its screenshots.

## Performance — PASS

The exact Vite build emitted 19.82 kB raw / 6.95 kB gzip JavaScript and 8.90 kB raw / 2.76 kB gzip CSS. The hero WebP is 58,974 bytes. These are comfortably below the 200 kB JS, 50 kB CSS, and 300 kB hero budgets.

Fresh mobile Lighthouse against the live candidate scored **99 performance, 100 accessibility, 100 best practices, and 100 SEO**. FCP was 1.0 s, LCP 1.3 s, TBT 130 ms, CLS 0, and total transfer 88 KiB. Evidence: `qa-artifacts/verification-18-lighthouse.json`.

## Backend, persistence, concurrency, and rate limit — PASS

- Live `GET /health` returned `200 {"build":"818b868c0ba7ecece8fdae9b4abb4d6b927bdae1","ok":true}`.
- A 100-request same-client live burst completed in 440 ms: exactly **40 × 401** reached authorization and **60 × 429** were limited. Every 429 included `Retry-After: 1` and `Cache-Control: no-store, private`. Health remained 200. Observed allowance: burst 40, refill 20 requests/second.
- The limiter wraps the entire `/api` router; health is intentionally exempt. Tests also passed spoof-resistant forwarded-client selection, independent client buckets, idle eviction, restart boundaries, and single-replica enforcement.
- The container contract creates and owns `/data`. Reproducing that filesystem and launching the release binary with `env -i PORT=8099` succeeded twice. A workspace token created before SIGTERM still authorized a 200 read after restart; both shutdowns drained cleanly.
- No Docker, Podman, or Buildah executable exists in this worker, so an OCI image build/run could not be repeated. The exact native locked release build, Dockerfile contract claim, non-root `/data` setup inspection, and port-only container-equivalent runtime all passed. This is an environment limitation, not a product defect.

## Defects by severity

| Severity | Findings |
| --- | --- |
| Critical | None |
| High | None |
| Medium | None |
| Low | None |

## Evidence index

- `qa-artifacts/live-cold-desktop.png`
- `qa-artifacts/verification-18-live-demo-mobile.png`
- `qa-artifacts/verification-18-live-workspace-recovery.png`
- `qa-artifacts/verification-18-live-home-desktop.png`
- `qa-artifacts/verification-18-live-privacy-mobile.png`
- `qa-artifacts/verification-18-lighthouse.json`
- `qa-artifacts/verification-18-verify-url/verify.json`
- Reproducible live probes: `qa-artifacts/verification-18-live-audit.mjs` and `qa-artifacts/verification-18-live-workspace.mjs`
