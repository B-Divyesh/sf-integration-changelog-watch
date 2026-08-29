# Independent verification 6 — FAIL

Verified independently on 2026-08-29 UTC.

- Candidate commit: `5e14f6f44e25eb3c733c1708522be0ca2197cade`
- Live URL: `https://integration-changelog-watch.sociobot.in`
- Work order: `integration-changelog-watch-verify-6`
- Result: **FAIL — do not release**

The candidate source is healthy locally and the exact candidate image and static assets are live. The release nevertheless fails under ordinary production scale-out. The deployed service can run three replicas without shared storage while workspace tokens and records live in replica-local SQLite. Once the required load/rate check caused three replicas to serve, fresh workspaces became unusable across requests. The same topology also multiplies the intended per-client request allowance. The claims inventory remains incomplete, and the backend has no graceful shutdown path.

No product code or deployment setting was changed during verification.

## Mandatory first gates

### First read — PASS

A cold live visit answers the three required questions on the first screen, including at 390 px:

- What: **“Turn vendor changes into owned actions.”**
- For whom: **“For engineers who maintain payment, auth, analytics, or messaging integrations.”**
- First action: **“Try it with sample data”**, beside **“See matched notices, owners, and checks.”**

One click opens `/demo`. Its first render contains three watches, two realistic action cards, owners, check commands, and the persistent **“Demo — sample data, nothing is saved”** banner with **Reset demo** and **Start for real**.

### Declared claims — PASS locally after clean install

`.factory/claims.json` exists with nine entries. As required, every exact command was attempted before other repository work. Before dependency installation the seven commands beginning with `npm run build` stopped at `vite: not found`; both standalone Rust claim commands passed. After the documented `npm ci` prerequisite, every exact command passed from the clean candidate:

| Claim | Result |
| --- | --- |
| `sample-action-cards` | PASS, 2/2 Playwright projects |
| `csv-export` | PASS, 2/2 Playwright projects |
| `demo-local` | PASS, 2/2 Playwright projects |
| `workspace-boundary` | PASS locally, 2/2 Playwright projects |
| `redirecting-feeds` | PASS, 1/1 Rust test |
| `requested-scans` | PASS, 2/2 Playwright projects |
| `cli-repository-workflow` | PASS, 2/2 Playwright projects |
| `cli-demo-local` | PASS, 2/2 Playwright projects |
| `database-persistence` | PASS, 1/1 Rust test |

The production `workspace-boundary` claim later failed in both desktop and mobile after the live service scaled out. The fresh-token consistency regression also failed. This is live evidence, not a local claim-command failure.

## Release-blocking findings

### Major — replica-local SQLite makes live workspaces fail after scale-out

Read-only inspection of the live Container App returned:

```json
{
  "image": "sociobotregistry.azurecr.io/sf-integration-changelog-watch:5e14f6f44e25",
  "minReplicas": 1,
  "maxReplicas": 3,
  "volumes": null,
  "volumeMounts": null
}
```

After the live rate probe, Azure reported revision `sf-integration-changelog-watch--0000015` as `RunningAtMaxScale` with **three running replicas**. The backend uses a local SQLite URL and hashes workspace tokens into that database; there is no shared store.

Fresh observable failures at three replicas:

- One newly created token was used for 48 sequential authenticated reads: **16 × 200, 32 × 401**. The 401 body was **“This workspace token is not active on the server.”**
- Five independent cold visits to `/` each received `POST /api/workspaces` 201, one dashboard read 200, and the other dashboard read 401. Every visit displayed **“Your private workspace could not load. Check your connection, then reload.”** and logged a browser console resource error.
- Targeted live Playwright for `@claim:workspace-boundary|fresh workspace token`: **3 failed, 1 skipped**. Both desktop and mobile workspace claims failed; the 24-read consistency check received **8 × 200 and 16 × 401** instead of 24 × 200.

At low load, before scale-out, the full live browser suite happened to pass (39 passed, one deliberate duplicate burst skipped). That result is not stable: the required load probe exposed the deployment's incompatible state model. This breaks the core hosted job, makes acknowledgement/history availability depend on routing, and provides no revision-replacement durability.

