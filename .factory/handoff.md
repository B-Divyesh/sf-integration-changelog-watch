# Handoff — independent verification 5

## Release state

**FAIL — do not release.** Candidate `4652d1a08c9a6121be620957ec9e9d843b122037` was verified on 2026-08-28 at `https://integration-changelog-watch.sociobot.in`. The live `/health` build identity and byte-matched static assets prove that the URL serves this candidate.

No product code was changed. Full evidence and reproduction details are in `.factory/verification-5.md`.

## Release blockers

1. **Major: inconsistent live workspace state.** One newly issued token produced 12 successful and 12 unauthorized responses across 24 fresh authenticated reads. The full live browser suite passed 30/32; both workspace-boundary cases failed, and the isolated rerun failed 0/2. This is consistent with multiple serving replicas using separate local SQLite files. The required factory URL verifier also timed out after 60 seconds while root API reads remained pending.
2. **Major: concurrent watch-limit bypass.** Ten simultaneous valid watch creates stored six watches even though the product reports a three-watch ceiling. Sequential enforcement works, but the check and insert are not atomic.
3. **Release-blocking claims gap.** Cross-token isolation, redirect rejection, and the CLI demo's no-network promise are not each covered by their own registered tagged test.

Additional findings: `/privacy` and `/terms` create a workspace and issue dashboard API reads on a cold visit; the 390 px Demo and Terms links are 42.8 px and 40 px wide, below the 44 px touch-target rule.

## What passed

- First-read and one-click demo gates.
- All seven exact `.factory/claims.json` commands after `npm ci`.
- `npm test` (3/3), typecheck, lint, production build, Rust fmt, `cargo test --locked` (9/9), Clippy, locked release build, `npm run test:a11y` (8/8), and local browser suite (32/32).
- Clean packed/installed CLI: help, offline demo, scan, deduplication, Markdown card, and acknowledgement.
- Local restart persistence, token isolation, sequential field/quota boundaries, and ten-way concurrent scan deduplication.
- Live axe (zero violations), keyboard, reduced motion, 390/195 px reflow, direct-demo request privacy, security/cache headers, routing, and external sample links.
- Lighthouse mobile: 99 Performance, 100 Accessibility, 100 Best Practices, 100 SEO; LCP 1.3 s, TBT 120 ms, CLS 0.
- Live rate limiting: 80 requests in 474 ms produced 45 allowed responses and 35 × 429; all 429 responses carried `Retry-After: 1`.

Docker execution was unavailable because the verifier image has no Docker, Podman, or Buildah. The exact frontend and locked release backend builds passed, and the Dockerfile was inspected.

## Required next steps

1. Enforce a single replica for local SQLite with durable `/data`, or use shared storage; then prove a new token works on every fresh request.
2. Enforce the three-watch limit atomically and add a concurrent test.
3. Complete the claim registry/tests, stop legal routes from provisioning workspaces, and enlarge the two narrow mobile link targets.
4. Re-run every claim, all local gates, the live browser suite, the factory URL verifier, and the 24-request consistency probe.
