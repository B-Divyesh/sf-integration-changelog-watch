# Handoff — independent verification 8

## Release state

**FAIL — do not release.** Independently verified on 2026-08-29 UTC at candidate `4eee3434cfacac5ac1cea17ec9b7c149a403f7ec` and `https://integration-changelog-watch.sociobot.in`.

The exact candidate image and static assets are live. The running Container App has reverted to `minReplicas: 1`, `maxReplicas: 3` with no volume or `/data` mount. It currently runs three replicas. A fresh token's 24 authenticated reads split **12 × 200 / 12 × 401**; six cold browser loads all showed the workspace-load recovery message and logged a 401 console error. One 150-request client burst received **120 × 401 / 30 × 429**, so the configured 40-request allowance is tripled. Every 429 did include `Retry-After: 1`.

Full findings and evidence are in `.factory/verification-8.md` and `.factory/qa-artifacts/verification-8/`.

## What was verified

- First-read and one-click sample-data gates pass at desktop and true 390 px.
- After `npm ci`, all 13 exact `.factory/claims.json` commands pass. The `single-replica-durable-data` claim is nevertheless false for the running deployment because its test checks only the repository template.
- Unit tests, typecheck/lint, frontend production build, Rust formatting/tests/Clippy, locked release build, and the local browser matrix pass. The final live matrix fails 3 tests (44 passed, 1 skipped): both `@claim:workspace-boundary` projects and the authenticated-read consistency test.
- One local release process completes add → scan → action → acknowledge → deduplicate → edit/delete, rejects invalid/private inputs, enforces three concurrent watches, persists across restart, and enforces 40 allowed / 40 limited with `Retry-After`.
- A separately installed packaged CLI completes help, demo, scan, deduplicate, and acknowledge workflows.
- Direct demo privacy, desktop/mobile layout, keyboard focus, reduced motion, 200% reflow, Playwright Axe, headers, caching, routing, link status, and bundle budgets pass.
- Lighthouse mobile `/demo`: 100 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 1.3 s and CLS 0.
- `/health`, HTML build marker, and exact asset hashes prove that production serves candidate `4eee3434…`.

## Required repair

1. Deploy with `deploy/deploy-repair.sh` after the final candidate commit and verify the running revision has `maxReplicas: 1`, Azure Files storage, and a `/data` mount. Prevent the generic 1–3 replica deployment from overwriting this topology.
2. Re-prove one workspace token across repeated reads and a revision restart. Confirm its watches/actions remain available.
3. Re-run a fresh live burst and require exactly 40 accepted requests, then 429 with `Retry-After`.
4. Resolve the brief's missing hosted team/paid path or record an accepted product-scope change outside the product's self-authored handoff.
5. Update the CLI Markdown card when `ack` records completion, and ship a square 180 × 180 Apple touch icon.

## Commands

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
PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run test:browser
```

No product code was modified during verification. Docker/Podman/Buildah were unavailable, so the full container image could not be launched locally; the exact frontend and locked release-backend build steps passed, and the Dockerfile was inspected.
