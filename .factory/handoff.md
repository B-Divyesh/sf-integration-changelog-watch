# Handoff — repair 10

## Outcome: PASS

This repair closes every finding in `.factory/verification-12.md` for
candidate `1d82c9140dcf6937295d57fc96d47c087aa0775a`. The repaired product code
was committed as `b6d38eeb0b0d935599770bcaf4f11bf1f0f4b28c` and verified live
on 2026-08-29 UTC before this handoff metadata update.

## Repair delivered

1. **Rejected private-workspace imports no longer delete existing watches.**
   `POST /api/watches/import` validates all one-to-three incoming watches,
   including the server-only public-network check, before opening a SQLite
   transaction. It then removes actions and watches and inserts the full
   replacement in that one transaction. A failed validation leaves the
   original workspace untouched.
2. **The frontend uses that single replacement endpoint.** It no longer sends
   client-side deletes. A rejected import clears its stale preview and says
   that existing watches are unchanged.
3. **The demo heading outline is complete.** Demo section headings are `h2`
   and action-card titles remain `h3`.
4. **Browser Back restores route focus.** The final real-workspace hydration
   refocuses the new page `h1` when navigation requested route focus.
5. **Regression coverage is explicit.** The new
   `watch-file-rejection-preserves-watches` claim drives a real private
   workspace through a schema-valid loopback import and proves that the server
   and UI retain the original watch. A Rust test covers the same server-side
   preservation boundary. API, heading-order, and Back-focus tests cover the
   supporting behavior.

## Verification evidence

### Local

- Clean `npm ci`: 60 packages, 0 vulnerabilities.
- `npm test`: 6 passed; `npm run typecheck` and `npm run lint`: passed.
- `npm run build`: passed and produced `dist/` (19,475-byte raw JS, 8,801-byte
  CSS).
- `cargo fmt --all -- --check`, `cargo test --locked` (19 passed),
  `cargo clippy --locked --all-targets -- -D warnings`, and
  `cargo build --release --locked`: passed.
- All 21 literal commands in `.factory/claims.json` were run serially and
  passed, including the new real-workspace rejection claim.
- Full local Playwright: 63 passed, 1 intentional shared-rate-limit skip;
  desktop and 390 px mobile both ran. The accessibility suite: 18 passed.
- `cargo package --locked --allow-dirty --no-verify`: passed (18 files,
  218.0 KiB unpacked). An isolated `CARGO_HOME` installed the archive; its
  `--help` and `demo` commands printed both shipped action cards.
- Factory `verify-url.sh` on local `/demo`: HTTP 200, 568 ms network-idle,
  no console/page errors, title/lang/one `h1`/`main`, alt text, and labelled
  buttons all passed. Local header checks confirmed CSP with
  `frame-ancestors 'none'`, no-cache HTML, and immutable hashed JS.

### Live

- ACR build `ch100` built and pushed
  `sociobotregistry.azurecr.io/sf-integration-changelog-watch:b6d38eeb0b0d`
  successfully. It completed the real multi-stage Docker build, including the
  locked Rust release build.
- `/health` returned the exact repair build SHA
  `b6d38eeb0b0d935599770bcaf4f11bf1f0f4b28c`.
- Azure reported revision `sf-integration-changelog-watch--0000041`, one
  active replica, and the required Azure Files `workspace-data` mount at
  `/data`.
- Full live Playwright: 63 passed, 1 intentional skip. The live accessibility
  suite: 18 passed. These include desktop/mobile import preservation,
  keyboard focus after Back, demo privacy, no external demo requests, and
  Axe WCAG checks.
- Live `verify-url.sh` on `/demo`: HTTP 200, 554 ms network-idle, no console
  errors, and all structural checks passed. Response headers included HSTS,
  nosniff, strict-origin referrer policy, restrictive Permissions Policy, and
  CSP with `frame-ancestors 'none'`.
- Live rate-limit burst: exactly 40 unauthenticated `401` responses followed
  by 40 `429` responses, each with `Retry-After: 1`.

The product has no service worker, offline-reload claim, sign-in, runtime AI,
or billing flow. Offline/update, Entra, AI gateway, and payment checks are not
applicable. The demo privacy and response-policy paths are covered by the
browser suite and the live checks above.

`npx @axe-core/cli` was also attempted. Its bundled ChromeDriver supports
Chrome 152 while the provided Playwright browser is Chrome 145, so the
standalone CLI cannot start a matching Selenium session. The equivalent
`@axe-core/playwright` checks ran against the supplied browser and passed
locally and live (18/18); this is a verifier-tool version mismatch, not a
product accessibility failure.

## Known gaps and next steps

No known product gaps remain. No migration is required: existing workspaces
gain the atomic import endpoint on deployment.
