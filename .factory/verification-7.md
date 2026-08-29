# Independent verification 7 — FAIL

Verified independently on 2026-08-29 UTC.

- Candidate commit: `4f40913d1e8c105d9bbdec0c72ba4dae3be577ba`
- Live URL: `https://integration-changelog-watch.sociobot.in`
- Work order: `integration-changelog-watch-verify-7`
- Result: **FAIL — do not release**

The exact candidate image and static assets are live, and the candidate works on one local process. Production does not use the candidate's required one-replica, durable-volume topology. During a fresh load probe it scaled to two local-SQLite replicas, split every new workspace, and multiplied the per-client request allowance. The product also omits the affected dependency version from both web and CLI action cards, despite that field being central to the researched job.

No product code was changed during verification.

## Mandatory first gates

### First read — PASS

A cold live visit answered all three required questions in its first screen:

- What: **“Turn vendor changes into owned actions.”**
- For whom: **“For engineers who maintain payment, auth, analytics, or messaging integrations.”**
- First action: **“Try it with sample data”**, beside **“See matched notices, owners, and checks.”**

One click opened `/demo` with three realistic watches, two action cards, owners, check commands, and the persistent **“Demo — sample data, nothing is saved”** banner with **Reset demo** and **Start for real**.

### Declared claims — PASS locally

`.factory/claims.json` exists with 13 entries. After the clean `npm ci`, every listed command was run exactly as declared through the product's demo/local entry points. All passed.

| Claim | Result |
| --- | --- |
| `sample-action-cards` | PASS, 2 Playwright projects |
| `csv-export` | PASS, 2 Playwright projects |
| `demo-local` | PASS, 2 Playwright projects |
| `workspace-boundary` | PASS locally, 2 Playwright projects |
| `redirecting-feeds` | PASS, 1 Rust test |
| `requested-scans` | PASS, 2 Playwright projects |
| `cli-repository-workflow` | PASS, 2 Playwright projects |
| `cli-demo-local` | PASS, 2 Playwright projects |
| `database-persistence` | PASS, 1 Rust test |
| `demo-isolation-transitions` | PASS, 2 Playwright projects |
| `cli-shipped-mapping-local` | PASS, 2 Playwright projects |
| `port-only-startup` | PASS, 1 Rust test |
| `single-replica-durable-data` | PASS, 1 Rust test |

The last claim proves the versioned deployment template, not the running deployment. Fresh production evidence below contradicts the required runtime topology.

## Release-blocking findings

### Critical — production splits and loses workspace state across ephemeral replicas

Read-only Azure inspection of revision `sf-integration-changelog-watch--0000021` returned:

```json
{
  "image": "sociobotregistry.azurecr.io/sf-integration-changelog-watch:4f40913d1e8c",
  "minReplicas": 1,
  "maxReplicas": 3,
  "terminationGrace": null,
  "volumes": null,
  "volumeMounts": null
}
```

This differs from `deploy/containerapp.yaml`, which fixes both scale bounds at one, mounts Azure Files at `/data`, and sets a 30-second termination grace. Two ready replicas were observed during the probe. Each replica therefore used its own image-local SQLite database and in-memory rate bucket.

Fresh production failures:

- A workspace creation returned 201, but its next authenticated add and scan returned 401 **“This workspace token is not active on the server.”** Other immediate reads with the same token returned 200.
- A separate fresh token made 48 sequential authenticated reads: **24 × 200 and 24 × 401**, in a repeating cross-replica pattern.
- Ten concurrent creates in one new workspace yielded **3 × 201, 6 × 401, and 1 × 409** instead of the single-process result **3 × 201 and 7 × 409**.
- The factory `verify-url.sh` twice received HTTPS 200 but timed out after 60 seconds waiting for root-page `networkidle`. A targeted trace showed `/api/watches` returning 200, `/api/actions` returning 401, a browser console resource error, and both fetches remaining unfinished from Playwright's perspective.
- Azure later scaled back to one replica. Any workspace stored only on the removed replica disappeared because no volume is attached.

This breaks the hosted product's core create/read/scan/acknowledge workflow and its privacy promise that a browser-held workspace remains available. Apply the checked-in topology (or use a shared durable database), then prove a token across scale/restart and repeat the full live suite.

### Major — the documented single-client rate allowance is fragmented

The candidate implements a burst of 40 requests with a 20 request/second refill. A single local process enforced it exactly: an 80-request burst returned **40 × 401 and 40 × 429**, and every 429 had `Retry-After: 1`.

The same fresh live client sent 120 requests in 589 ms and received **90 × 401 and 30 × 429**. All 429 responses did include `Retry-After: 1`, but production accepted 90 requests before limiting because multiple replicas own independent buckets. The observed live allowance is therefore 90 for this burst, not the configured 40.

### Major — action cards omit the affected dependency version

The researched gap explicitly requires a repository-owned mapping from a vendor change to an owner, the affected dependency version, a test command, and acknowledgement state. The data model accepts `version`, and the shipped example contains `stripe-node 16.2`, but neither product path operationalizes it:

- The web **Add a watch** flow asks only for vendor, URL, keywords, owner, and check command. It never asks for a dependency version.
- Watch rows and action cards never display `version`; even the demo hides its populated sample versions.
- CLI Markdown action cards include rule, owner, check, and notice, but omit `watch.version`.

The resulting action card cannot tell an engineer which installed dependency is affected, so the smallest product does not fully meet its distinguishing job.

### Moderate — the core pending label contradicts the card

