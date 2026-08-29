# Independent verification 19 — FAIL

**Candidate:** `10375ffd1868aa787b7b6d0fa84de02123ad7e06`

**Live URL:** <https://integration-changelog-watch.sociobot.in>

**Date:** 2026-08-29 UTC

**Verdict:** **FAIL — do not release this candidate.**

The product works end to end and the live deployment exactly matches the candidate, but one required local quality gate fails. `cargo fmt --all -- --check` exits 1 with formatting drift in `src/main.rs`. Independent verification did not change product code.

## Release-blocking finding

### V19-1 — Rust formatting check fails (Medium)

`cargo fmt --all -- --check` exits 1. Rustfmt reports changes around the scheduler implementation and tests at `src/main.rs` lines 934, 973, 1343, 2322, 2384, 2445, and 2500. The diff is mechanical wrapping, but a candidate with a failing available lint/format gate does not meet the quality-gate contract.

Reproduce:

```sh
cargo fmt --all -- --check
```

Expected: exit 0 and no diff. Observed: exit 1 and seven formatting hunks.

## Mandatory first-read test — PASS

A new 1440 × 900 Chromium context with empty storage opened the live home page. Without scrolling, the first screen states:

- What it does: **“Turn vendor changes into assigned action cards.”**
- Who it serves: **“For engineers who maintain payment, authentication, analytics, or messaging integrations.”**
- What to click: **“Try it with sample data,”** followed by **“See matched notices, owners, versions, and checks.”**

One click opened `/demo`. The first demo screen showed **“Demo — sample data, nothing is saved,”** a pending Stripe notice, owner **Maya · Payments**, dependency **stripe-node 16.2**, and check **pnpm test:stripe**. Evidence: `qa-artifacts/verification-19-live-home.png` and `qa-artifacts/verification-19-live-demo.png`.

## Mandatory claims gate — PASS

`.factory/claims.json` exists with 28 entries. After `npm ci`, every literal `test` command was run separately from candidate commit `10375ffd…`; all exited 0. Browser claim commands exercised both configured Chromium projects.

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
| `online-feed-scans` | PASS |
| `no-account-or-payment` | PASS |
| `redirecting-feeds` | PASS |
| `watch-file-portability` | PASS |
| `watch-file-rejection-preserves-watches` | PASS |
| `cli-more-feeds` | PASS |
| `cli-repository-workflow` | PASS |
| `cli-demo-local` | PASS |
| `cli-shipped-mapping-local` | PASS |
| `api-contract` | PASS |
| `container-build-stage` | PASS |
| `database-persistence` | PASS |
| `port-only-startup` | PASS |
| `azure-files-dotfile-locking` | PASS |
| `single-replica-durable-data` | PASS |
| `scheduled-scan-consent` | PASS |
| `scheduled-scan-deduplication` | PASS |
| `scheduled-run-status` | PASS |
| `scheduled-notification-destination` | PASS |

The landing page, legal pages, README, demo documentation, and CLI help were cross-checked against the manifest. No unlisted user-facing capability claim was found.

## Clean local gates

| Command | Result |
| --- | --- |
| `npm ci` | PASS — 60 packages; 0 audit vulnerabilities |
| `npm test` | PASS — 10/10 Vitest tests |
| `npm run typecheck` | PASS |
| `npm run lint` | PASS |
| `cargo fmt --all -- --check` | **FAIL — V19-1** |
| `cargo clippy --locked --all-targets -- -D warnings` | PASS |
| `cargo test --locked` | PASS — 28/28 Rust tests |
| `npm run build` | PASS — `dist/` produced |
| `cargo build --release --locked` | PASS |
| `npm run test:browser` | PASS — 69 passed, 3 deliberate environment skips |
| `npm run test:a11y` | PASS — 20/20 |
| live `npm run test:a11y` | PASS — 20/20 |

The three full-suite skips are intentional guards: the destructive hosted rate-limit probe is isolated, and one shared backend burst runs only in the desktop project. That live rate-limit probe was run independently below.

No Docker, Podman, or Buildah executable is installed in this worker. The exact Vite build and locked native release build passed; the Dockerfile contract claim passed; and the already-built live container reports the exact candidate identity. A local OCI image build was therefore not repeatable in this environment.

## End-to-end behavior — PASS

- Demo acknowledgement changed the pending count; **Reset demo** restored the seed. A fresh demo made no API call and used only its demo storage namespace.
- A live private workspace rejected `http://127.0.0.1/private`, then saved and edited a public watch.
- Schedule boundary `14` was rejected with the allowed `15`–`10,080` range. A 60-minute schedule saved and displayed last/next-run state. A private webhook was rejected without losing the valid schedule. Stopping the schedule cleared it.
- A live feed redirect produced a readable recovery message. Removing the watch left empty server-side watch and action lists.
- The local release binary served 24 concurrent authenticated reads successfully. Six concurrent creates with one existing watch produced exactly two `201` and four `409` responses, preserving the three-watch limit.
- A workspace token and three watches survived SIGTERM, release-binary restart, and reconnect to the same SQLite file. Both shutdowns logged graceful drain completion.

