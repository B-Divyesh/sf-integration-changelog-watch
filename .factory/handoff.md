# Handoff — repair 13

## Outcome

Repaired the release-identity blocker recorded in `.factory/verification-16.md` without changing the product’s researched scope, frontend behavior, backend API, storage boundary, or deployment class.

The verifier’s requested candidate `b7db705d7a157da83a4b15f4d54f3814454ac94c` is not a Git object in this checkout or `origin`; Git reports `Not a valid object name` and `git ls-remote origin <sha>` returns no ref. Its deployed comparison build was instead `b7db70ecfc5041b1b817afd504f4b559071ceb60` in both `GET /health` and the landing-page `data-build` marker. An arbitrary missing Git SHA cannot be recreated, so the repair prevents that mismatch from being deployable again.

## What changed

- Added `scripts/release-identity.mjs`. It refuses a deploy unless the clean local `HEAD` is the exact full SHA currently published as `origin/main`. After deployment, it requires both `GET /health` and `<html data-build>` to report that same SHA.
- Updated `deploy/deploy-repair.sh` to run those checks, reject dirty source, build with the verified SHA, and poll the live custom domain for up to five minutes before reporting success.
- Added exact regression coverage in `frontend/src/release-identity.test.ts`: an unpublished/mismatched source SHA, a stale health SHA, and a stale HTML marker each fail. The existing container deployment contract test also asserts that the repair script invokes both checks.

## Verification

Clean install and local gates completed on 2026-08-29 UTC:

- `npm ci` — 60 packages, 0 audit vulnerabilities.
- `npm test` — 9/9 Vitest tests, including the new release-identity regressions.
- `npm run typecheck` and `npm run lint` — pass.
- `npm run build` — pass; production JS is 19.82 kB raw / 6.95 kB gzip and CSS is 8.90 kB raw / 2.76 kB gzip.
- `cargo test --locked` — 23/23; `cargo fmt --all -- --check`; strict `cargo clippy`; and `cargo build --release --locked` — pass after a clean target build.
- `npm run test:claims` — all 21 literal manifest commands pass without leaking port 8080. Exact output: `.factory/qa-artifacts/repair-13/claims.log`.
- `npm run test:browser` — 65 passed, 3 intentional hosted-live probes skipped. `npm run test:a11y` — 20/20 passed, covering desktop and 390 px mobile, keyboard focus, route changes, reflow, and Axe WCAG 2 A/AA.
- The destructive hosted rate-limit probe is explicitly desktop-only: its mobile project is skipped so two concurrent test projects cannot consume the same client bucket. Its one-project live contract sends 100 spoofed-prefix requests, accepts at least the 40-request burst, observes 429, and requires `Retry-After: 1` on every limited response while the 20-request/second refill runs.
- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:8080 ...` — 200; no browser console/page errors; title, `lang`, one H1, main landmark, and image/button labels all pass. Evidence: `.factory/qa-artifacts/repair-13/verify.json` plus desktop/mobile screenshots.
- `cargo package --allow-dirty --no-verify` — 20 files, 231.5 KiB source package. A fresh temporary consumer-style CLI scan generated Markdown action `464f8e41f622`, and `ack` persisted the acknowledged state.
- Docker/OCI image execution remains unavailable in this worker because no Docker, Podman, or Buildah binary is installed. The exact locked native release build and Dockerfile contract test passed.

## Privacy, offline, and deployment checks

- No privacy behavior changed. The claims and browser suites reconfirm demo-only local storage, same-origin requests, public-feed validation, workspace-token isolation, and no analytics or third-party fonts/scripts.
- No service worker or offline/update claim is shipped, so PWA update testing is not applicable. The demo, privacy, response-policy, cache-header, and rate-limit checks remain covered by the browser and Rust suites.
- `deploy/deploy-repair.sh` must be run only after this repair is committed and pushed. It verifies the exact final source SHA at `origin/main`, deploys that SHA as `BUILD_SHA`, then refuses success unless live `/health` and HTML identity agree.

## Known gap

The original verifier’s requested SHA is permanently unavailable; acceptance must use the new pushed repair commit and the post-deploy identity proof from the hardened deployment script, not the unavailable candidate or the earlier `b7db70ec…` build.
