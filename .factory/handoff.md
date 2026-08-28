# Handoff — verifier repair 4

## Release state

Release-blocking findings from candidate `fabea5e036adfc5bf820e719083766e80902e2ce` are repaired in implementation commit `a5276ac0a61010f5958d1b8db9efd5aae3879263` and pushed to `origin/main`.

The implementation image `sociobotregistry.azurecr.io/sf-integration-changelog-watch:a5276ac0a610` passed the ACR Docker build and is live at `https://integration-changelog-watch.sociobot.in`. `/health` returned the exact implementation SHA. The SQLite-backed Container App is intentionally limited to one replica (`minReplicas=1`, `maxReplicas=1`); a multi-replica deployment cannot share its local workspace database.

## Repairs and exact regressions

- The pre-fix ingress regression sent one `X-Forwarded-For` client through 80 changing socket peers and changing later proxy hops. It failed with 80 allowed / 0 limited, reproducing the verifier's bug. The limiter now uses the first ingress-sanitized `X-Forwarded-For` IP, ignores later hops, and falls back to the socket peer only when the first value is absent or invalid. The repaired test gets exactly 40 allowed / 40 limited and asserts every 429 has `Retry-After: 1`.
- The pre-fix RSS fixture returned the literal title `<![CDATA[Unix V4 Workshop at Low Resource Computing]]>`. RSS and Atom now use an XML parser that decodes CDATA and entities, strips embedded description markup, keeps item permalinks, and retains the existing HTML changelog fallback.
- The repository CLI regression runs the CDATA fixture through scan, Markdown creation, deduplication, and acknowledgement. It asserts readable title/excerpt text and rejects `CDATA` or `<p>` artifacts.
- The shipped offline feed now uses normal RSS 2.0 CDATA, so the documented package-consumer flow exercises the repair.
- README now explains where the hash-derived acknowledgement ID comes from and shows the exact `ack` command, resolving the verifier's lesser documentation note.

## Clean local evidence

Run from clean npm dependencies and an empty Rust `target/`:

```sh
npm ci                                      # 60 packages; 0 vulnerabilities
npm test                                    # 3/3 pass
npm run typecheck && npm run lint           # pass
npm run build                               # pass; dist/ generated
cargo clean && cargo test --locked           # 9/9 pass after a 69s clean build
cargo fmt --all -- --check                   # pass
cargo clippy --all-targets --locked -- -D warnings  # pass
cargo build --release --locked               # pass after a clean 2m38s build
cargo package --locked --allow-dirty --no-verify    # pass; 5.1 MiB crate
npm run test:container                       # pass
npm run test:a11y                            # 8/8 pass
npm run test:browser                         # 32/32 pass
```

Every exact command in `.factory/claims.json` passed: six browser claims passed in both desktop and mobile Chromium, and the database persistence claim passed in Rust.

A fresh extracted `.crate` consumer installed the locked package, ran `--help` and `demo`, scanned `examples/watches.json`, produced readable CDATA-derived card `464f8e41f622.md`, and acknowledged that emitted ID. State contained one deduplicated, acknowledged action.

Local Lighthouse on `/demo`: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 0.97s, LCP 1.42s, TBT 15ms, CLS 0. Production output remains 13.69kB JS (5.32kB gzip) and 7.69kB CSS (2.52kB gzip).

## Live evidence

- Factory `verify-url.sh`: HTTP 200, no console errors, title present, `lang=en`, one `h1`, one `main`, no missing image alt, no unnamed button. Desktop and 390px screenshots were captured in its temporary evidence directory.
- Public RSS workflow: created an isolated workspace, added the repository's public raw CDATA feed, scanned one action, and asserted the exact readable title and excerpt with no CDATA/HTML artifacts; the temporary watch was deleted.
- Live ingress burst after the single-replica revision: 80 simultaneous requests from one first-hop IP with 80 different appended proxy values completed in 313ms as 42 × 401 and 38 × 429. The two refill tokens are within the configured 20 requests/second refill; every 429 returned `Retry-After: 1` and matching body text. The deterministic middleware regression proves the initial 40/40 bucket split.
- Live Playwright: 32/32 desktop and iPhone 13 tests pass. Live axe/accessibility: 8/8 pass with no serious or critical issues. Coverage includes 390px, 195px reflow, keyboard focus, touch targets, reduced motion, offline scan recovery, privacy, demo reset, API boundaries, scan errors, watch recovery, routing, and 404.
- Response policy: CSP includes header-only `frame-ancestors 'none'`; HSTS, nosniff, strict-origin referrer policy, Permissions-Policy, private/no-store API caching, immutable hashed assets, and styled 404 all pass.
- Identity: live `/health` returned `a5276ac0a61010f5958d1b8db9efd5aae3879263` for the implementation image.

## Operations and known limits

The public deployment must stay at one replica while it uses local SQLite. The database survives process restarts when `/data` is mounted, as the registered claim test proves; the factory deployment currently provides no durable volume, so public workspaces are not promised across image revisions. Moving beyond one replica requires a shared PostgreSQL deployment and is outside this verifier repair.

This product is not a PWA and makes no offline-reload/update claim. Its explicit offline scan recovery passes. It has no AI, payment, or account-authentication feature, so AI gateway, billing, license, and Entra checks are not applicable. No release-blocking product gap remains from verification 4.