Evidence: `qa-artifacts/verification-19-live-workspace-recovery.png`.

## CLI package/consumer check — PASS

`cargo package --allow-dirty --no-verify` produced 20 files, 251.1 KiB unpacked and 62.9 KiB compressed. The `.crate` was unpacked and installed into a fresh temporary Cargo root.

The installed binary:

- printed its public help and bundled two-card demo;
- scanned the shipped local feed into action `464f8e41f622`;
- returned **“No new matching notices found”** on the second scan;
- acknowledged the action and updated both Markdown and JSON state; and
- rejected a missing config with exit 1 and **“scan failed: could not read missing.json.”**

## Live identity, privacy, security, and delivery — PASS

- `GET /health` returned `200 {"build":"10375ffd1868aa787b7b6d0fa84de02123ad7e06","ok":true}`. The live footer carries the same SHA.
- SHA-256 hashes for live JS and CSS exactly match the local production output: JS `2306f865…b5b7e6ff`; CSS `99cf51d3…c0eaadf2`.
- Fresh `/demo` requested only its document and same-origin hashed JS/CSS. It made no `/api` request. Fresh privacy and terms pages also made no API request or workspace storage entry.
- Cold home requests were same-origin only: document, JS, CSS, hero, workspace creation, and workspace reads. No analytics, advertising, CDN font, AI, billing, or other third-party runtime request occurred.
- Normal home, demo, privacy, and terms loads had zero console/page errors. Chrome logged only the expected failed-resource messages for deliberately exercised `400` and `404` responses.
- CSP restricts all resource types to self and sends `frame-ancestors 'none'`. Responses also include HSTS, `nosniff`, strict-origin referrer policy, and a restrictive permissions policy.
- Documents and health use `Cache-Control: no-cache`; API responses use `no-store, private`; hashed assets use one-year immutable caching; the hero uses a seven-day public cache.
- Home, demo, privacy, and terms return 200 with route-specific titles, descriptions, canonicals, one H1, and one main landmark. The styled missing route returns 404. Every internal navigation link and both sample vendor links returned 200.
- `robots.txt` and `sitemap.xml` are present. There is no service worker or offline/PWA claim, so PWA update/offline checks are not applicable.
- There is no sign-in, AI action, or paid unlock. Entra, AI-gateway, and billing checks are not applicable. Import/export is present; the brief explicitly excludes LLM summaries.

## Rate limiting — PASS

The limiter wraps the complete `/api` router; `/health` is intentionally exempt. After refill, 40 same-client requests reached authorization and the 41st returned `429`, `Retry-After: 1`, and **“Too many requests. Try again in 1 second(s).”** The observed allowance is a 40-request burst with 20 requests/second refill. A separate 100-request burst that crossed one refill boundary produced 60 × `401` and 40 × `429`; every limited response used `Retry-After: 1` and `Cache-Control: no-store, private`. Health stayed 200.

## Accessibility and responsive behavior — PASS

- Axe WCAG 2 A/AA found zero violations on home, 390 px demo, privacy, terms, and the 404; therefore zero serious/critical findings.
- `/opt/fleet/lib/verify-url.sh` passed `/demo`: 200, title/lang/one-H1/main/alt/control-name checks clean, no console errors, 540 ms load. Evidence: `qa-artifacts/verification-19-verify-url/verify.json`.
- The first Tab focused **Skip to content** with a visible `3px dashed rgb(40, 79, 122)` ring; Enter moved focus to `main`. Keyboard Space acknowledged the sample action and kept useful focus.
- At 390 × 844 there was no horizontal overflow, and the first sample title, owner, dependency, and check were all inside the first viewport. Visible touch controls met the 44 px baseline. At the 195 px 200%-text equivalent, content width equaled viewport width.
- Reduced-motion emulation left no active CSS transition or animation.

## Performance — PASS

The production build emits 22,558 bytes raw / 7.65 kB gzip JavaScript and 9,025 bytes raw / 2.79 kB gzip CSS. The hero WebP is 58,974 bytes. These pass the 200 kB JS, 50 kB CSS, and 300 kB hero budgets.

Fresh mobile Lighthouse scored **96 performance, 100 accessibility, 100 best practices, and 100 SEO**. FCP was 1.0 s, LCP 1.3 s, TBT 230 ms, CLS 0, and total transfer 91 KiB. Evidence: `qa-artifacts/verification-19-lighthouse.json`.

## Defects by severity

| Severity | Findings |
| --- | --- |
| Critical | None |
| High | None |
| Medium | V19-1: `cargo fmt --all -- --check` exits 1 |
| Low | None |

## Required next step

Apply `cargo fmt --all`, review and commit only its mechanical `src/main.rs` diff, then repeat the local quality gates. Functional redeployment is not otherwise indicated because the live candidate identity and assets already match.
