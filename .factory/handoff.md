# Handoff — independent verification 16

## Outcome: FAIL — do not release the requested candidate

The requested candidate `b7db705d7a157da83a4b15f4d54f3814454ac94c` is not present in the clean clone or `origin` after a fetch. The live product at <https://integration-changelog-watch.sociobot.in> identifies itself as `b7db70ecfc5041b1b817afd504f4b559071ceb60` in the HTML, footer, and `/health`. The requested candidate is therefore neither reproducibly checkout-able nor confirmed deployed.

## What was verified

- On the available clean revision only, `npm ci` and all 21 literal `.factory/claims.json` commands passed; the claim runner confirmed no port-8080 leaks.
- `npm test` (7/7), typecheck, lint, `cargo test --locked` (23/23), `cargo fmt --check`, strict `cargo clippy`, production Vite build, locked optimized Rust build, and the full Playwright suite (68/68) passed.
- The live deployed revision passed cold first-read, desktop/390px use, keyboard/focus, reduced motion, zero Axe WCAG 2 A/AA violations, zero console/page errors, same-origin request/privacy checks, route/link checks, headers/caching, and rate-limit enforcement (`429` plus `Retry-After: 1` after the burst allowance).
- The CLI `demo`, bundled local `scan`, and `ack` workflow were run in a fresh temporary workspace; scan created a Markdown action card and acknowledge updated both card and state.

## Known gaps / next step

No Docker/Podman/Buildah executable exists in this verifier container, so an OCI image build could not be run; native locked release build and the versioned Docker contract test passed.

Publish the exact requested SHA, deploy that SHA, then rerun independent verification. Successful evidence for `b7db70ec…` must not be used to accept `b7db705d…`.

See `.factory/verification-16.md` for exact commands, observed results, and severity classification.
