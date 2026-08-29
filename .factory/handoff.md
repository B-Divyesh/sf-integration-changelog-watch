# Handoff — independent verification 7

## Release state

**FAIL — do not release.** Verified on 2026-08-29 UTC at candidate `4f40913d1e8c105d9bbdec0c72ba4dae3be577ba` and `https://integration-changelog-watch.sociobot.in`.

The exact candidate image and assets are live, but the deployment is configured for 1–3 replicas with no volume. During fresh verification it ran two replicas, split workspace-local SQLite state, and multiplied in-memory rate limits. A new token produced 24 × 200 and 24 × 401 across 48 reads. A 120-request single-client burst accepted 90 before 30 × 429; all 429s had `Retry-After: 1`. The factory root-page verifier timed out because parallel dashboard reads split into 200/401 responses.

The researched core mapping is also incomplete: the web flow never asks for an affected dependency version, and neither web nor CLI action cards display one. Pending cards say **“Needs owner”** even while displaying an owner; the missing step is acknowledgement. The hosted three-watch cap also has no researched paid/team path.

Full evidence and repair requirements are in `.factory/verification-7.md`.

## What was verified

- All 13 exact `.factory/claims.json` commands passed after clean `npm ci`.
- First-read and one-click sample-data gates passed.
- `npm test`, typecheck, lint, exact frontend build, Rust format/test/clippy, locked release build, container build script, full browser matrix, and dedicated accessibility suite passed locally.
- Local and live browser matrices each reported 47 passed / 1 intentional shared-IP skip before the explicit live scale probe.
- One local release process completed add → scan → action → acknowledge → deduplicate → delete, rejected invalid/private inputs, enforced 3 concurrent watches, persisted across restart, and enforced 40 allowed / 40 limited with `Retry-After`.
- A clean packaged CLI consumer installed and completed help, demo, scan, deduplicate, and acknowledge flows.
- Live desktop/390 px, keyboard, focus, reduced motion, axe, request privacy, headers, caching, routes, bundle budgets, and candidate byte identity were checked.
- Lighthouse mobile `/demo`: 96 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 1.3 s, CLS 0.
- Read-only Azure inspection confirmed candidate image `4f40913d1e8c`, `maxReplicas: 3`, no volume/mount, and no termination grace, contrary to `deploy/containerapp.yaml`.

## Required next steps

1. Apply and verify the checked-in one-replica `/data` Azure Files topology, or move state and rate limiting to shared durable services.
2. Prove the same token remains valid across bursts, scale/restart, and a 48-read sequence; prove a 120-request client sees the configured 40-request burst and `Retry-After` on every 429.
3. Add the affected dependency version to web add/edit, watch rows, web action cards, and CLI Markdown cards.
4. Rename the pending state to acknowledgement language.
5. Implement or explicitly rescope the brief's hosted many-watch/team paid path.
6. Re-run every claim, full live browser suite, factory `verify-url.sh`, and this deployment probe.

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
npm run test:a11y
```

No product code was modified. Only this handoff and the independent verification report were added/updated.
