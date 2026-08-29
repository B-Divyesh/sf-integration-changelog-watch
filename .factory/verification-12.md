# Independent verification 12 — FAIL

Verified independently on 2026-08-29 UTC.

- Candidate: `1d82c9140dcf6937295d57fc96d47c087aa0775a`
- Live URL: <https://integration-changelog-watch.sociobot.in>
- Work order: `integration-changelog-watch-verify-12`
- Result: **FAIL — the candidate is not release-ready.**

The exact candidate is deployed and its build, claims, core scan workflow,
privacy boundary, rate limit, and local quality gates pass. Release is blocked
by a fresh destructive recovery failure: a server-rejected watch-file import
deletes the workspace's existing watches, reports that the import failed, and
does not restore the deleted records.

No product code was changed during this verification.

## Mandatory first gates

### Claims — PASS

The clean checkout started at the requested SHA with no worktree changes.
`.factory/claims.json` exists and contains 20 entries. After `npm ci` (60
packages, zero audit findings), every literal `test` command was run serially
before broader QA.

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
| `container-build-stage` | PASS |
| `database-persistence` | PASS |
| `port-only-startup` | PASS |
| `single-replica-durable-data` | PASS |

The repaired `container-build-stage` command uses Vitest's supported,
anchored `--testNamePattern` and selected one passing test. No unlisted public
claim was found in the landing page or README. The destructive import path
below is not covered by the demo-only happy-path portability claim.

### Cold first read — PASS

At both 1440×900 and 390×844, the cold first screen plainly answers:

- What: **“Turn vendor changes into assigned action cards.”**
- Who: **“For engineers who maintain payment, auth, analytics, or messaging
  integrations.”**
- First action: **Try it with sample data**, beside **“See matched notices,
  owners, versions, and checks.”**

The button is inside the first mobile viewport. One click opens `/demo`, whose
first view contains a realistic Stripe action, its matched keyword, owner,
dependency version, and local check. The persistent banner says **“Demo —
sample data, nothing is saved”** and offers **Reset demo** and **Start a private
workspace**.

## Candidate and deployment identity

- `git rev-parse HEAD` returned the requested full SHA.
- `GET /health` returned `200` and
  `{"build":"1d82c9140dcf6937295d57fc96d47c087aa0775a","ok":true}`.
- The live HTML build marker and SPA/404 footers report the same full SHA.
- Azure reports image
  `sociobotregistry.azurecr.io/sf-integration-changelog-watch:1d82c9140dcf`,
  revision `sf-integration-changelog-watch--0000040`, `Running`, 100% traffic.
- Live JS, CSS, and hero bytes match the clean local production output by
  SHA-256. The only HTML difference is the expected runtime replacement of
  `{{BUILD_ID}}` with the full SHA.

This fresh evidence resolves any earlier deployment-only uncertainty: the
candidate under review is live.

## Local quality gates

| Check | Result |
| --- | --- |
| `npm ci` | PASS; 60 packages, 0 vulnerabilities |
| `npm test` | PASS; 6/6 |
| `npm run typecheck` | PASS |
| `npm run lint` | PASS |
| `npm run build` | PASS; produced `dist/` |
| `cargo fmt --all -- --check` | PASS |
| `cargo test --locked` | PASS; 18/18 |
| `cargo clippy --locked --all-targets -- -D warnings` | PASS |
| `cargo build --release --locked` | PASS |
| `npm run test:browser` | PASS; 59 passed, 1 intentional project skip |
| Live `npm run test:browser` | PASS; 59 passed, 1 intentional project skip |
| `npm run test:a11y` | PASS locally and live; 16/16 each |

Docker, Podman, and Buildah are unavailable in this verifier image, so a fresh
Docker invocation was not possible. The exact frontend build and locked
optimized Rust build used by the Dockerfile passed; its contract assertions
passed; and the candidate-tagged production image is running live.

## End-to-end product and recovery evidence

### Core workflow — PASS

A fresh live workspace completed the useful job with a real public Stripe Node
releases feed:

1. workspace creation returned `201` and a 64-character token;
2. invalid `not-a-url` input returned `400` with the complete-public-URL next
   step;
3. a Stripe Node Atom watch saved with owner, dependency version, keyword, and
   check command;
4. an explicit scan returned `200`, no feed failures, and seven new action
   cards;
5. the first readable card linked to the vendor notice and retained its owner,
   version, match, and command;
6. acknowledgement returned `200` with `acknowledged: true`;
7. a second scan returned zero new actions, proving deduplication;
8. editing the keywords and deleting the temporary watch both succeeded.

The 120-character vendor boundary returned `201`; 121 characters returned
`400` with the exact limit. Blank required input and a loopback source returned
readable `400` responses. Three hosted watches saved; the fourth returned the
documented `409`. A browser-level invalid-URL attempt displayed a recovery
message, and a subsequent valid save and removal succeeded.

### Destructive rejected import — FAIL

Fresh live reproduction:

1. Save a valid watch named **Keep me** in a new private workspace (`201`).
2. Choose **Import watch file** and supply a schema-valid file whose one source
   is `http://127.0.0.1/private`.
3. The preview accepts it. Press **Import 1 watch**.
4. Network order is `DELETE /api/watches/59` → `204`, then
   `POST /api/watches` → `400` because the source is blocked.
5. The page says **“The watch file was not imported…”**, but a direct server
   read returns an empty watch list.
6. The old watch and rejected preview remain misleadingly visible until reload.
   Reload then shows `0/3` and **“Nothing is watched yet.”**

Screenshots: `import-before.png`, `import-failed-stale.png`, and
`import-after-reload.png` in `.factory/qa-artifacts/verification-12/`.

