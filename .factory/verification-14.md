# Independent verification 14 — FAIL

Verified independently on 2026-08-29 UTC.

- Candidate: `d0d52f17be36cf336ac00583d94ba3e7183ad343`
- Live URL: <https://integration-changelog-watch.sociobot.in>
- Work order: `integration-changelog-watch-verify-14`
- Result: **FAIL — the candidate is not release-ready.**

The candidate is deployed, identity-matched, fast, accessible, and useful end to end. Release is blocked by a failing mandatory claim run, a global rather than per-client API limiter, and public scope copy that is broader than its registered claim test.

No product code was changed during verification.

## Mandatory first gates

### Claims — FAIL

`.factory/claims.json` exists and contains 21 claims. From the clean candidate checkout, `npm ci` installed 60 packages with zero audit findings. Every literal `test` command was then run in manifest order. Sixteen passed and five failed.

| Claim | Result in the required manifest run |
| --- | --- |
| `sample-action-cards` | PASS |
| `csv-export` | **FAIL — `http://127.0.0.1:8080/health is already used`** |
| `demo-local` | PASS |
| `demo-isolation-transitions` | **FAIL — backend bind panicked with address already in use** |
| `workspace-boundary` | **FAIL — health URL already used** |
| `hosted-scan-result` | PASS |
| `hosted-watch-limit` | PASS |
| `keyword-edit` | **FAIL — health URL already used** |
| `requested-scans` | PASS |
| `redirecting-feeds` | PASS |
| `watch-file-portability` | **FAIL — health URL already used** |
| `watch-file-rejection-preserves-watches` | PASS |
| `cli-more-feeds` | PASS |
| `cli-repository-workflow` | PASS |
| `cli-demo-local` | PASS |
| `cli-shipped-mapping-local` | PASS |
| `api-contract` | PASS |
| `container-build-stage` | PASS |
| `database-persistence` | PASS |
| `port-only-startup` | PASS |
| `single-replica-durable-data` | PASS |

The first browser command leaves its spawned backend listening long enough for a following literal command to fail. The pattern recurred throughout the same run. All five failed claims passed when rerun individually after waiting for port 8080 to become free, and the complete browser suite passed. That confirms a test-lifecycle defect rather than broken CSV, demo, workspace, keyword, or watch-file behavior. It does not change the claims contract: any failing claim test is release-blocking.

There is also an unlisted claim mismatch. The landing page and README say the CLI supports **“four or more watch mappings.”** The registered `cli-more-feeds` claim and its test prove exactly four mappings only. The broader public claim is not listed and tested as written.

### Cold first read — PASS

At 1440×900 and 390×844, the first viewport answers all three required questions:

- What: **Turn vendor changes into assigned action cards**.
- Who: **For engineers who maintain payment, auth, analytics, or messaging integrations.**
- First action: **Try it with sample data**, beside **See matched notices, owners, versions, and checks.**

On mobile, the heading, audience, action, and stated click result all fit above 519 px. One click opens `/demo` with realistic Stripe and Auth0 records, owners, affected versions, and checks. The persistent banner provides **Reset demo** and **Start a private workspace**.

## Candidate and deployment identity

- The clean checkout began at the exact requested SHA.
- Live `/health` returned `200` with `{"build":"d0d52f17be36cf336ac00583d94ba3e7183ad343","ok":true}`.
- Azure Container Apps reports image tag `d0d52f17be36`, revision `sf-integration-changelog-watch--0000049`, and 100% traffic to the latest revision.
- The live configuration has exactly one replica, only `PORT=8080`, and the `workspace-data` Azure Files volume mounted at `/data`.
- Live JS, CSS, and hero SHA-256 values exactly match the clean local production build.

This fresh evidence resolves any previous deployment-only uncertainty: the reviewed candidate is live.

## Local quality gates

| Check | Result |
| --- | --- |
| `npm ci` | PASS; 60 packages, 0 vulnerabilities |
| `npm test` | PASS; 6/6 |
| `npm run typecheck` | PASS |
| `npm run lint` | PASS |
| `npm run build` | PASS; produced `dist/` |
| `cargo fmt --all -- --check` | PASS |
| `cargo test --locked` | PASS; 21/21 |
| `cargo clippy --locked --all-targets -- -D warnings` | PASS |
| `cargo build --release --locked` | PASS |
| Full local Playwright | PASS; 63 passed, 3 intentional skips |
| Full live Playwright | PASS; 63 passed, 3 intentional skips |

Docker, Podman, and Buildah are unavailable in this verifier. The exact frontend build and locked optimized Rust build passed, the Dockerfile contract test passed, the candidate image is running live, and Azure confirms its port-only environment plus `/data` mount. A bare-host launch with only `PORT` is not representative and failed because this host has no `/data`; the README correctly tells host users to supply `DATABASE_URL`.

## End-to-end product evidence

A fresh live browser workspace completed the smallest useful job against the public Stripe Node releases feed:

1. `not-a-url` was rejected with **Enter a complete public http or https URL**.
2. The corrected feed saved with owner `Maya · Payments`, version `stripe-node 16.2`, keywords, and `pnpm test:stripe`.
3. An explicit scan created eight action cards.
4. The first card linked to the vendor release and retained its matched keyword, owner, dependency version, and check.
5. Acknowledgement updated the card.
6. A second scan created zero duplicates and retained eight cards.

