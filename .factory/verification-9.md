# Independent verification 9 — FAIL

Verified independently on 2026-08-29 UTC.

- Requested candidate: `99f0ca341adf545402991ee466c545fa7e67e724`
- Available local commit: `99f0ca341a13140030b4f50272b4b399c54cbd57`
- Live URL: `https://integration-changelog-watch.sociobot.in`
- Work order: `integration-changelog-watch-verify-9`
- Result: **FAIL — do not release**

## Release-blocking finding

### Critical — the candidate cannot be verified or released

The requested object is not in the clean clone: `git cat-file -t 99f0ca341adf545402991ee466c545fa7e67e724` failed. A fresh `git fetch --no-tags origin 99f0ca341adf545402991ee466c545fa7e67e724` also failed with GitHub's **“not our ref”** response. `git ls-remote origin` advertises only `main` at `99f0ca341a13140030b4f50272b4b399c54cbd57`.

Production is that available/base commit, not the requested candidate:

- `GET /health` returned `{ "build": "99f0ca341a13140030b4f50272b4b399c54cbd57", "ok": true }`.
- The live footer carries the same full SHA.
- SHA-256 of both live hashed assets equals the local base build: JS `9d63cfc7917656c6a5565448f43b2fe7f78fbf802cf6e6e3924a1f2d691515e3` and CSS `4166cc20784845dddb0fba8ac17cf7bdd03877b0f911555444b048a795aa0fa9`.

Passing checks below therefore establish that the *base deployment* is healthy; they are not evidence for the supplied candidate. Supply/push the requested commit and deploy it before requesting release QA again.

## Mandatory first gates

### First read — PASS for the live base deployment

A cold desktop visit answered the three questions in the first screen in plain words:

- What: “Turn vendor changes into owned actions.”
- For whom: “For engineers who maintain payment, auth, analytics, or messaging integrations.”
- First click: **Try it with sample data**, beside “See matched notices, owners, versions, and checks.”

The action opens `/demo` in one click. It shows the persistent “Demo — sample data, nothing is saved” banner, Reset demo, Start for real, three realistic watches, and two owned action cards. A direct fresh `/demo` visit made only same-origin document, JS, CSS, and hero-image requests; acknowledging a sample card wrote only `demo:integration-changelog-watch` and made no API request.

### Claims manifest and exact commands — PASS for the available base

`.factory/claims.json` exists and lists 13 complete claim tests. After `npm ci` (60 packages, zero audit findings), I ran every literal command in the manifest from the clean checkout. All returned zero: `sample-action-cards`, `csv-export`, `demo-local`, `workspace-boundary`, `redirecting-feeds`, `requested-scans`, `cli-repository-workflow`, `cli-demo-local`, `database-persistence`, `demo-isolation-transitions`, `cli-shipped-mapping-local`, `port-only-startup`, and `single-replica-durable-data`.

## Product, backend, and privacy evidence for the live base

- A real end-to-end workspace created a 64-character token, added a public GitHub `stripe-node` release feed, scanned three matching action cards, acknowledged one, reread actions successfully, and removed the watch.
- A fresh token received 24 parallel authenticated `GET /api/watches` and `GET /api/actions` responses: all 24 were `200`.
- Invalid/unsafe source addresses are covered by the exact workspace-boundary claim: anonymous access is `401`, loopback feed creation is `400`, and a second workspace cannot read the first workspace's watch.
- A 70-request burst to `POST /api/workspaces` observed 49 × `201` then 21 × `429`; every limited response had `Retry-After: 1`. The observed per-client burst allowance is therefore 49 in that fresh probe.
- The direct demo request log had no analytics, advertising, remote fonts, or third-party origins. Normal real-workspace requests stay same-origin.
- Headers on HTML/API included HSTS, `X-Content-Type-Options: nosniff`, strict-origin referrer policy, restrictive permissions policy, and a header-delivered CSP with `frame-ancestors 'none'`. API responses are `no-store, private`; hashed JS/CSS are one-year immutable.

## Quality gates for the available base

| Check | Result |
| --- | --- |
| `npm test` | PASS, 5/5 |
| `npm run typecheck`; `npm run lint`; `npm run build` | PASS; `dist/` produced |
| `cargo fmt --all -- --check` | PASS |
| `cargo test --locked` | PASS, 17/17 |
| `cargo clippy --all-targets --locked -- -D warnings` | PASS |
| `cargo build --release --locked` | PASS |
| `npm run test:browser` | PASS |
| Factory `verify-url.sh` | PASS: 200, 990 ms, no console errors, `lang=en`, one `h1`, one `main`, no missing alt or unnamed button |
| `cargo package --locked --allow-dirty --no-verify` | PASS: 101 files, 9.2 MiB / 7.8 MiB compressed |
| Clean consumer CLI install | PASS: packaged crate installed in a new temporary Cargo root; `--help` and `demo` worked |

Docker is not installed in this worker, so the actual `docker build`/container startup contract could not be executed here. The locked release build passed.

## UX, accessibility, routing, and performance for the live base

- Desktop and 390 px mobile had no horizontal overflow; body text is 16 px. The shipped 195 px/200%-equivalent browser test passed.
- First Tab focuses the Skip to content link with a visible `3px` dashed indigo outline; Enter focuses `main`. Reduced-motion media has no active transition or animation.
- Axe found **zero serious or critical** findings. It did report one non-blocking moderate `landmark-complementary-is-top-level` finding; see the minor finding below.
- `/privacy`, `/terms`, `/demo`, `robots.txt`, and `sitemap.xml` returned 200. An unknown route returned the styled 404 with “That page is not here” and a Return home link. Each tested route has its own title.
- First-load assets are within budget: JS 14,416 bytes raw / 5.54 KiB gzip, CSS 7,942 bytes raw / 2.56 KiB gzip, hero WebP 58,974 bytes.
- A fresh Lighthouse mobile `/demo` run scored Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 1.0 s, LCP 1.3 s, TBT 50 ms, CLS 0.

Factory verifier evidence is in `.factory/qa-artifacts/verification-9/`. The product is not a PWA and does not claim offline reload. It has no sign-in, runtime AI, billing/unlock, or payment endpoint, so Entra, service-worker, AI-gateway, and billing-flow checks are not applicable.

## Other finding

### Minor — one non-serious Axe landmark finding

The live landing page produces Axe's moderate `landmark-complementary-is-top-level` result (one node). This does not violate the required serious/critical accessibility gate, but the complementary region should be made top-level or given an appropriate non-landmark element before a future release.

## Release decision

**FAIL.** The requested commit is unavailable from the supplied repository and the live deployment proves it is running a different SHA. Make the exact candidate reachable, deploy that SHA, and repeat candidate-specific QA.
