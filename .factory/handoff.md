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

The repair commit `89dbd2aa26060d5070293173554e10eeefda72c0` was pushed to
`origin/main` and deployed with `deploy/deploy-repair.sh`. ACR build `chxf`
succeeded. The Container App's latest revision is healthy, has 100% traffic,
and remains configured with `minReplicas: 1`, `maxReplicas: 1`, and the durable
Azure Files `/data` mount.

- Live `GET /health`: `200` with build
  `89dbd2aa26060d5070293173554e10eeefda72c0`; the live HTML `data-build`
  marker matches it.
- Live `PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in
  npm run test:browser`: PASS, 59 passed / 1 intentional project skip.
- Live `npm run test:a11y`: PASS, 16 tests across desktop and mobile with Axe
  coverage, keyboard skip-link flow, route metadata, touch targets, reflow,
  and 404 coverage.
- Demo privacy path loads only same-origin document, hashed JS, and hashed CSS;
  no third-party script, font, or runtime AI origin is present.
- Live `/demo` responses have self-only CSP including header-delivered
  `frame-ancestors 'none'`, HSTS, `nosniff`, strict-origin referrer policy,
  permissions policy, and no-cache HTML. `/privacy` and `/terms` return 200;
  an unknown route returns the styled 404.
- A direct 100-request unauthenticated API burst returned 80 × 401 and
  20 × 429; every 429 included `Retry-After: 1`.

## Known gaps

None in the repaired product. The local Docker CLI is absent as noted above.