The demo heading says **“1 action needs an owner”** and the card says **“Needs owner”** while that same card already displays **“Owner — Maya · Payments.”** The pending state is actually acknowledgement. This conflicts with the brief's acknowledgement measure and gives the engineer the wrong next step; it should say that the action needs acknowledgement or verification.

### Moderate — the researched hosted paid path is absent

The hosted dashboard enforces a hard three-watch limit (`0/3`, then HTTP 409), but offers no upgrade, team ownership, hosted-history tier, price, checkout, or license restore. The brief says a few watches are free and many watches/team ownership support a subscription. The CLI can map more feeds locally, but the researched hosted monetization path is not implemented or documented as a deliberate deviation.

## Candidate behavior on one process

The locked release binary was run with a temporary SQLite database and the repository's shipped feed. The smallest useful workflow passed:

1. Create workspace: 201, 64-character token.
2. Add the public shipped feed mapping: 201.
3. Scan: 200, one **“Webhook delivery format changes”** action.
4. Acknowledge: 200 with `acknowledged: true`.
5. Scan again: 200 with zero new actions.
6. Second workspace sees zero records and cannot delete the first workspace's watch (404).
7. Delete the original watch: 204.

Boundary and recovery behavior on that process:

- missing vendor: 400 with all required fields named;
- 121-character vendor: 400 with the 120-character limit;
- malformed URL: 400 with a complete-URL instruction;
- loopback URL: 400 with the public-network instruction;
- credential-bearing URL: 400 with the no-credentials instruction;
- ten concurrent creates: exactly 3 × 201 and 7 × 409;
- file-backed workspace survived a real graceful process stop/restart;
- `env -i PATH=... PORT=8092` started successfully with no other product configuration, served `/health`, and stopped gracefully on SIGINT.

## Local quality gates

| Check | Result |
| --- | --- |
| `npm ci` | PASS; 60 packages, 0 audit findings |
| `npm test` | PASS, 3/3 |
| `npm run typecheck` | PASS |
| `npm run lint` | PASS |
| `npm run build` | PASS; `dist/` generated |
| `cargo fmt --all -- --check` | PASS |
| `cargo test --locked` | PASS, 16/16 |
| `cargo clippy --all-targets --locked -- -D warnings` | PASS |
| `cargo build --release --locked` | PASS |
| `npm run test:container` | PASS |
| `npm run test:browser` | PASS locally, 47 passed / 1 intentional shared-IP skip |
| full browser suite against live URL | PASS before the scale-out probe, 47 passed / 1 intentional skip |
| `npm run test:a11y` | PASS, 14/14 |
| factory `verify-url.sh` on live root | **FAIL**, 60-second `networkidle` timeout caused by split API responses |

Docker, Podman, and Buildah are unavailable in this worker. The Dockerfile's exact frontend and locked release-backend build steps passed; inspection confirms a current-stable Rust build stage, `BUILD_SHA`, non-root runtime user, `/data`, and port 8080.

## Packaged CLI consumer

`cargo package --locked --allow-dirty --no-verify` produced an 86-file, 5.8 MiB compressed crate. It was extracted and installed into a separate Cargo root. The installed binary passed `--help` and `demo`; the shipped mapping created one Markdown card, a second scan created no duplicate, and `ack` persisted `acknowledged: true`. This also exposed the missing version and contradictory **“Needs owner”** wording in the generated card.

## Accessibility, privacy, routing, headers, and performance

- Fresh desktop and 390 px visual checks found no clipping or overlap. At 390 px, content width equaled viewport width; the 195 px/200% reflow test passed.
- The first Tab reveals a 141.5 × 44 px skip link with a 3 px dashed indigo focus ring. Keyboard acknowledgement and route-focus tests passed.
- Axe WCAG A/AA checks found zero serious/critical issues on desktop and mobile. The live full suite included all 14 accessibility checks.
- With `prefers-reduced-motion: reduce`, action-card transition durations become `0s`.
- Direct `/demo` requested only its same-origin document, hashed JS, CSS, and hero image. It made no API, analytics, advertising, remote-font, third-party, console, or page-error request.
- Security headers include header-delivered CSP with `frame-ancestors 'none'`, HSTS, `nosniff`, strict-origin referrer policy, and restrictive permissions policy.
- Cache behavior is correct: HTML `no-cache`; hashed JS/CSS one-year immutable; hero/icon assets one week; API `no-store, private`.
- `/`, `/demo`, `/privacy`, `/terms`, robots, sitemap, social card, favicon, and apple-touch icon return 200. The styled unknown route returns 404. All same-origin links return 200; both demo vendor links return 200.
- Production sizes: JS 13,794 bytes raw / 5.35 KiB gzip, CSS 7,845 bytes raw / 2.55 KiB gzip, hero WebP 58,974 bytes. Total first-load transfer was 81 KiB.
- Fresh mobile Lighthouse on `/demo`: Performance 96, Accessibility 100, Best Practices 100, SEO 100; FCP 1.0 s, LCP 1.3 s, TBT 250 ms, CLS 0.
- `GET /health` returns the full candidate SHA. Live JS, CSS, and hero hashes exactly match the candidate; normalized `index.html` differs only by replacement of `{{BUILD_ID}}` with that SHA.

The product is not a PWA and makes no offline-reload claim. It has no sign-in or runtime AI feature, so service-worker, Entra, and AI-gateway checks are not applicable. It also has no billing/unlock endpoint; that absence is recorded above against the researched monetization scope.

## Release decision

**FAIL.** The candidate is healthy on one process and its demo is strong, but the deployed backend is neither single-replica nor durable. Fresh users receive cross-request 401s, workspace data can disappear on scale-in, and rate limits are multiplied. The missing dependency-version field also leaves the core action-card contract incomplete.