Required repair: use a shared durable database, or enforce exactly one replica and attach a verified durable `/data` volume. Re-run the cold-page, 48-read, browser-claim, restart-persistence, and scale tests on the actual deployment.

### Major — the intended per-client rate allowance is multiplied/fragmented in production

The source implements a 40-request bucket refilled at 20 requests/second and keys it in process memory. Local deterministic coverage passes 40 allowed / 40 limited for one client.

Fresh live observations from one browser client:

- **120 requests in 811 ms:** 120 × 401, **0 × 429**. This already exceeded the intended burst allowance without enforcement.
- **400 requests in 3,174 ms:** 289 × 401, 111 × 429. All 111 limited responses included `Retry-After: 1`.

Thus a 429 with `Retry-After` eventually exists, but the live allowance observed was 289 accepted requests in the larger burst and no rejection at all in the 120-request burst. The limiter is local to each replica and does not enforce the documented-in-code allowance for one client across the deployed service. A shared/edge limiter or a single-replica deployment is required.

### Major — public claims remain outside `.factory/claims.json`

The supplied claims contract makes any unlisted or under-tested public promise release-blocking. The manifest does not cover all statements in the shipped documentation and UI:

- `.factory/demo.md` says the demo makes **no API call** and that Reset/Start for real perform specific sandbox transitions. `demo-local` only asserts that requests are same-origin; it would pass a same-origin `/api` call, and it does not exercise either transition.
- README says the shipped `scan --config examples/watches.json` workflow works **without a network request**. `cli-demo-local` covers the separate `demo` command. `cli-repository-workflow` creates its own temporary mapping and does not record network attempts.
- README says the container **starts with only `PORT` required**. This was manually verified, but it has no claim entry and no `@claim:` test.
- The landing page and README say the product does not access private portals, alter code, or detect undocumented changes. The private-address portion is covered; the remaining scope promises are not listed or asserted.

Required repair: add exact claim entries and observable sandbox tests, or remove/reword the public promises.

## Other findings

### Moderate — server termination is not graceful

The locked release binary started with only `PORT` and `PATH`, logged that the database configuration was defaulted, and served `/health`. A real file-backed workspace survived process restart. However, sending SIGTERM ended the server non-zero with no shutdown log. Source awaits `axum::serve(...).await.expect("serve")` without `with_graceful_shutdown` or a signal handler. This misses the backend contract and risks dropping in-flight scans during deployments.

### Minor — footer version identity is inconsistent

SPA routes show `v2`; the styled 404 shows `v3`; the package is `1.0.0`; neither footer exposes the candidate/build identity. The standard site skeleton requires a consistent version/build id.

## Functional and boundary evidence

Before production scaled out, a fresh live workspace completed the useful workflow:

1. workspace creation 201 with a 64-character token;
2. public shipped feed watch creation 201;
3. scan 200 with one action, **“Webhook delivery format changes”**;
4. acknowledgement 200 with `acknowledged: true`;
5. rescan 200 with zero duplicates;
6. watch deletion 204.

Recovery and validation behaved correctly on that replica:

- empty vendor: 400 with the missing-fields instruction;
- 121-character vendor: 400 with the 120-character limit;
- loopback feed: 400 with the public-network instruction;
- local concurrent watch-limit test: exactly three creates succeed and seven return 409;
- local SQLite record survived a real process stop/start.

These code paths pass, but they do not overcome the production state split.

## Local quality gates

| Check | Result |
| --- | --- |
| `npm ci` | PASS; 60 packages, 0 audit findings |
| `npm test` | PASS, 3/3 |
| `npm run typecheck` | PASS |
| `npm run lint` | PASS |
| `npm run build` | PASS; `dist/` generated |
| `cargo fmt --all -- --check` | PASS |
| `cargo test --locked` | PASS, 11/11 |
| `cargo clippy --all-targets --locked -- -D warnings` | PASS |
| `cargo build --release --locked` / `npm run test:container` | PASS |
| `npm run test:browser` | PASS locally, 39 passed / 1 intentional skip |
| initial full live browser run at one replica | PASS, 39 passed / 1 intentional skip |
| live workspace tests after scale-out | FAIL, 3 failed / 1 skipped |
| `npm run test:a11y` | PASS, 12/12 |
| `/opt/fleet/lib/verify-url.sh` | PASS; 682 ms, no errors, title/lang/one h1/main/alt/buttons valid |
| `cargo package --locked --allow-dirty --no-verify` | PASS; 84 files |

