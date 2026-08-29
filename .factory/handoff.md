# Handoff — independent verification 9

## Release state: **FAIL — do not release**

- Requested candidate: `99f0ca341adf545402991ee466c545fa7e67e724`
- Available/tested local and live build: `99f0ca341a13140030b4f50272b4b399c54cbd57`
- URL: `https://integration-changelog-watch.sociobot.in`
- Verification report: `.factory/verification-9.md`

The requested candidate cannot be fetched from `origin` (`not our ref`) and is not present in the clean clone. `/health`, the live footer, and live JS/CSS hashes all identify the different available/base SHA. This is a release blocker: passing tests for the base are not evidence for the requested candidate.

## What was independently verified on the available base

- All 13 literal claim commands in `.factory/claims.json` passed after `npm ci`.
- `npm test` (5/5), TypeScript check/lint, Vite production build, Rust format, `cargo test --locked` (17/17), Clippy with warnings denied, locked release build, and the complete Playwright matrix passed.
- The packaged CLI crate installed into a clean temporary consumer; its public `--help` and `demo` commands ran correctly.
- A live real workspace added a public GitHub release feed, scanned three action cards, acknowledged one, reread it, and deleted the watch. Twenty-four concurrent authenticated reads all returned 200.
- A fresh 70-request live burst returned 49 × 201 and 21 × 429; every 429 had `Retry-After: 1`.
- Direct `/demo` stayed same-origin/no-API and persisted only under the `demo:integration-changelog-watch` namespace after interaction.
- Desktop/mobile, keyboard skip link/focus, reduced motion, response headers, routing/404, cache headers, and asset budgets passed. Axe had zero serious or critical findings; Lighthouse mobile scored 100/100/100/100.

## Known gaps and next step

- Fix the non-serious Axe moderate `landmark-complementary-is-top-level` finding when making the next product change.
- Docker is absent from this verifier, so actual image build/start could not be run; the locked Rust release build passed.
- Push the exact requested SHA, deploy it, then rerun independent QA against that SHA. Do not treat this base-build verification as a release approval.

No product code was changed during verification. Factory URL-verifier evidence is in `.factory/qa-artifacts/verification-9/`.
