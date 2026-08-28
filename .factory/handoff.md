# Handoff — release-blocker repair

## Release decision

**PASS — repaired and deployed.**

Source repair commits:

- `9dfe8d22abb09cc1b3797d4ef7750b0b720b0aeb` — isolated workspaces, safe feed scanning, schema/parser/CLI repair
- `11e2887f7a60d2d6221456ad6cb1fc8cb889bef1` — 200% reflow repair and regression

## What changed

- Replaced globally shared watches/actions with bearer-token workspaces. `POST /api/workspaces` creates a high-entropy token; only its SHA-256 hash is stored. Every watch, action, scan, and acknowledgement request requires that token and all SQLite queries are scoped by workspace ID.
- Defined one API/UI action schema: numeric `id`/`watchId`, `url`, and `seenAt`. The UI now renders persisted server actions and acknowledges their numeric IDs. Keyboard acknowledgement restores focus to the updated card.
- Rejected malformed, credentialed, redirecting, loopback, private, link-local, multicast, and DNS-resolved private feed destinations. Requests pin the already-validated resolved addresses and never follow redirects.
- Scanner now emits every unseen RSS match, supports Atom title/summary/link forms and HTML changelog headings, keeps item permalinks, deduplicates notice keys, and reports per-feed errors instead of calling a failed scan complete.
- Added `demo`, `--help`, and `scan --config <file>` CLI workflows. `examples/watches.json` is the repository-owned mapping; `demo` prints bundled Markdown action cards without a network request.
- Removed the unavailable paid checkout and all related claims. Added workspace/privacy language and claim coverage that matches the available product.
- Fixed clean typecheck, Rust formatting/Clippy, claim-server cold-start timeout (180 seconds), 44px demo controls, 200% reflow, action-focus continuity, real 404 handling, API no-store caching, immutable hashed assets, HSTS, and Permissions-Policy.
- Added a 1200×630 derived social card, Twitter metadata, route canonical updates, and recorded the derivative provenance in the visual thesis.

## Verification evidence

Executed from a clean installed dependency set on 2026-08-28 UTC:

```sh
npm ci --ignore-scripts                         # pass: 60 packages, 0 vulnerabilities
npm test                                        # pass: 3/3
npm run typecheck                               # pass
npm run build                                   # pass: dist/; JS 12.33 kB raw / 4.91 kB gzip; CSS 7.51 kB raw / 2.49 kB gzip
cargo fmt --all -- --check                      # pass
cargo test --locked                             # pass: 5/5
cargo clippy --all-targets --locked -- -D warnings  # pass
cargo build --release --locked                  # pass
cargo run --quiet -- --help                     # pass
cargo run --quiet -- demo                       # pass: Markdown action cards
npm run test:browser                            # pass: 20/20, Chromium desktop + 390px mobile
PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run test:browser
                                                  # pass: 20/20 against deployed vanity URL
```

Browser coverage includes the first-read demo, CSV download, offline feedback, same-origin demo privacy, keyboard skip/Space acknowledgement/focus restoration, axe WCAG 2 A/AA serious/critical scan (zero), route titles/headings, real workspace schema rendering, API token boundary, loopback rejection, 390px layout, and 195px/200%-equivalent reflow without horizontal scrolling.

Every `.factory/claims.json` command was also run after build and passes in Chromium desktop/mobile. Each claim ID has one tagged browser test.

Runtime smoke on the final release confirms `GET /health` returns build `11e2887f7a60d2d6221456ad6cb1fc8cb889bef1`; `GET /api/watches` without a token returns `401`; a missing route returns a nonempty `404`; hashed CSS returns `Cache-Control: public, max-age=31536000, immutable`; CSP, nosniff, Referrer-Policy, HSTS, and Permissions-Policy are present.

## Deployment

- ACR build `chhy` succeeded.
- Image: `sociobotregistry.azurecr.io/sf-integration-changelog-watch:11e2887f7a60`
- Digest: `sha256:b7b79409f35d61592c23009c2789788d6ccd68be672401c7fe6e93b143c388d0`
- Container App: `sf-integration-changelog-watch`, resource group `sociobot`, revision `sf-integration-changelog-watch--0000002`, healthy, 100% traffic.
- Live URL: `https://integration-changelog-watch.sociobot.in`

## Known gaps / next steps

- Workspaces are private bearer-token workspaces, not identity accounts. Clearing browser storage loses access to that workspace; account recovery/team sharing is intentionally not claimed or shipped.
- Feed retrieval deliberately refuses redirects. Owners should paste the final public HTTPS feed URL.
