# Independent verification 13 — FAIL

Verified independently on 2026-08-29 UTC.

- Candidate: `f7674a134cf4081857606be255dfcf51781d3408`
- Live URL: <https://integration-changelog-watch.sociobot.in>
- Work order: `integration-changelog-watch-verify-13`
- Result: **FAIL — the candidate is not release-ready.**

The exact candidate is deployed and the product's real hosted and CLI workflows work. Release is blocked because one mandatory command in `.factory/claims.json` fails from the clean checkout and against the live site. The failure is a concurrency-sensitive defect in the claim test, not evidence that the documented delete endpoint is broken. The live rate limit is also bypassable with a caller-supplied `X-Forwarded-For`, and the legal pages miss the required 44 px mobile touch target for their visible **Return home** link.

No product code was changed during verification.

## Mandatory first gates

### Claims — FAIL

The checkout began clean at the requested SHA. `.factory/claims.json` exists with 21 entries. After `npm ci` installed 60 packages with zero audit findings, every literal `test` command was run serially. Twenty passed and one failed.

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
| `watch-file-rejection-preserves-watches` | PASS |
| `cli-more-feeds` | PASS |
| `cli-repository-workflow` | PASS |
| `cli-demo-local` | PASS |
| `cli-shipped-mapping-local` | PASS |
| `api-contract` | **FAIL: both default desktop and mobile projects received `404` for deletion where the test expects `204`** |
| `container-build-stage` | PASS |
| `database-persistence` | PASS |
| `port-only-startup` | PASS |
| `single-replica-durable-data` | PASS |

The exact failing command was:

```text
npm run build && npm run test:browser -- --grep @claim:api-contract
```

The failure reproduces against the live deployment as well as locally. With the normal two workers, both browser projects fail. With `--workers=1`, both pass locally and live. The cause is deterministic under concurrent execution: the test creates a watch, calls `POST /api/watches/import` (documented to replace all watches), and then deletes the original pre-import ID. Concurrent projects prevent SQLite from coincidentally reusing that deleted row ID, so the correct server response is `404`. A serial run can reuse the maximum ROWID and masks the bad assertion. The test must delete the ID returned by import, or cover delete before replacement.

Full output: `qa-artifacts/verification-13/claims.log`, `a11y-and-live-claim.log`, and `api-claim-serial.log`.

No unlisted public claim was found in the live landing page, legal pages, or README.

### Cold first read — PASS

At 1440×900 and 390×844, the first screen plainly answers all three questions:

- What: **Turn vendor changes into assigned action cards**.
- Who: **For engineers who maintain payment, auth, analytics, or messaging integrations.**
- First action: **Try it with sample data**, beside **See matched notices, owners, versions, and checks.**

The action is inside the first mobile viewport. One click opens `/demo` with a Stripe notice, owner, affected version, and check already visible. The persistent banner says **Demo — sample data, nothing is saved** and provides **Reset demo** and **Start a private workspace**.

Screenshots: `qa-artifacts/verification-13/first-read-desktop.png`, `live-demo-desktop.png`, and `live-demo-mobile.png`.

## Candidate and deployment identity

- `git rev-parse HEAD` returned the requested full SHA.
- Live `/health` returned `200` with `{"build":"f7674a134cf4081857606be255dfcf51781d3408","ok":true}`.
- The live footer reports the same full SHA.
- Azure reports image tag `f7674a134cf4`, revision `sf-integration-changelog-watch--0000044`, and 100% traffic to the latest revision.
- Live JS, CSS, and hero bytes match the clean local build by SHA-256.
- Azure reports one minimum/maximum replica and the `workspace-data` Azure Files volume mounted at `/data`.

This fresh evidence resolves any prior deployment-only uncertainty: the reviewed candidate is live.

## Local quality gates

| Check | Result |
| --- | --- |
| `npm ci` | PASS; 60 packages, 0 vulnerabilities |
| `npm test` | PASS; 6/6 |
| `npm run typecheck` | PASS |
| `npm run lint` | PASS |
| `npm run build` | PASS; produced `dist/` |
| `cargo fmt --all -- --check` | PASS |
| `cargo test --locked` | PASS; 19/19 |
| `cargo clippy --locked --all-targets -- -D warnings` | PASS |
| `cargo build --release --locked` | PASS |
| Full local Playwright | PASS; 63 passed, 1 intentional shared-rate-limit skip |
| Full live Playwright | PASS; 63 passed, 1 intentional shared-rate-limit skip |
| Local and live accessibility suites | PASS; 18/18 each |

The full suites pass because project scheduling can allow SQLite ROWID reuse; the isolated default claim command above is the required release gate and fails. Docker, Podman, and Buildah are unavailable in this verifier image. The Dockerfile contract test, exact frontend build, locked optimized Rust build, port-only configuration test, and running candidate image all passed.

## End-to-end product evidence

A fresh live browser workspace completed the smallest useful job against the public Stripe Node release feed:

1. workspace creation returned `201`;
2. `not-a-url` returned `400` with the recovery instruction **Enter a complete public http or https URL**;
3. a corrected watch saved with owner, version, keyword, and check;
4. an explicit scan produced seven action cards;
5. the first card linked to the vendor release and retained its matched keyword, owner, dependency version, and command;
6. acknowledgement returned `200` and updated the card;
7. a second scan created zero duplicate actions;
8. removing the watch returned `204` and restored the empty state.

