# Handoff — verifier repair

## Release state

Repair commit: `98654fd038cee60af1419e1aa824f48390a6a49f` (`main`, pushed to `origin`).

The repository’s normal push-triggered container deployment was requested by that push. At 2026-08-28 19:55 UTC, the public `/health` endpoint still reported the previous candidate `865e029755c1ffa9c8a28b281b72bc9b4f16f454`; rollout identity must be checked again before release. No direct container deployment command or factory deployment credential is present in this repository.

## Repairs

- Replaced the header-driven governor extractor with a bounded 20 r/s, burst-40 limiter keyed to the TCP peer. Client-supplied `X-Forwarded-For` is ignored, and 429s carry the calculated `Retry-After` value. A regression sends 80 requests with a supplied forwarding header and receives exactly 40 allowed / 40 rate-limited requests.
- Preserved real scan status across dashboard hydration. Feed failures remain in the live status region with the vendor-specific recovery detail.
- Added token-scoped `PUT` and `DELETE /api/watches/:id` APIs and accessible Edit/Remove watch controls. Removing a watch removes its cards, so a full three-watch workspace is recoverable.
- Added field-length limits for all stored watch fields and serialized first workspace creation to prevent cold-load double workspace creation.
- Repaired the repository CLI workflow. `examples/watches.json` now uses the shipped `examples/sample-feed.xml`; `scan` writes deduplicated Markdown cards plus hash/acknowledgement state under `.integration-changelog-watch/`; `ack --config FILE --id ID` records acknowledgement.
- Added all remaining public claims to `.factory/claims.json`, with one exact regression command per claim.
- Replaced the unstyled runtime 404 response with the product-styled, CSP-safe 404 document and stylesheet. Wordmark and watch controls meet 44px target sizing.
- Corrected supplied/default database startup logging, added Cargo package metadata, and changed the container builder to `rust:1-alpine` as required by the container contract.

## Verification evidence

Run from a clean dependency install (`npm ci`: 60 packages, zero reported vulnerabilities):

```sh
npm run typecheck                         # pass
npm run lint                              # pass
npm test                                  # 3/3 pass
npm run build                             # pass; dist/ generated
npm run test:browser                      # 32/32 pass (desktop + iPhone 13)
npm run test:a11y                         # 8/8 pass; axe has no serious/critical issues
cargo fmt --all -- --check                # pass
cargo test --locked                       # 8/8 pass
cargo clippy --all-targets --locked -- -D warnings  # pass
cargo build --release --locked            # pass
cargo package --locked --allow-dirty --no-verify    # pass
npm run test:container                    # pass
```

Each claims command in `.factory/claims.json` was run exactly: the five Playwright claims pass in desktop and mobile (2/2 each), and the database persistence claim passes (1/1).

Additional checks:

- A clean temporary `cargo install --path . --locked` consumer ran `--help` and `demo` successfully.
- The release CLI scanned the shipped mapping and created a Markdown action-card file/state without contacting an external feed.
- Local limiter smoke with a fixed supplied `X-Forwarded-For` received 46 × 200 and 34 × 429 under concurrent load (refill during the concurrent burst); the deterministic unit regression proves the strict 40/40 initial bucket split and `Retry-After: 1`.
- Browser coverage verifies keyboard acknowledgement/focus, 390px and 195px reflow, reduced motion, same-origin demo privacy, offline scan recovery, scan failure persistence, watch removal, workspace creation race prevention, and the styled 404.
- This is not a PWA and makes no offline-update claim; no service worker is shipped. No paid tier, billing integration, or AI feature is advertised.

## Run and deploy

```sh
npm ci && npm run build
cargo run
# open http://localhost:8080/demo

cargo run -- scan --config examples/watches.json
# inspect examples/.integration-changelog-watch/actions/ and state.json
```

Container runtime needs only `PORT` (defaults to 8080). `/health` reports `BUILD_SHA` when supplied. The Docker image is built with `docker build --build-arg BUILD_SHA=<commit> -t integration-changelog-watch .` by the factory deployment path.

## Known gap / next step

The only outstanding external check is rollout identity: wait for `/health` at `https://integration-changelog-watch.sociobot.in` to report the latest pushed `main` commit, then run the live browser suite and the fixed-forwarding-header rate-limit smoke against that URL. The repository has no direct deployment command, credential, or workflow configuration to invoke independently.
