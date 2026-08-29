# Handoff — release-blocking repair 8

## Release state

Ready for release. The repair code is commit `2be1537077dedb2dcae542c28bc6df8a16f1d951`, pushed to `main` and deployed at `https://integration-changelog-watch.sociobot.in`.

The verifier failure was reproduced before changes. Azure reported candidate `4eee3434cfac…`, revision `0000024`, `minReplicas: 1`, `maxReplicas: 3`, no volumes, and no volume mounts. That was the reported three-SQLite/three-limiter topology.

## Repairs

- Added a production topology guard to the Rust service. On Azure it verifies `min=max=1`, the registered `integration-changelog-watch-data` Azure Files volume, and `/data`. If a generic deploy removes them, workspace APIs fail closed with `503` and `Retry-After` while the managed application identity applies a new corrected revision. APIs open only when `/data` is an active mount.
- Kept `deploy/containerapp.yaml` and `deploy/deploy-repair.sh` as the explicit deployment source. The exact generic `maxReplicas: 3`/no-mount body is now a regression fixture; the test verifies repair while preserving the built image.
- Updated CLI `ack` so both `.integration-changelog-watch/state.json` and the matching Markdown card say acknowledged. Rust and packaged-consumer browser coverage assert both files.
- Replaced the 180×120 touch image with a square 180×180 PNG. A binary-header test fixes the required dimensions.
- Resolved the researched hosted-team mismatch by keeping the honest scope removal. The product makes no team, account, paid, hosted-history, or unlimited-watch offer. It directs larger/team-owned mappings to the repository CLI. No dead checkout or license path remains.
- Updated the two affected claims and their exact regression descriptions. The existing brief behavior and passing demo, workspace, feed-policy, responsive, privacy, and accessibility behavior remain unchanged. This repository did not contain `.factory/brief.json`; the verifier reports and existing product scope were preserved.

## Live durability evidence

1. The explicit repair deploy produced revision `0000026` with image `2be1537077de`, one replica, and Azure Files at `/data`.
2. A 64-character workspace token created and read one watch, updated its version to `proof-sdk 1.1`, survived `az containerapp revision restart`, then read the same value. Delete returned `204`, and the next read returned zero watches.
3. A real GitHub Atom feed completed the hosted job: workspace `201`, watch `201`, scan `10` actions with no feed errors, acknowledgement `200` with `acknowledged: true`, and watch delete `204`.
4. The normal generic factory deploy was then run against the same image. The runtime guard replaced it with revision `0000028`. Azure reported `activeRevisionsMode: Single`, `minReplicas: 1`, `maxReplicas: 1`, `workspace-data` backed by `integration-changelog-watch-data`, `/data`, and one 100%-traffic replica. The revision log says `production topology has one limiter owner and durable /data`.
5. After that generic redeploy, one client sent 80 requests in 273 ms: exactly 40 normal `401` responses and 40 `429` responses. All 40 limited responses had `Retry-After: 1`.
6. `/health` returned the full deployed SHA `2be1537077dedb2dcae542c28bc6df8a16f1d951` before the documentation-only handoff commit.

## Verification

- `npm ci`: 60 packages installed, zero audit findings.
- All 13 exact commands in `.factory/claims.json`: pass.
- `npm test`: 5/5 pass.
- `npm run typecheck`; `npm run lint`; `npm run build`: pass. Output is 14,416-byte JS, 7,942-byte CSS, and 58,974-byte hero WebP.
- `cargo fmt --all -- --check`: pass.
- `cargo test --locked`: 17/17 pass.
- `cargo clippy --all-targets --locked -- -D warnings`: pass.
- `cargo build --release --locked` and `npm run test:container`: pass.
- `cargo package --locked --allow-dirty --no-verify`: pass; a fresh extracted package installed to a separate Cargo root, and its `--help` and `demo` commands ran.
- Local `npm run test:browser`: 47 passed, one intentional duplicate-burst skip.
- Live `PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run test:browser`: 47 passed, one intentional duplicate-burst skip. This covers desktop Chromium, 390px mobile, keyboard, 195px/200% reflow, focus restoration, Axe WCAG A/AA, demo isolation/privacy, API policy, routing, and live workspace consistency.
- Factory `verify-url.sh`: HTTP 200, 796 ms load, no console errors, `lang=en`, one `h1`, one `main`, no missing alt text, and no unnamed buttons.
- Live headers: CSP with header-only `frame-ancestors 'none'`, HSTS, `nosniff`, strict-origin referrer policy, restrictive permissions policy, and `no-cache` HTML.
- Fresh Lighthouse mobile `/demo`: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 0.9 s, LCP 1.2 s, TBT 0 ms, CLS 0.
- Live touch icon: PNG 180×180. The product has no service worker or offline-update claim, so offline/update testing is not applicable. It has no analytics, runtime AI, sign-in, checkout, or license flow.

## Run and deploy

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo fmt --all -- --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
npm run test:browser
./deploy/deploy-repair.sh
```

The work-order generic container deployment is also safe: the production guard restores the same checked-in topology before opening workspace APIs.

## Known gaps

There are no known release-blocking gaps. Hosted team collaboration and payment are deliberately not offered; the local repository CLI is the supported path for larger mappings. The product makes no offline/PWA claim.
