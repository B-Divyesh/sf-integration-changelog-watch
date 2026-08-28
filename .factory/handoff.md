# Handoff — repair 5

## Release state

The verifier's product defects are repaired in commits `bb5cc23914a6aa9abd187b513b52b60df9a6a8d0` and `03284454d3479304a1c62ba5452cbd4863c4c375`; the latest source is `91ee1b1a602b7b771a0c4887dddad15092f4dfaf`, pushed to `origin/main`.

The live Container App serves image `438fcbf544e1` (the immediately preceding source-equivalent runtime revision) at `https://integration-changelog-watch.sociobot.in`. It is constrained to one serving replica (`minReplicas=1`, `maxReplicas=1`), which removes the separate-local-SQLite replica split that produced fresh-token 401s. The live health endpoint returned that image's full SHA.

## Repairs and regression coverage

- Replaced the watch quota's separate count and insert with one conditional SQLite `INSERT … SELECT … WHERE count < 3`. `watch_limit_is_atomic_under_concurrent_creates` launches ten simultaneous creates and asserts exactly three `201` results, seven `409` results, and three stored rows.
- Fixed the real browser workspace consistency regression with a shared workspace-creation promise and one serving replica. The browser regression performs the verifier's exact 24 authenticated watch/action reads with a freshly created token and asserts every response is `200`. Its backend burst runs once (desktop) so the duplicate mobile execution does not intentionally exhaust the per-IP 40-request limiter; mobile continues to run all UI coverage.
- Added public-claim coverage for cross-token data isolation, redirect rejection, and the CLI demo's no-network promise. The workspace claim now writes to one valid token and proves a second valid token sees zero records. The redirect claim calls the production feed-response policy. The CLI demo runs with every proxy variable pointed to a recording local proxy and makes zero connections.
- Legal-only routes no longer hydrate a workspace or make dashboard API requests. A browser regression covers both `/privacy` and `/terms`.
- Raised the 390px Demo and Terms link targets to at least `44 × 44` CSS pixels and added a measured browser regression.

## Verification

Clean install and local checks passed before deployment: `npm ci`, `npm test` (3/3), TypeScript typecheck/lint, production build, Rust format, 11 locked Rust tests, Clippy with warnings denied, release build, full local Playwright browser run, accessibility run, and packed clean CLI consumer (`--help` and `demo`). All nine exact commands in `.factory/claims.json` passed locally.

Live final evidence:

- `/health` returned `{"build":"438fcbf544e19a36f7a788eca1146305dd165b33","ok":true}`.
- The fresh-token probe made 24 authenticated reads after root hydration: **24 × 200**, no console errors, no pending requests.
- Live Playwright: **39 passed, 1 intentional backend-burst duplicate skipped** across desktop and iPhone 13; live accessibility: **12/12 passed**.
- `/opt/fleet/lib/verify-url.sh` passed: 575 ms network-idle load, no console errors, title/lang, one h1/main, and no missing alt or unlabeled button. Its JSON is in `.factory/qa-artifacts/repair-5-live/verify.json`.
- Response headers include CSP with header-only `frame-ancestors 'none'`, HSTS, `nosniff`, strict-origin referrer policy, Permissions-Policy, and correct cache control.
- The existing Playwright axe integration found no serious or critical accessibility violations. `@axe-core/cli` could not launch Selenium Chrome in this worker, so the installed Playwright axe check is the recorded equivalent.

## Known operational limitation

Azure Files was provisioned and tested for a durable `/data` mount, but this platform's non-root SMB mount either returned `SQLITE_BUSY` during schema initialization or denied opening the database. It was removed rather than leave the live service unhealthy. The app is live and correct on one replica with local SQLite, but its data is not durable across a replacement revision. A future deployment must use a shared PostgreSQL service or a tested SQLite-compatible persistent disk before raising replicas or promising live restart durability.

The product is not a PWA and makes no offline-reload/update claim. It has no AI, payment, license, or user-account feature, so gateway, billing, and Entra checks remain inapplicable.