Docker, Podman, and Buildah are unavailable in this worker, so the Dockerfile itself could not be executed. Its exact frontend and locked release-backend build steps passed, and inspection confirms current-stable Rust, `BUILD_SHA`, a non-root runtime user, port 8080, and no `.git` dependency.

## Clean CLI consumer

The produced `.crate` was extracted into a clean source directory and installed into a separate Cargo root. The installed executable passed `--help` and `demo`. With the packaged example, the first scan created `464f8e41f622.md`, the second scan created no duplicate, and `ack` persisted `acknowledged: true`.

## Accessibility, privacy, routing, headers, and performance

- Fresh axe checks at desktop and 390 px found **zero serious/critical violations**.
- `lang=en`, route-specific titles, one h1, one main, image alt text, ordered headings, skip link, and the styled 404 are present.
- Keyboard smoke: first Tab exposes a `141.5 × 44 px` skip link with a 3 px dashed indigo focus ring; Enter/Space flows and acknowledgement focus pass.
- At 390 px there is no horizontal overflow; 195 px reflow coverage also passes. The sample action is visible in the first mobile viewport.
- With reduced motion, action transition and animation durations are both `0s`.
- Direct `/demo` requested only its document and same-origin hashed JS, CSS, and hero image. There were no third-party, analytics, advertising, remote-font, API, console, or page errors in the demo audit.
- Headers include restrictive CSP with header-only `frame-ancestors 'none'`, HSTS, `nosniff`, strict-origin referrer policy, and restrictive Permissions-Policy.
- Cache behavior: HTML `no-cache`; hashed JS/CSS one-year immutable; hero/icon assets one week; API `no-store, private`.
- Candidate/live byte parity is exact for `index.html`, hashed JS, hashed CSS, and the hero image. `/health` returns the full candidate SHA.
- Production sizes: JS 13,699 bytes raw / 5,350 gzip; CSS 7,813 bytes raw / 2,557 gzip; hero WebP 58,974 bytes.
- Fresh Lighthouse mobile `/demo`: Performance 91, Accessibility 100, Best Practices 100, SEO 100; FCP 1.3 s, LCP 1.6 s, TBT 370 ms, CLS 0, total transfer 81 KiB.
- `/`, `/demo`, `/privacy`, `/terms`, metadata assets, robots, and sitemap return 200. The styled unknown route returns 404.
- Visual inspection of fresh desktop and 390 px screenshots found no clipping, overlap, or image artifact.

The product is not a PWA and makes no offline-reload claim. It has no sign-in, AI runtime feature, paid tier, or unlock endpoint, so service-worker, Entra, AI gateway, and billing checks are not applicable.

## Deployment identity

`GET /health` returned:

```json
{"build":"5e14f6f44e25eb3c733c1708522be0ca2197cade","ok":true}
```

Live and local SHA-256 values matched:

| File | SHA-256 |
| --- | --- |
| `index.html` | `c465410173d9a42140405ca0804a9d609c3ba3dc0a5e58c699bbfd0c98c7470b` |
| `assets/index-r7gIByL8.js` | `3e59e8d3b35631b2a8f5f6c118f2a38893610fe61f354f245394de2c8c8bd77d` |
| `assets/index-DghLgAOj.css` | `3ded19a4257894329dc748758320156c7be138b88bf03871cd260b0ee7e9c149` |
| `paper-cut-hero.webp` | `fb0d415bec60a017de28d29ee05795fbf156fc3ab069dab80d3bd7dce49a252b` |

## Release decision

**FAIL.** The exact candidate is deployed and strong locally, but the real hosted product loses workspace affinity and rate-limit guarantees as soon as its configured scale-out occurs. The production state topology, claim inventory, and graceful shutdown must be repaired before release.
