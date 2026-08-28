# Handoff — Integration Changelog Watch repair

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
