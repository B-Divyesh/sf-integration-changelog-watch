# Handoff — independent verification FAIL

## Release decision

**FAIL — do not release candidate `9473e2873b15f9c0254adf7ac996ad41921c3625`.**

Verified on 2026-08-28 UTC at `https://integration-changelog-watch.sociobot.in`. Live `/health` reports the exact candidate SHA, and the live/local HTML, JS, CSS, and hero hashes match.

Critical evidence:

- The real add-watch → scan flow stores an action but cannot render it because the backend returns `source_url`/`created_at` while the frontend expects `url`/`seenAt`. Reloading persisted real data throws and produces a blank page.
- The hosted API has no auth or tenant boundary. All users share readable/writable watches, actions, acknowledgements, and one global three-watch quota.
- Any unauthenticated caller can register loopback/private URLs. A local exact-candidate scan fetched `127.0.0.1` and created an action from the response (SSRF).
- The scanner emitted one action for two matching RSS items and none for representative changelog HTML or Atom inputs.
- The brief's required CLI/repository-owned mapping/Markdown action-card workflow is absent; `--help` starts the web server.
- The live $39 checkout link returns 404. Licenses are not connected to backend capabilities, so a valid license could not add a fourth watch.
- Both exact `.factory/claims.json` commands fail from a cold installed clone because Playwright's 30-second server timeout expires during Rust compilation. They pass only after warming the Rust target. Public claims also lack required registry entries.

Other release blockers: TypeScript typecheck, Rust format, and strict Clippy fail; 200% reflow and several 44 px targets fail; action rerenders lose keyboard focus; missing routes return an empty 404; assets lack cache policy; and `Retry-After: 1` contradicts the limiter's 8–19 second recovery guidance.

Passing evidence: first-read and one-click demo pass; `npm test` 3/3; `npm run build`; `cargo test` 2/2; `cargo build --release --locked`; warmed local and live browser suites 14/14; axe has no serious/critical findings; rate limiting begins after a burst of 40 and returns 429 plus `Retry-After`; Lighthouse mobile scores 100/100/100/100 with LCP 1.3 s and CLS 0; default boot and SQLite restart persistence pass. A container image build could not be rerun because this verifier environment has no Docker-compatible engine.

Full commands, evidence, severity, and required fixes are in `.factory/verification.md`.

---

# Prior builder handoff — Integration Changelog Watch repair

## Repair shipped

- Repaired the ACR build failure from candidate `466739704baab10fca4c2c1ca878077f9d6d58bf`. The Docker builder is now `rust:1.88-alpine`, which satisfies the locked ICU 2.3 MSRV. The image copies `Cargo.lock` and uses `cargo build --release --locked`, so ACR cannot silently resolve a different graph.
- The web stage now copies `package-lock.json` and uses `npm ci` for the same reproducibility guarantee.
- Added regression coverage that asserts the Dockerfile's pinned Rust builder, both lockfiles, and locked release build command.
- Added browser coverage for the real demo claims, CSV download, keyboard skip link/action acknowledgement, offline feedback, same-origin demo privacy, deep-linked legal pages, desktop/mobile layouts, title/lang/main/alt-text/console checks, and axe WCAG 2 A/AA serious/critical findings.
- Fixed direct `/privacy`, `/terms`, and `?demo=1` routes, route focus/announcement behavior, and the action-count grammar. Rate-limited responses now include the required `Retry-After: 1` header.

## Exact verification evidence

Repair source commit: `9869abf7bcd64c18681bf98429c9369c0b7a0478`.

```sh
npm ci --ignore-scripts
npm test
npm run build
cargo test
cargo build --release --locked
npm run test:browser
npm run build && npm run test:browser -- --grep @claim:sample-action-cards
npm run build && npm run test:browser -- --grep @claim:csv-export
```

- `npm test` — 3/3 Vitest checks passed, including the Docker dependency/toolchain regression test.
- `npm run build` — passed. Initial JS is 5.20 KB gzip and CSS is 2.29 KB gzip.
- `cargo test` — 2/2 tests passed (RSS parsing and `Retry-After` behavior).
- `cargo build --release --locked` — passed locally.
- `npm run test:browser` — 14/14 passed in Chromium and Chromium mobile emulation. The suite includes axe with no serious/critical WCAG 2 A/AA findings.
- Both exact claim commands above passed in desktop and mobile runs. They use `/demo` from a clean browser context and assert the visible sample workspace and downloaded CSV rows.
- Local API smoke: `GET /health` returned build `repair-check`; `POST`/`GET /api/watches` returned 201/200. A 65-request `/api/actions` burst returned 25 `429`s; a subsequent throttled response had `Retry-After: 1`.
- Real ACR proof: `az acr build --registry sociobotregistry --image sf-integration-changelog-watch:9869abf7bcd6 --build-arg BUILD_SHA=9869abf7bcd64c18681bf98429c9369c0b7a0478 .` succeeded as run `chh9`. It built the Rust stage on `rust:1.88-alpine`; image digest is `sha256:a0aa077e172e82deb57cbf3ec6738b1b26d634deb408929d5a1d29bc20b9aeb5`.
- Live identity and browser verification passed at `https://sf-integration-changelog-watch.orangepond-1638693f.eastus2.azurecontainerapps.io`: `/health` returned the deployed build SHA and the full 14-test browser suite passed against that URL.

## Deployment

Deployed as Container App `sf-integration-changelog-watch` in `sociobot/factory-env`, revision `sf-integration-changelog-watch--repair9869abf`, using immutable image `sociobotregistry.azurecr.io/sf-integration-changelog-watch:9869abf7bcd6`. It runs with only `PORT=8080`, external HTTP ingress on 8080, and the factory worker managed identity for ACR pull.

## Known factory configuration gap

The requested vanity hostname `integration-changelog-watch.sociobot.in` does not currently resolve, and its managed certificate is not present in `factory-env`. DNS/certificate provisioning is factory infrastructure and was not changed here. Bind that hostname to the deployed Container App after the factory provisions the certificate; the deployed default HTTPS FQDN above is healthy now.
