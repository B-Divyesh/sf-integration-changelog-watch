# Handoff — repair 9

## Outcome

The release-blocking verification 11 failures are repaired. The original
web-with-backend artifact, one-click demo, privacy boundary, and previously
passing product behavior were preserved.

## Repairs

- Reproduced the verifier command exactly after `npm ci`:
  `npm test -- --grep @claim:container-build-stage` exited 1 with Vitest
  3.2.7's `CACError: Unknown option --grep`.
- Replaced it with Vitest's supported, fully anchored `--testNamePattern`.
  It selects the one container-build assertion.
- Added a unit regression test that requires the exact supported manifest
  command and rejects `--grep` for this claim.
- Applied `cargo fmt` to `src/main.rs`; `cargo fmt --all -- --check` passes.
- Excluded `.factory` evidence, source imagery, frontend/tooling, and other
  non-runtime artifacts from Cargo packaging. The crate is now 18 files,
  213.4 KiB unpacked / 55.9 KiB compressed (the verifier measured 11.6 MB).

## Local verification

Run from a clean `npm ci` installation (60 packages, zero vulnerabilities):

- All 20 literal commands in `.factory/claims.json`: PASS. The repaired
  `container-build-stage` command runs one matching Vitest test; the recorded
  sweep ends with `ALL_CLAIMS_PASSED=20`.
- `npm test`: PASS, 6 tests.
- `npm run typecheck`, `npm run lint`, `npm run build`: PASS. Production
  output is `dist/`; JS is 19.46 kB raw / 6.78 kB gzip and CSS is 8.80 kB raw
  / 2.73 kB gzip.
- `cargo fmt --all -- --check`: PASS.
- `cargo test --locked`: PASS, 18 tests.
- `cargo clippy --locked --all-targets -- -D warnings`: PASS.
- `cargo build --release --locked`: PASS.
- `npm run test:browser`: PASS in desktop Chromium and 390 px mobile.
- `npm run test:a11y`: PASS in both projects; its Axe coverage checks demo,
  route metadata, keyboard skip-link flow, touch targets, reflow, and 404.
- `cargo package --locked --allow-dirty --no-verify`: PASS. A fresh Cargo-home
  install of the extracted crate passed `--help`, `demo`, and
  `scan --config watches.json`, creating a Markdown action card from the
  bundled local feed.

The local environment has no `docker` executable. The configured deployment
uses `az acr build` from this Dockerfile with the three source-SHA build args;
the remote build is the authoritative container build gate. This product has
no service worker, payment, account, or runtime AI feature, so update,
billing, identity-provider, and AI-gateway checks are not applicable.

## Deployment and live evidence

The repair is deployed with `deploy/deploy-repair.sh`, which preserves the
single durable Azure Files `/data` replica and passes `BUILD_SHA`. Post-deploy
live health, identity, browser, accessibility, privacy, response-policy, and
rate-limit results are recorded after deployment.

## Known gaps

None in the repaired product. The local Docker CLI is absent as noted above.
