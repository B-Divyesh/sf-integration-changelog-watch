# Handoff — polish 1

## Outcome

All 19 findings from `.factory/review-1.md` are repaired and mapped in `.factory/polish-1.md`. The deployed repair is `c2b3a9716b68357317753ab71eb78b7af3d12b9a` at <https://integration-changelog-watch.sociobot.in>.

The demo now opens directly onto visible sample work at 390×844. Watch configuration moves between the dashboard and CLI JSON schema through previewed import/export. Route metadata, 404 navigation, terminology, exact claim coverage, and the full Axe result were repaired without changing the paper-cut operations-board identity.

## Verification

- Clean clone at `/tmp/icw-clean-0WsED3`: `npm ci`, then every command listed in `.factory/claims.json` completed from the committed source.
- Local: `npm test`, `npm run typecheck`, `npm run build`, `cargo test --locked`, `cargo build --release --locked`, `npm run test:browser`, and `npm run test:a11y` passed.
- Live: `PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run test:browser` passed with 59 tests and one intentional duplicate-project skip.
- Live: `PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run test:a11y` passed 16/16, including full Axe with zero violations.
- Live: `/opt/fleet/lib/verify-url.sh 'https://integration-changelog-watch.sociobot.in/?verify=c2b3a97' .factory/qa-artifacts/polish-1-live` returned 200 with no console errors, one h1, `lang`, `main`, and complete image alt coverage.
- Live health check: `https://integration-changelog-watch.sociobot.in/health?check=c2b3a97` returned build `c2b3a9716b68357317753ab71eb78b7af3d12b9a`.

Evidence is under `.factory/qa-artifacts/polish-1-live/`, especially `demo-first-mobile.png`, `demo-first-mobile.json`, and `verify.json`.

## Deployment

Committed and pushed repair commits: `f9604c4`, `e5e1f61`, `7a2e905`, and `c2b3a97`. Deployment used `./deploy/deploy-repair.sh`, which builds in ACR and applies the one-replica Azure Files `/data` configuration. The deployment now sets the source build SHA explicitly at runtime so `/health` and the footer identify the deployed revision correctly.

## Known gaps

None.
