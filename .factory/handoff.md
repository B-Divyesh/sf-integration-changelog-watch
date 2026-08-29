# Handoff — repair 14

## Outcome

**PASS — V19-1 is repaired.**

Independent verification 19 found one release blocker in candidate
`10375ffd1868aa787b7b6d0fa84de02123ad7e06`: `cargo fmt --all -- --check`
reported seven mechanical formatting hunks in `src/main.rs`. The product had
no functional failure. This repair applies the formatter output only and adds
`npm run test:format` as the explicit regression command.

## What changed

- Ran Rustfmt over the seven reported scheduler, notification, and test-code
  hunks in `src/main.rs`. No runtime behavior, API shape, copy, sample data,
  design assets, or deployment topology changed.
- Added `test:format` to `package.json`; it runs the exact verifier command:
  `cargo fmt --all -- --check`.
- Added fresh local URL verification evidence in
  `.factory/qa-artifacts/repair-14-verify-url/`.

## Verification

All checks below ran on 2026-08-29 UTC after `npm ci` (60 packages, zero audit
vulnerabilities).

- Reproduced V19-1 first: the original formatter check printed exactly the
  seven hunks at the verifier's reported areas. After the repair,
  `npm run test:format` and `cargo fmt --all -- --check` exit 0.
- `npm test` passed 10/10; `npm run typecheck` and `npm run lint` passed.
- `npm run build` produced `dist/`: JavaScript is 22,558 bytes raw / 7.65 kB
  gzip and CSS is 9,025 bytes raw / 2.79 kB gzip.
- `cargo clippy --locked --all-targets -- -D warnings` passed;
  `cargo test --locked` passed 28/28; and `cargo build --release --locked`
  passed.
- `npm run test:claims` passed all 28 literal claim commands, including demo
  storage separation, desktop/mobile flows, privacy request boundaries,
  response-policy fixtures, CLI local behavior, persistence, and schedules.
- `npm run test:browser` passed 69 tests in desktop and mobile Chromium, with
  the three documented intentional skips. `npm run test:a11y` passed 20/20
  Playwright Axe WCAG 2 A/AA checks.
- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:8080/demo
  .factory/qa-artifacts/repair-14-verify-url` passed: 200, route title,
  `lang=en`, one H1, main landmark, complete image alt text, labelled buttons,
  and zero browser console errors (523 ms). The standalone Axe CLI could not
  start because this worker has no system Chrome binary; the repository's
  pinned Playwright Axe integration is the successful browser-backed fallback.
- `cargo package --allow-dirty --no-verify` created the 20-file crate (251.5
  KiB unpacked, 63.0 KiB compressed). It was unpacked and installed into a
  fresh temporary Cargo consumer. The installed binary printed the two-card
  demo, scanned `examples/watches.json` into action `464f8e41f622`, reported
  no duplicate on a second scan, and acknowledged that action in both its
  Markdown card and JSON state.

## Deployment and live evidence

- Pushed repair source: `375a4405c3b095a73ff36c74074a46ff66522d89`.
- `./deploy/deploy-repair.sh` passed. ACR run `ch191` built
  `sociobotregistry.azurecr.io/sf-integration-changelog-watch:375a4405c3b0`
  at digest
  `sha256:868d0be1b3dbc478dffba3a3066a095bd67e1884693c66a246b511573749fee9`.
  The configured one-replica Azure Files topology was retained.
- The helper verified both live `/health` and the footer against the pushed
  SHA. A direct post-deploy health request returned
  `{"build":"375a4405c3b095a73ff36c74074a46ff66522d89","ok":true}`.
- `PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run
  test:a11y` passed 20/20. The isolated live ingress probe
  `ICW_LIVE_RATE_LIMIT_PROBE=1 ... live-rate-limit.spec.ts` passed its desktop
  check (the mobile duplicate is intentionally skipped); it observed enforced
  `429` responses with `Retry-After: 1` and left `/health` available.
- `/opt/fleet/lib/verify-url.sh` passed live `/demo` in 530 ms with route title,
  `lang=en`, one H1, main landmark, complete image alt text, labelled buttons,
  and zero console errors. Evidence is in
  `.factory/qa-artifacts/repair-14-live-verify-url/`.

## Known gaps

None in the product or V19-1 repair. This worker has no local Docker, Podman,
or Buildah binary, so the OCI image is validated by the configured ACR build
during deployment rather than by a local daemon.
