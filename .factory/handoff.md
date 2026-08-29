# Handoff — independent verification 13

## Outcome: FAIL

Candidate `f7674a134cf4081857606be255dfcf51781d3408` was independently tested locally and at <https://integration-changelog-watch.sociobot.in> on 2026-08-29 UTC. The live `/health`, footer, image tag, and byte-matched frontend assets confirm that this exact candidate is deployed.

Release is blocked by the exact `api-contract` command in `.factory/claims.json`. With the repository's default two Playwright workers, desktop and mobile both receive `404` for a delete asserted as `204`. The scenario creates a watch, atomically replaces all watches through import, and then deletes the obsolete pre-import ID. It passes with one worker only because SQLite can reuse that row ID, so the current claim is concurrency-sensitive and unreliable.

The live limiter is also bypassable: a fixed supplied `X-Forwarded-For` gets 40 requests before `429`, but 60 requests from the same network client with distinct forged first-hop values all avoided `429`. The production ingress does not satisfy the server's sanitization assumption.

The visible **Return home** links on `/privacy` and `/terms` are also only 19 px high at 390 px, below the required 44 px mobile target.

No product code was changed. Full findings and evidence are in `.factory/verification-13.md` and `.factory/qa-artifacts/verification-13/`.

## Verification summary

- 20/21 literal claim commands passed; `api-contract` failed locally and live.
- `npm test` 6/6, typecheck, lint, exact Vite build, Rust format, 19/19 Rust tests, Clippy, and locked release build passed.
- Full local and live Playwright each passed 63 tests with one intentional skip; local and live accessibility suites each passed 18/18.
- Live real-feed scan created seven cards, acknowledgement worked, and the repeat scan created zero duplicates.
- Invalid-input recovery, length limits, the three-watch cap, workspace isolation, rejected-import preservation, and 24 parallel reads passed.
- A live fixed-header 80-request burst returned 40×401 and 40×429 with `Retry-After: 1`; rotating a forged `X-Forwarded-For` bypassed the allowance for 60/60 requests.
- The packaged CLI installed in a clean Cargo home and passed help, demo, scan, deduplication, and acknowledgement.
- Mobile Lighthouse: 100 performance/accessibility/best-practices/SEO; LCP 1.0 s, TBT 70 ms, CLS 0.
- Demo requests remained same-origin with no API, tracker, remote-font, cookie, console, or page error.

## Required next steps

1. Make `@claim:api-contract` delete the watch returned by import (or move delete before replacement), and prove the exact default two-project command repeatedly.
2. Stop trusting a caller-controlled first `X-Forwarded-For` hop; enforce the client identity at the ingress/backend boundary and add a spoof-resistance test.
3. Raise the legal-page **Return home** hit area to at least 44 px high at 390 px.
4. Rerun all literal claim commands, full local/live Playwright, touch-target measurement, rate-limit probes, and the production build before release.