Independent API boundaries also passed: a 120-character vendor was accepted and 121 rejected; blank required input, malformed URLs, and loopback sources returned readable `400` responses; three watches saved and the fourth returned `409`; a second workspace could not read the first; a rejected private-address import preserved the existing watch; and 24 parallel authenticated reads all returned `200`.

Evidence: `qa-artifacts/verification-13/live-e2e-ui.json`, `live-invalid-recovery.json`, and `live-api-boundaries.json`.

## Backend, persistence, and rate limiting

- The live deployment has one replica and a durable Azure Files `/data` mount, matching the SQLite persistence boundary.
- Unit tests cover reconnect persistence, concurrent quota enforcement, hashed tokens, forwarded-client rate buckets, topology restoration, and graceful SIGTERM.
- A fresh live burst sent 80 simultaneous unauthenticated API requests with one fixed forwarded address in 395 ms. Exactly 40 returned `401` and 40 returned `429`; every `429` included `Retry-After: 1`.
- Observed nominal allowance: **40-request burst with a 20 requests/second refill**. The limiter middleware wraps every API route, including health.
- The ingress does not enforce the code's assumption that it sanitizes the first `X-Forwarded-For` hop. Sixty additional requests from the same network client, each with a different caller-supplied first hop, all returned `401`; none returned `429`. A client can therefore evade the allowance simply by rotating this header.

Evidence: `qa-artifacts/verification-13/live-rate-limit.json`, `live-rate-limit-bypass.json`, and `live-identity-headers.log`.

## Clean CLI consumer

`cargo package --locked --allow-dirty --no-verify` passed with 18 packaged files (218.0 KiB unpacked, 56.6 KiB compressed). The archive was extracted and installed with a new `CARGO_HOME` and install root. The installed executable passed `--help` and `demo`; it scanned the packaged sample into action `464f8e41f622`, produced no duplicate on a second scan, and persisted acknowledgement in both JSON state and Markdown.

Evidence: `qa-artifacts/verification-13/cli-consumer.log`.

## Privacy, accessibility, routes, and headers

- A fresh direct `/demo` load at desktop and mobile requested only the same-origin document, hashed JS, and hashed CSS. It made no API or third-party request, set no cookie, and produced no console/page error.
- The demo has one `h1`, one `main`, `lang="en"`, labelled controls, and no missing image alt text. Axe reported zero WCAG A/AA violations in both viewports.
- Keyboard Tab exposes the skip link with a 3 px dashed indigo focus ring. Existing browser coverage confirms Enter activates it, Space acknowledges an action, route changes restore heading focus, and there is no trap.
- Reduced-motion mode computes zero-duration motion and no transform. At 390 px there is no horizontal overflow.
- Root, demo, privacy, terms, robots, sitemap, icons, social card, and the styled 404 resolve correctly. Both external sample notice links return `200`.
- The factory `verify-url.sh` passed `/demo` in 544 ms with no console errors and all structural checks passing.
- Responses include HSTS, `nosniff`, strict-origin referrer policy, restrictive Permissions Policy, and header CSP with `frame-ancestors 'none'`. HTML is `no-cache`, APIs are `no-store, private`, hashed assets are one-year immutable, and the hero is cached for seven days.

The product has no service worker, offline-reload claim, sign-in, runtime AI, payment, or paid unlock. PWA update/offline reload, Entra, AI-gateway, and billing checks are not applicable. The brief explicitly makes LLM summaries a non-goal.

## Performance and bundle budgets

Fresh mobile Lighthouse on live `/demo` scored 100 for performance, accessibility, best practices, and SEO. FCP and LCP were 1.0 s, total blocking time 70 ms, CLS 0, and initial transfer 30 KiB.

Production assets are within budget: JS 19,475 bytes raw / 6.81 KiB gzip; CSS 8,801 / 2.73 KiB gzip; hero WebP 58,974 bytes; social image 163,227 bytes. No remote font or script loads.

Evidence: `qa-artifacts/verification-13/lighthouse-summary.json` and `lighthouse-live.json`.

## Defects by severity

### High — release-blocking

1. **The declared `api-contract` claim fails under its exact default command.** Both default Playwright projects receive `404` where the test expects `204`. The test deletes the pre-import ID after atomically replacing the watch set and only passes serially when SQLite happens to reuse that ID. This makes a mandatory claim nondeterministic and unprovable under the repository's supported default runner. Update the scenario to delete the imported watch's returned ID, then rerun every literal claim command.
2. **The live per-client rate limit trusts a spoofable header.** A fixed caller-supplied `X-Forwarded-For` is limited after 40 requests, but 60 requests from the same client with 60 different forged values all bypassed the limiter. Each forged address also creates a permanent entry in the in-memory bucket map. The production ingress demonstrably preserves or accepts the caller's first hop, contrary to the server comment. Sanitize/overwrite the header at ingress or derive the trusted client address from a verified proxy chain, evict stale buckets, and add a live spoof-resistance test.

### Medium

1. **The visible legal-page return links miss the required mobile touch target.** At 390 px, **Return home** on `/privacy` and `/terms` measures about 103.5×19 px. The attached baseline requires every touch target to be at least 44×44 px. Give this link a block/inline-flex hit area with at least 44 px height and preserve its visible focus treatment. The skip link's off-screen resting box is not included in this finding because it expands to 44 px when focused.

## Release decision

**FAIL.** The deployed product is identity-matched, useful end to end, fast, private in demo mode, and durable within its documented topology. It cannot pass the acceptance contract until every exact claim command is reliable and the live rate limit cannot be bypassed; the legal-page touch targets also need correction.
