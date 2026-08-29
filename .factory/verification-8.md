# Independent verification 8 — FAIL

Verified independently on 2026-08-29 UTC.

- Candidate commit: `4eee3434cfacac5ac1cea17ec9b7c149a403f7ec`
- Live URL: `https://integration-changelog-watch.sociobot.in`
- Work order: `integration-changelog-watch-verify-8`
- Result: **FAIL — do not release**

The candidate code and static assets are live, and the product works on one local process. The production deployment has reverted to the exact unsafe topology found in verification 7: three local-SQLite replicas, no durable volume, and three independent rate buckets. Fresh visitors consistently receive cross-replica 401 responses, and a single client receives 120 requests before limiting instead of the configured 40.

No product code was changed during this verification. Browser evidence is in `.factory/qa-artifacts/verification-8/`.

## Mandatory first gates

### First read — PASS

A cold desktop and true 390 × 844 mobile visit answered all three required questions in the first screen:

- What: **“Turn vendor changes into owned actions.”**
- For whom: **“For engineers who maintain payment, auth, analytics, or messaging integrations.”**
- First action: **“Try it with sample data”**, beside **“See matched notices, owners, versions, and checks.”**

At 390 px the action was fully visible at `y=373.94`, measured 241.67 × 46 px, and opened `/demo` in one click. The demo displayed the persistent sample-data banner, three watches, two action cards, owners, affected versions, commands, reset, and start-for-real controls.

Evidence: `first-read-desktop.png`, `first-read-mobile390.png`, `demo-mobile390.png`, and `mobile-direct-evidence.json`.

### Declared claims — exact commands PASS after the required install

`.factory/claims.json` exists with 13 complete entries. In the literal pre-install run, the nine frontend commands stopped at `vite: not found`; the four Rust commands passed. After the repository's documented `npm ci`, all 13 exact claim commands passed. Each Playwright claim ran in desktop and mobile Chromium, and each claim has one selected test.

| Claim | Installed clean-clone result |
| --- | --- |
| `sample-action-cards` | PASS, 2/2 |
| `csv-export` | PASS, 2/2 |
| `demo-local` | PASS, 2/2 |
| `workspace-boundary` | PASS locally, 2/2 |
| `redirecting-feeds` | PASS, 1/1 |
| `requested-scans` | PASS, 2/2 |
| `cli-repository-workflow` | PASS, 2/2 |
| `cli-demo-local` | PASS, 2/2 |
| `database-persistence` | PASS, 1/1 |
| `demo-isolation-transitions` | PASS, 2/2 |
| `cli-shipped-mapping-local` | PASS, 2/2 |
| `port-only-startup` | PASS, 1/1 |
| `single-replica-durable-data` | PASS only as a static-template test, 1/1; **false in production** |

The `single-replica-durable-data` test inspects the versioned deployment template. It does not verify the running deployment. Fresh production evidence below contradicts the claim, making that claim release-blocking despite its local test result.

## Release-blocking findings

### Critical — production splits and will lose workspace state

Read-only Azure inspection of the live candidate returned:

```json
{
  "image": "sociobotregistry.azurecr.io/sf-integration-changelog-watch:4eee3434cfac",
  "revision": "sf-integration-changelog-watch--0000024",
  "minReplicas": 1,
  "maxReplicas": 3,
  "volumes": null,
  "volumeMounts": null
}
```

The active revision had three running replicas. This contradicts `deploy/containerapp.yaml`, which requires exactly one replica, an Azure Files volume, and a `/data` mount.

A fresh live end-to-end attempt produced:

1. workspace create: 201;
2. add GitHub changelog watch: 201;
3. immediate scan: 401, **“This workspace token is not active on the server.”**;
4. immediate actions read: 401;
5. next scan: 200 and 10 actions;
6. 24 parallel authenticated reads: **12 × 200 and 12 × 401**.

Six additional cold browser contexts reproduced the same split every time: workspace create 201, watches 401, actions 200. Each page displayed **“Your private workspace could not load. Check your connection, then reload.”** and logged a failed-resource 401 console error.

A final full live Playwright rerun while all three replicas were active reported **3 failed, 44 passed, 1 skipped**. The `@claim:workspace-boundary` case failed in both desktop and mobile projects, and the authenticated 24-read test received 16 × 401 and 8 × 200. This is a live failure of claimed behavior, not only an infrastructure observation.

Each replica owns image-local SQLite. State is inconsistent while several replicas run and disappears when the owning replica scales in. This breaks the real add → scan → review → acknowledge job and the privacy/durability promises.

### Major — the live request allowance is tripled

The source configures one client bucket with a 40-request burst and 20 requests/second refill. One local process enforced it exactly: an 80-request burst returned **40 × 401 and 40 × 429**, and every 429 had `Retry-After: 1`.

After live buckets had refilled, one client sent 150 simultaneous requests in 542 ms. Production returned:

```json
{
  "401": 120,
  "429": 30,
  "Retry-After": "1"
}
```

The observed live allowance is therefore **120**, not 40. Every 429 did include the required `Retry-After` header, but three replica-local buckets multiply the allowance. This independently fails the mandatory server-side rate-limit contract.

## Other findings

### Moderate — researched hosted team/paid scope is absent

The researched brief says that many watches, hosted history, and team ownership support a subscription. The product instead exposes a free private three-watch workspace and explicitly says it has no accounts, team workspace, unlimited tier, or paid plan. This is honestly disclosed and the local CLI supports more watches, but it remains an unimplemented acceptance-contract capability rather than an impossible requirement.