Boundary and recovery checks passed: a 120-character vendor was accepted and 121 rejected; malformed, blank-owner, and loopback inputs returned readable `400` errors; three watches saved and the fourth returned `409`; another workspace read zero records. The live and local browser suites also passed 24 parallel authenticated reads. Rust tests passed atomic concurrent quota enforcement, persistence across reconnects, token hashing, feed redirect rejection, single-replica durability, and graceful shutdown.

## Backend and rate limiting — FAIL

The live allowance is enforced at a **40-request burst with a 20 requests/second refill**. A burst of 80 unauthenticated API calls produced exactly 40×`401` and 40×`429`; every `429` had `Retry-After: 1`. After 1.1 seconds, a 25-request burst produced 20×`401` and 5×`429`.

However, `rate_limit` deliberately keys every caller to `0.0.0.0`. Eighty requests carrying 80 different forwarded-client prefixes still shared one bucket. Immediately after that burst, `/health` also returned `429` with `Retry-After: 1`. This violates the mandatory per-client requirement and lets one anonymous caller consume the allowance for every visitor and the health endpoint. The single-replica topology makes this globally effective.

The feed fetch path also calls unbounded `response.text()` after a 12-second timeout. There is no content-length or streamed byte cap. An anonymous workspace can therefore point a scan at a very large public response and pressure the 1 GiB service memory. This is a medium reliability/security defect.

## Privacy, accessibility, routes, and headers

- A fresh `/demo` flow loaded only the same-origin document, hashed JS, and hashed CSS. Acknowledge, reset, and sample scan made no API or third-party request, set no cookie, and logged no error. Starting a private workspace removed demo storage before making the expected same-origin workspace calls.
- Cold root also made only same-origin requests. Deliberately rejected `400`/`409` API probes produced expected browser resource messages; ordinary route and demo loads had no console or page errors.
- Playwright axe found zero WCAG A/AA violations and zero serious/critical findings at 390 px.
- The demo has `lang="en"`, one `h1`, one `main`, complete alt text, ordered headings, and labelled controls.
- Keyboard Tab exposes the skip link with a 3 px dashed focus ring; Enter moves focus to main, and Space acknowledges an action without a trap.
- Reduced-motion mode computed no transitions, animations, or transforms. The 390 px and 195 px reflow views have no horizontal overflow. Visible controls meet the 44 px target; the hidden skip link expands to 44 px when focused.
- The factory `verify-url.sh` passed live `/demo` in 530 ms with no browser errors.
- Root, demo, privacy, terms, robots, sitemap, icons, social image, styled 404, and all visible links resolve as intended. The two external sample links returned `200`.
- Responses include HSTS, `nosniff`, strict-origin referrer policy, restrictive Permissions Policy, and header CSP with `frame-ancestors 'none'`. HTML is `no-cache`, API responses are `no-store, private`, hashed assets are one-year immutable, and the hero has a seven-day cache. No response set a cookie.

## Performance and bundle budgets

Fresh mobile Lighthouse on live `/demo` scored 100 for performance, accessibility, best practices, and SEO. FCP and LCP were 1.2 s, total blocking time was 0 ms, CLS was 0, and transfer was 30 KiB.

Production assets are within budget: JS 19,495 bytes raw / 6.81 KiB gzip; CSS 8,903 bytes raw / 2.76 KiB gzip; hero WebP 58,974 bytes; social image 163,227 bytes. No third-party fonts or scripts load.

## Clean CLI consumer

`cargo package --locked --allow-dirty --no-verify` passed with 18 files (221.3 KiB unpacked, 57.1 KiB compressed). The crate was extracted, then installed using a separate empty `CARGO_HOME`, install root, and target directory. The installed executable passed `--help` and `demo`; it scanned its packaged local feed, created action `464f8e41f622`, produced no duplicate on the second scan, and persisted acknowledgement in both JSON state and Markdown.

## Applicability notes

This is not a PWA and makes no offline-reload claim, so service-worker update and offline reload checks do not apply. It has no sign-in, runtime AI, payment, or paid-unlock call, so Entra, AI gateway, and billing checks do not apply. LLM summaries are explicitly outside the brief.

## Defects by severity

### High — release-blocking

1. **Five mandatory claim commands failed in the required manifest run.** The Playwright web server is not fully released before the next literal claim command starts, producing port-8080 collisions. Isolated reruns and full suites pass, but the acceptance contract explicitly makes any failed claim test blocking. Provide a reliable aggregate claim runner or fix backend teardown, then rerun all 21 commands from one clean install.
2. **The live API allowance is global, not per client.** The implementation hard-codes the limiter key to `0.0.0.0`. Forty calls by one anonymous actor also throttle every other visitor and `/health`. Use a trustworthy ingress-derived client identity or enforce the client-scoped allowance at ingress; keep `429` plus `Retry-After` and add a two-client isolation regression.
3. **“Four or more watch mappings” is broader than its registered claim.** The manifest and tagged test prove exactly four. Narrow the copy or add a claim/test that proves the advertised scope.

### Medium

1. **Hosted feed bodies are not size-limited.** `fetch_public` buffers `response.text()` without a byte cap. Reject oversized `Content-Length` values and enforce a streamed maximum before parsing.

## Release decision

**FAIL.** The candidate’s core workflow, deployment identity, privacy behavior, accessibility, build, performance, persistence boundary, and CLI all verify. It cannot pass this acceptance contract until the mandatory claim run is reliable and the backend enforces a per-client rather than global request allowance. The unlisted scope claim must also be narrowed or tested.
