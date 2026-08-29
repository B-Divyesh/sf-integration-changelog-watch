# Independent verification 16 — FAIL

**Requested candidate:** `b7db705d7a157da83a4b15f4d54f3814454ac94c`  
**Available checkout / deployed build:** `b7db70ecfc5041b1b817afd504f4b559071ceb60`  
**Live URL:** <https://integration-changelog-watch.sociobot.in>  
**Date:** 2026-08-29 UTC  
**Verdict:** **FAIL — do not release the requested candidate.**

## Release blocker

The requested SHA is not an object in the supplied clean clone. `git show` and `git rev-parse --verify` both returned `bad object` / `Needed a single revision`; `git fetch --prune origin` did not make it available. This prevents a candidate checkout and reproducible verification.

The live deployment is explicitly a different revision. Its cold HTML has `data-build="b7db70ecfc5041b1b817afd504f4b559071ceb60"`, its footer shows the same build, and `GET /health` returned:

```json
{"build":"b7db70ecfc5041b1b817afd504f4b559071ceb60","ok":true}
```

Therefore the deployed site cannot be claimed to match `b7db705d7a157da83a4b15f4d54f3814454ac94c`.

## First-read test — PASS for the deployed revision

A new desktop Chromium context loaded the live home page cold with no cache, console errors, or page errors. The first screen says **“Turn vendor changes into assigned action cards”**, says it is **“For engineers who maintain payment, authentication, analytics, or messaging integrations,”** and presents **“Try it with sample data”** with **“See matched notices, owners, versions, and checks.”** beside it.

Thus the deployed revision clearly answers what it does, for whom, and what to click first in plain words. One click opens `/demo`, where a 390 px viewport immediately shows realistic action cards, owner, dependency version, local check, and the persistent **“Demo — sample data, nothing is saved”** banner with Reset and Start-a-private-workspace controls.

## Mandatory claims gate — PASS on the available clean revision only

`.factory/claims.json` exists. After `npm ci` (60 packages, zero audit vulnerabilities), `npm run test:claims` executed all **21 literal manifest commands** against the shipped demo/server entry point. The runner ended:

```text
All 21 literal claim commands passed without leaking port 8080.
```

The full output was captured during a second clean invocation in `/tmp/icw-claims-second-run.log`; Playwright's status file records `passed` with no failed tests. This is evidence for `b7db70ec…`, not the unavailable requested SHA.

| Claims exercised | Result |
| --- | --- |
| `sample-action-cards`, `csv-export`, `demo-local`, `demo-isolation-transitions`, `workspace-boundary` | PASS |
| `hosted-scan-result`, `hosted-watch-limit`, `keyword-edit`, `requested-scans`, `redirecting-feeds` | PASS |
| `watch-file-portability`, `watch-file-rejection-preserves-watches`, `cli-more-feeds`, `cli-repository-workflow`, `cli-demo-local`, `cli-shipped-mapping-local` | PASS |
| `api-contract`, `container-build-stage`, `database-persistence`, `port-only-startup`, `single-replica-durable-data` | PASS |

## Local quality gates — PASS on the available clean revision

The following completed successfully:

```sh
npm test                    # 7/7 Vitest tests
npm run typecheck
npm run lint
cargo test --locked         # 23/23 Rust tests
npm run build               # creates dist/
cargo build --release --locked
npm run test:browser        # 68/68 Playwright tests
```

Vite's production output is 19.82 kB raw / 6.95 kB gzip JavaScript and 8.90 kB raw / 2.76 kB gzip CSS. The 58,974-byte hero WebP and initial JS are below the stated budgets. The exact Docker image build could not run because this verifier environment has no `docker`, `podman`, or `buildah`; the locked native release build and Docker build-stage claim test did pass.

The public CLI was also exercised independently from a fresh temporary consumer-style workspace. `demo` printed the shipped Stripe/Auth0 action cards; `scan --config watches.json` created one Markdown card from the bundled local feed; `ack --id 464f8e41f622` updated both that card to **Acknowledged** and `.integration-changelog-watch/state.json` to `"acknowledged": true`.

## Product, privacy, accessibility, and delivery evidence — PASS for deployed revision

- Live `/`, `/demo`, `/privacy`, and `/terms` returned 200 with their route-specific title and one H1. A missing route returned a styled 404 with a return-home link. Every landing-page link returned 200.
- Fresh live `/demo` made only three same-origin document/JS/CSS requests and **no `/api/` calls**. Fresh landing-page requests were all same-origin; no analytics, advertising, remote fonts, or third-party scripts appeared. The landing page's expected same-origin workspace calls were `POST /api/workspaces`, then reads of watches and actions.
- A live Axe WCAG 2 A/AA scan found zero violations on desktop home and 390 px demo (therefore zero serious/critical findings). Both screens had no console/page errors, one `main`, `lang="en"`, and a first-Tab skip link with a visible `rgb(40, 79, 122) dashed 3px` focus outline. Mobile had `scrollWidth === clientWidth`; reduced-motion emulation reported no transitions.
- Response headers include CSP with `frame-ancestors 'none'`, `X-Content-Type-Options: nosniff`, strict referrer policy, HSTS, and permissions policy. HTML is `no-cache`; the hashed JS/CSS are `public, max-age=31536000, immutable`; the hero has `max-age=604800`.
- API limiter: a 100-request concurrent same-client probe completed in 516 ms with **60 × 401 and 40 × 429**. The first 429 had `Retry-After: 1` and `Cache-Control: no-store, private`. The implemented policy is a 40-request burst refilling at 20 requests/second; the observed run crossed the allowance and enforced 429 as required. `/health` remained 200.

## Defects by severity

| Severity | Finding |
| --- | --- |
| Critical | None found in the inspected deployed revision. |
| High | **Requested candidate is unavailable and not deployed.** `b7db705d…` cannot be checked out after fetching origin; live identity is `b7db70ec…`. This blocks reproducible acceptance of the requested candidate. |
| Medium | None found in the inspected deployed revision. |
| Low | Docker image build was not executable in this verifier container because no OCI build tool is installed. This is an evidence gap, not a product-code failure; native locked release build and the versioned Docker contract test passed. |

## Required next step

Publish/fetch the exact requested commit and deploy that exact SHA, then rerun this verification against it. Do not treat the successful checks of `b7db70ec…` as acceptance of `b7db705d…`.
