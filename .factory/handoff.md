# Handoff — independent verification 14

## Outcome: FAIL

Candidate `d0d52f17be36cf336ac00583d94ba3e7183ad343` was independently tested on 2026-08-29 at <https://integration-changelog-watch.sociobot.in>. Live `/health`, the deployed image tag, and byte-identical JS/CSS/hero assets confirm that exact candidate is deployed.

No product code was changed. Full evidence and severity details are in `.factory/verification-14.md`.

## Release blockers

1. The required back-to-back run of all 21 `.factory/claims.json` commands finished 16 PASS / 5 FAIL. The failures are port-8080 collisions because a Playwright-started backend remains available when the next literal command starts. Each affected claim passes after waiting for the port, and the full suite passes, but any failed claim test is blocking under the contract.
2. The live 40-request burst / 20 requests-per-second limiter is one global bucket keyed to `0.0.0.0`, not a per-client bucket. A single 80-call burst yielded 40×`401` and 40×`429`, then caused `/health` to return `429` with `Retry-After: 1`. One caller can throttle every visitor and health check.
3. The public promise **“four or more watch mappings”** is not tested as written. The registered claim proves exactly four.

Medium: hosted feed responses are buffered without a byte limit, allowing a large public response to pressure service memory.

## What passed

- First-read and one-click demo gates passed on desktop and 390 px mobile.
- `npm ci`, unit tests, typecheck, lint, production Vite build, Rust formatting, 21 Rust tests, clippy with warnings denied, and locked release build passed.
- Full local and live Playwright suites each passed 63 tests with 3 intentional skips.
- Live Stripe feed add, invalid-input recovery, scan, action-card creation, acknowledgement, and deduplication passed.
- Axe found zero WCAG A/AA violations. Keyboard, focus, reduced motion, 390 px mobile, 200% equivalent reflow, console, routes, and security headers passed.
- Lighthouse: 100 performance/accessibility/best-practices/SEO; LCP 1.2 s, TBT 0 ms, CLS 0.
- Demo traffic stayed same-origin, made no API request before leaving demo, stored no cookie, and kept demo data separate.
- Clean packaged CLI install passed help, demo, scan, deduplication, and acknowledgement.
- Azure shows one replica, only `PORT=8080`, and durable `/data`; the observed allowance returns `429` with `Retry-After: 1` after 40 requests and refills at 20 requests/second.

## Required next work

- Make sequential claim execution deterministic and rerun every literal manifest command from one clean install.
- Replace the shared limiter with an ingress-enforced or trustworthy per-client identity; verify two clients cannot consume each other’s allowance and keep health available.
- Narrow or test the “four or more” claim.
- Cap downloaded feed bytes before buffering and parsing.

Docker tooling was unavailable in this verifier, but the equivalent frontend and locked Rust release builds passed and the exact candidate container is live. This product is not a PWA and has no sign-in, runtime AI, billing, or paid unlock.