The implementation deletes all current watches before attempting any imported
create and has no transaction or rollback. Any server-side rejection that the
browser's shape-only preview cannot detect can irreversibly erase the existing
mapping. This violates invalid-input recovery and the requirement that
destructive actions be reversible or specifically confirmed.

## Backend, concurrency, persistence, and rate limit

- Twenty-four parallel authenticated reads from one fresh token all returned
  `200`.
- Anonymous access returned `401`; a second workspace could not read the first
  workspace's records.
- Local tests prove atomic concurrent enforcement of the three-watch limit,
  hashed token storage, SQLite reconnect persistence, graceful SIGTERM, and
  restoration of token validity and the rate bucket after the modeled restart.
- Azure reports `minReplicas: 1`, `maxReplicas: 1`, an Azure Files volume named
  `workspace-data`, and its `/data` mount on the only container. This is the
  required persistence boundary for the local SQLite design.
- A fresh single-client burst sent 80 simultaneous unauthenticated API reads in
  369 ms: exactly 40 returned `401` and the next 40 returned `429`. Every
  limited response carried `Retry-After: 1` and **“Too many requests. Try again
  in 1 second(s).”**

Observed allowance: **40-request burst with a 20 requests/second refill**. The
limiter middleware wraps all API routes, including health, and keys the first
forwarded client hop.

## Clean CLI consumer

`cargo package --locked --allow-dirty --no-verify` passed with 18 files:
213.4 KiB unpacked and 57,209 bytes compressed. The crate was extracted and
installed with a new `CARGO_HOME` and install root. The installed binary passed
`--help` and `demo`, scanned the packaged local feed into action
`464f8e41f622`, created no duplicate on its second scan, and persisted the
acknowledgement in both state and Markdown.

## Privacy, accessibility, routes, and headers

- A fresh live `/demo` acknowledgement and reset made exactly three requests:
  document, same-origin hashed JS, and same-origin hashed CSS. It made no API,
  analytics, advertising, remote-font, or third-party request and raised no
  console or page error.
- Cold `/` and normal `/demo` loads had no console/page/request errors. The
  intentionally rejected `400` input produced only Chromium's expected failed
  resource console line, not an application exception.
- Demo storage never wrote the real workspace key. Legal pages made no API call
  and did not create a workspace.
- The factory URL verifier passed title, `lang`, one `h1`, one `main`, alt text,
  named buttons, load, and console checks. Evidence is in
  `.factory/qa-artifacts/verification-12/`.
- Axe found zero WCAG A/AA violations in both desktop and mobile projects.
- At 390 px, there was no horizontal overflow and no visible interactive target
  below 44×44 CSS px. The 195 px reflow test passed.
- The first keyboard Tab exposes the skip link with a 3 px dashed indigo focus
  ring; Enter moves to `main`; Space acknowledges a demo action and restores
  focus to its card. Reduced-motion mode computes zero-duration transitions and
  no transforms.
- Root, demo, privacy, terms, metadata, robots, sitemap, and both sample vendor
  links resolve. The styled missing route correctly returns `404`.
- Responses include HSTS, `nosniff`, strict-origin referrer policy, restrictive
  Permissions Policy, and a header CSP with `frame-ancestors 'none'`. HTML uses
  `no-cache`; APIs use `no-store, private`; hashed JS/CSS use one-year immutable
  caching; the hero uses a seven-day cache.

Two non-blocking-to-Axe structural defects remain and are listed below.

## Performance and bundle budgets

Fresh mobile Lighthouse on `/demo`:

| Category/metric | Result |
| --- | ---: |
| Performance | 100 |
| Accessibility | 98 |
| Best practices | 100 |
| SEO | 100 |
| FCP | 1.0 s |
| LCP | 1.0 s |
| Total blocking time | 20 ms |
| CLS | 0 |
| Initial transfer | 30 KiB |

Production assets are within budget: JS 19,463 bytes raw / 6,820 gzip; CSS
8,801 / 2,745; hero WebP 58,974 bytes; social card 163,227 bytes. No remote
fonts or scripts load.

The product has no service worker, offline claim, sign-in, runtime AI,
payment, or paid unlock. PWA update, Entra, AI-gateway, and billing checks are
not applicable. The brief explicitly makes LLM summaries a non-goal.

## Defects by severity

### High — release-blocking

1. **A rejected real-workspace import deletes all existing watches.** The
   importer deletes current records before creating replacements. A blocked
   loopback source then fails creation, while the UI says the import did not
   happen and temporarily renders stale records. There is no rollback, undo,
   or warning that failed validation can erase the workspace. Validate the
   complete import server-side before mutation, or replace it atomically in one
   transaction; add a real-workspace rejection/rollback claim test.

### Medium

1. **The demo heading outline skips level 2.** Its sequence is `h1` → `h3`
   (`Action cards`, each card title, and `Watched feeds`). Lighthouse's
   `heading-order` audit fails, producing the 98 accessibility score. Use an
   `h2` for the section headings and nest card headings consistently.
2. **Browser Back loses route focus after returning to home.** Clicking Demo
   correctly focuses the demo `h1`, but Back to `/` leaves focus on `<body>`.
   The asynchronous real-workspace hydration replaces the already-focused
   heading. Preserve or restore focus after hydration so history navigation
   meets the documented screen-reader routing contract.

## Release decision

**FAIL.** The candidate is deployed, identity-matched, fast, private in demo
mode, correctly rate-limited, and passes every declared claim and automated
quality gate. It must not ship while a rejected import silently destroys
existing workspace records. The heading hierarchy and history-focus defects
should be repaired in the same release loop.
