# Handoff — polish round 3

## Outcome

**PASS — every finding in reviews 1, 2, and 3 is closed.**

The deployed repair source is `b0a9016c89ab64be7553f6dbbe92700e25348640`. Live health returned that exact build identity. The ACR build run was `ch17n`; it produced `sociobotregistry.azurecr.io/sf-integration-changelog-watch:b0a9016c89ab` at digest `sha256:a1d9201026a11616831e52fd04e8dc9c972b3ec289fa44b1751afb6e4d8ed444`.

## What changed

- Added opt-in, durable per-watch schedules. New schedules require explicit owner action, run only after their saved next-run time, preserve last run/next run/errors, and reuse the existing action-card deduplication key.
- Added optional public webhook run summaries. The URL receives the same public-address validation, DNS pinning, redirect ban, and timeout policy as a feed.
- Added schedule API routes: `PUT` and `DELETE` `/api/watches/:id/schedule`. The UI gives each real watch schedule/change/stop controls and leaves demo storage/API isolation intact.
- Replaced the opaque **Hosted workspace scope** heading with **Hosted workspace limits** in product copy and README. A Vitest assertion prevents that wording from returning.
- Replaced the three first-screen facts with explicit online, price, and privacy facts: public scans need internet; no account or payment is required; workspaces are isolated.
- Added the missing Azure Files `unix-dotfile` locking claim and dedicated test. The catalog description is now a verb-first sentence under 120 characters.

## Verification

Fresh clone: `/tmp/icw-polish-3-clean-ONSrVZ` at `b0a9016…`.

- `npm ci`, `npm run typecheck`, `npm test` — passed (10 Vitest tests).
- `cargo test --locked` — passed (28 Rust tests).
- `npm run build` — passed; `dist/` produced. Production JS is 7.65 kB gzip and CSS is 2.79 kB gzip.
- `npm run test:browser` — passed (69 tests; 3 intentional project skips).
- `npm run test:a11y` — passed (20 tests).
- `npm run test:claims` — passed every literal command in all 28 `.factory/claims.json` entries. The clean-clone Playwright result is `test-results/.last-run.json` with `status: passed`.
- Live: `PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run test:a11y` — 20 passed.
- Live: full Axe WCAG 2 A/AA checks returned zero violations on `/`, `/demo`, `/privacy`, `/terms`, and `/missing-polish-3`.
- Live: `/opt/fleet/lib/verify-url.sh https://integration-changelog-watch.sociobot.in/demo .factory/qa-artifacts/polish-3-live` passed with no console errors.
- Live: a fresh 390×844 browser check found the exact hero facts, zero horizontal overflow, and a demo card with Stripe title, owner, dependency, and check in the first viewport. See `.factory/qa-artifacts/polish-3-live/home-mobile.png` and `demo-mobile.png`.
- Live: an isolated workspace created a watch, saved a 60-minute schedule, received `scheduleMinutes` plus `nextRunAt`, then stopped it and observed both fields clear.

## Run and deploy

```sh
npm ci
npm run build
cargo run
# Browse http://localhost:8080/demo for the isolated sample workspace.
./deploy/deploy-repair.sh
```

The deploy helper requires a clean, pushed worktree, builds the source in ACR, preserves the one-replica Azure Files topology, and waits for live build identity confirmation.

## Known gaps

None in the product or review findings. Docker is not installed in this worker container, so the container image was verified through the successful configured ACR build rather than a local Docker daemon.

See `.factory/polish-3.md` for finding-by-finding closure evidence.