### Minor — CLI acknowledgement leaves its Markdown card stale

The installed CLI's `ack` command correctly changes `.integration-changelog-watch/state.json` to `"acknowledged": true`, but the corresponding Markdown action card still says **“Status: Needs acknowledgement.”** The repository state is correct, but the human-facing card can direct an engineer to repeat finished work.

### Minor — Apple touch icon is not the required square size

`frontend/public/apple-touch-icon.png` is 180 × 120, not a 180 × 180 touch icon. The favicon and social card dimensions are correct.

## Local product and backend evidence

The locked release binary completed the smallest useful job on one process:

- created a 64-character private workspace token;
- added a real GitHub changelog feed with owner, affected version, and check command;
- scanned 10 matching notices with item permalinks;
- acknowledged an action and preserved `acknowledged: true`;
- rescanned with zero duplicates;
- edited and removed watches;
- rejected anonymous access, empty fields, a 121-character vendor, credentialed URLs, and loopback URLs with useful messages;
- accepted exactly three of ten concurrent watch creates and returned seven 409 responses;
- preserved two watches, ten actions, one acknowledgement, and the affected version across a graceful process restart.

The host binary cannot start with only `PORT` because the host has no `/data`; this is expected by the README. No Docker-compatible runtime is installed in this worker, so the complete image start could not be executed. The Dockerfile was inspected: it creates and owns `/data`, uses current-stable Rust, uses a non-root runtime user, accepts `BUILD_SHA`, and does not depend on `.git`.

## Packaged CLI consumer

`cargo package --locked --allow-dirty --no-verify` produced a 92-file, 6.5 MiB compressed crate. It was extracted and installed into a separate clean Cargo root. The installed binary passed `--help` and `demo`; the shipped-format local mapping created one Markdown card, a second scan created no duplicate, and `ack` persisted the acknowledgement.

## Quality gates

| Check | Result |
| --- | --- |
| `npm ci` | PASS; 60 packages, 0 audit findings |
| all 13 exact claim commands | PASS after install |
| `npm test` | PASS, 4/4 |
| `npm run typecheck` | PASS |
| `npm run lint` | PASS |
| `npm run build` | PASS; `dist/` produced |
| `cargo fmt --all -- --check` | PASS |
| `cargo test --locked` | PASS, 17/17 |
| `cargo clippy --all-targets --locked -- -D warnings` | PASS |
| `cargo build --release --locked` | PASS; fresh release build in 4m54s |
| `npm run test:container` | PASS (locked release build) |
| local `npm run test:browser` | PASS, 47 passed / 1 intentional skip |
| live `npm run test:browser` before scale-out probe | PASS, 47 passed / 1 intentional skip |
| final live browser rerun with 3 active replicas | **FAIL, 3 failed / 44 passed / 1 skipped** |

The first live browser run passed while the service had not yet exposed the scale-out fault. The later real feed/concurrency probe scaled the revision to three replicas; the final suite then failed both live `@claim:workspace-boundary` projects and the authenticated-read consistency test.

## Accessibility, privacy, routing, headers, and performance

- Desktop and 390 px layouts have no horizontal overflow. The 195 px/200%-reflow check also has no horizontal overflow.
- First Tab reveals a 141.52 × 44 px skip link with a 3 px dashed indigo outline; Enter focuses `main`. Keyboard acknowledgement and focus restoration pass in the full browser suite.
- Playwright Axe WCAG A/AA found zero serious/critical issues on desktop and mobile. The full accessibility matrix passed 14/14.
- Reduced-motion emulation produces `0s` transition/animation duration and `scroll-behavior: auto`.
- Direct `/demo`, including acknowledgement and reset, requested only the same-origin document, hashed JS/CSS, and hero image. It made no API, analytics, advertising, remote-font, or third-party request and logged no console/page error.
- The real root page does log 401 console errors because of the deployment split described above.
- Header-delivered CSP includes `frame-ancestors 'none'`; HSTS, `nosniff`, strict-origin referrer policy, and restrictive permissions policy are present. API responses are `no-store, private`.
- HTML is `no-cache`; hashed JS/CSS are one-year immutable; local images have one-week caching. All expected routes and same-origin assets return 200, the styled unknown route returns 404, and the Stripe/Auth0 demo notice links return 200.
- Production assets: JS 14,416 bytes raw / 5.54 KiB gzip; CSS 7,942 bytes raw / 2.56 KiB gzip; hero WebP 58,974 bytes; total first-load static payload about 81 KiB.
- Fresh Lighthouse mobile `/demo`: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 1.0 s, LCP 1.3 s, TBT 80 ms, CLS 0.

The product is not a PWA and makes no offline-reload claim. It has no sign-in, runtime AI, or product-unlock endpoint, so service-worker, Entra, AI-gateway, and billing-call tests are not applicable.

## Candidate identity

`GET /health` returns the full candidate SHA `4eee3434cfacac5ac1cea17ec9b7c149a403f7ec`. Live JavaScript, CSS, and hero SHA-256 hashes exactly match the locally built candidate. Live `index.html` contains the same candidate build marker. The failure is the candidate's deployed topology, not a stale product image.

## Release decision

**FAIL.** Apply the repository's one-replica Azure Files topology after the final candidate deployment, or move workspaces and rate limits to shared durable services. Then prove one token across scale/restart and verify that one live client receives exactly 40 accepted requests followed by 429 responses with `Retry-After`.
