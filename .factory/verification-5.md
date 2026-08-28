# Independent verification 5 — FAIL

Verified independently on 2026-08-28 UTC.

- Candidate commit: `4652d1a08c9a6121be620957ec9e9d843b122037`
- Live URL: `https://integration-changelog-watch.sociobot.in`
- Work order: `integration-changelog-watch-verify-5`
- Result: **FAIL — do not release**

The candidate and its static assets are deployed, and all declared claim commands pass locally after the clean install. The release still fails the acceptance contract. The live service does not give a newly created workspace a consistent view of its SQLite data, concurrent requests bypass the three-watch ceiling, and public promises remain outside the claims registry.

No product code was changed during verification.

## Mandatory gates

### First read — PASS

A cold live visit answers all three required questions in the first viewport:

- What: **“Turn vendor changes into owned actions.”**
- For whom: **“For engineers who maintain payment, auth, analytics, or messaging integrations.”**
- What to click: **“Try it with sample data”**, beside “See matched notices, owners, and checks.”

At 390 px the button is fully inside the first viewport (`y=373.94`, `241.67 × 46 px`). One click opens `/demo`, immediately showing two realistic action cards, three watches, owners, commands, and the persistent **“Demo — sample data, nothing is saved”** banner with Reset demo and Start for real.

### Declared claims — PASS locally

`.factory/claims.json` exists with seven entries. After `npm ci` from the clean candidate, every exact command passed:

| Claim | Result |
| --- | --- |
| `sample-action-cards` | PASS, 2/2 Playwright projects |
| `csv-export` | PASS, 2/2 Playwright projects |
| `demo-local` | PASS, 2/2 Playwright projects |
| `workspace-boundary` | PASS, 2/2 Playwright projects |
| `requested-scans` | PASS, 2/2 Playwright projects |
| `cli-repository-workflow` | PASS, 2/2 Playwright projects |
| `database-persistence` | PASS, 1/1 Rust test |

The first attempted claim command before dependency installation stopped at `vite: not found`; installation is the documented clean-checkout prerequisite. The complete claim run after `npm ci` passed.

The same `workspace-boundary` browser claim against the live URL failed in both desktop and mobile projects, including an isolated rerun. Workspace creation returned a token, but its next authenticated request returned 401. This is the live-state defect documented below.

### Claim inventory — FAIL

The registry does not cover all public statements a visitor can rely on:

- `/terms` says redirecting source addresses are blocked. No claim entry or tagged sandbox test asserts redirect handling.
- `/privacy` says workspaces are not visible to other workspace tokens and server records exist only inside the token-created workspace. The registered `workspace-boundary` browser test checks only unauthenticated access and a loopback URL; it never tries a second valid token.
- CLI help says the `demo` command makes no network request. The registered CLI claim test runs `scan` and `ack`, not `demo`, and does not record its requests.

Under the supplied claims contract, an unlisted or under-tested public claim is release-blocking even where implementation code or an unrelated test suggests the statement is true.

## Release-blocking defects

### Major — live workspaces lose half their authenticated requests

A fresh browser created a workspace with `POST /api/workspaces` → 201 and a 64-character token. Its parallel initial reads then split: `/api/watches` returned 200 while `/api/actions` returned 401 **“This workspace token is not active on the server.”** A private-feed request using the just-issued token returned the same 401 instead of reaching validation.

Fresh-connection quantification used one newly issued token for 24 sequential authenticated reads:

| Endpoint | 200 | 401 |
| --- | ---: | ---: |
| `/api/watches` | 7 | 5 |
| `/api/actions` | 5 | 7 |
| Total | 12 | 12 |

This is consistent with more than one live instance serving separate local SQLite files. The application contract and previous handoff require one replica while storage is local. Whatever the deployment cause, the observable result is that a user can create data and immediately be told the token is unknown.

Consequences observed live:

- Full live Playwright: 30/32 passed; both workspace-boundary projects failed.
- Isolated live workspace-boundary rerun: 0/2 passed.
- Fresh `/`, `/privacy`, and `/terms` loads produced 401 resource errors in the browser console.
- The factory `/opt/fleet/lib/verify-url.sh` check returned HTTP 200, then failed because cold `/` did not reach network idle within 60 seconds.
- A diagnostic cold browser still had both `/api/watches` and `/api/actions` pending after 10 seconds.

This breaks the normal hosted product and the no-console-errors requirement. A one-connection API smoke happened to complete successfully, demonstrating that the code path works only when requests land on consistent state: create 201, invalid input 400, valid watch 201, scan 200 with one readable action, acknowledge 200, rescan with zero duplicates, and delete 204.

### Major — concurrent creates bypass the three-watch limit

Against a fresh local candidate server and database, ten simultaneous valid `POST /api/watches` requests in one workspace produced:

```text
201 Created: 6
409 Conflict: 4
stored watches: 6
```

Sequential behavior is correct: exactly three watches are accepted, the fourth returns 409, and deleting one permits a replacement. The concurrent check-then-insert is not atomic, so the visible `3/3` boundary and free-tier resource limit are unenforced under ordinary request concurrency.

Ten concurrent scans of one watch did behave safely: all returned 200 and exactly one action remained after deduplication.

## Other findings

### Moderate — legal-page visits create backend state

A fresh visit to either `/privacy` or `/terms` made `POST /api/workspaces`, `GET /api/watches`, and `GET /api/actions`, then stored `icw:workspace-token` in local storage. Merely reading legal information should not provision server state. On the current live deployment one read returned 200 and the other 401, adding a console error to each page.

### Minor — two mobile touch targets are narrower than 44 px

At 390 px, the navigation **Demo** link measures `42.8 × 44 px` and the footer **Terms** link `40 × 44 px`. This misses the attached accessibility requirement that touch targets be at least `44 × 44 px`. The skip link measures `141.5 × 44 px` when focused and is not part of this finding.

## Passing local evidence

| Check | Result |
| --- | --- |
| `npm ci` | PASS; 60 packages, 0 npm audit findings |
| `npm test` | PASS, 3/3 |
| `npm run typecheck` | PASS |
| `npm run lint` | PASS |
| `npm run build` | PASS; `dist/` generated |
| `cargo fmt --all -- --check` | PASS |
| `cargo test --locked` | PASS, 9/9 |
| `cargo clippy --all-targets --locked -- -D warnings` | PASS |
| `npm run test:container` | PASS; locked optimized Rust build |
| `npm run test:a11y` | PASS, 8/8 |
| `npm run test:browser` | PASS, 32/32 desktop/mobile |
| `cargo package --locked --allow-dirty --no-verify` | PASS; 79 files, 5,370,317-byte crate |

The release binary starts with only `PORT` and `PATH`; `/health` returns `{"build":"dev","ok":true}` and startup logs accurately say the database was defaulted. With a supplied temporary SQLite path, a watch survived a real process stop/start. A second workspace saw no first-workspace watches. Field boundaries accepted a 120-character vendor and rejected 121; the sequential fourth watch returned 409 and delete/replacement recovery worked.

Docker, Podman, and Buildah are unavailable in this verifier image, so the Dockerfile could not be executed. The exact frontend and locked release backend builds passed, and the Dockerfile was inspected for the required current-stable Rust builder, build argument, non-root runtime, port, and absence of `.git` use.

## Clean CLI consumer — PASS

The `.crate` archive was unpacked and installed into a clean root with an empty Cargo home. The installed executable passed `--help` and `demo`. Using only the packaged examples:

- first `scan` created `464f8e41f622.md`;
- second `scan` created no duplicate;
- the card contained readable CDATA-decoded title and excerpt text;
- `ack --id 464f8e41f622` persisted `acknowledged: true`.

## Accessibility, privacy, routing, and performance

- Live axe at desktop and 390 px: zero violations, including zero serious/critical findings.
- Semantics: `lang=en`, one `<h1>`, one `<main>`, no missing image alt, ordered headings, and independent route titles.
- Keyboard: first Tab exposes a 3 px dashed skip-link focus ring; Enter focuses `#main`; Space acknowledges the demo card and focus moves to that card.
- Reflow: no horizontal overflow at 390 px or at the 195 px 200%-equivalent viewport.
- Reduced motion: transition and animation durations are `0s`; scroll behavior is `auto`.
- Direct `/demo` request log: document, same-origin hashed JS/CSS, and same-origin hero image only. No API, analytics, ad, remote-font, cookie, or third-party request occurred. Demo storage remained separate from the real workspace.
- Security headers: restrictive CSP with header-only `frame-ancestors 'none'`, HSTS, `nosniff`, strict-origin referrer policy, and restrictive Permissions-Policy.
- Cache policy: HTML `no-cache`; hashed JS/CSS one-year immutable; hero one week; API `no-store, private`.
- Local production sizes: JS 13,686 bytes raw / 5.32 kB gzip; CSS 7,690 bytes raw / 2.52 kB gzip; hero WebP 58,974 bytes.
- Lighthouse mobile `/demo`: Performance 99, Accessibility 100, Best Practices 100, SEO 100; FCP 1.0 s, LCP 1.3 s, TBT 120 ms, CLS 0; total transfer 81 KiB.
- `/`, `/demo`, `/privacy`, `/terms`, metadata assets, Stripe sample notice, and Auth0 sample notice returned 200. The styled unknown route returned 404.
- Visual inspection found no obvious artifact in the original paper-cut hero and confirmed responsive stacking.

The product is not a PWA and makes no offline-reload claim, so service-worker update/offline reload is not applicable. It has no sign-in, AI runtime feature, paid tier, or unlock call; Entra, AI gateway, and Sociobot billing checks are not applicable.

## Rate limiting

A live burst of 80 concurrent unauthenticated API requests used one fixed first `X-Forwarded-For` hop and varied later proxy hops. In 474 ms it returned 45 × 401 and 35 × 429. Every 429 included `Retry-After: 1` and the matching body **“Too many requests. Try again in 1 second(s).”** The observed allowance was 45 during the sub-second burst, consistent with the configured burst of 40 plus 20 requests/second refill. The local deterministic test passed exactly 40 allowed / 40 limited.

## Deployment identity

Live `/health` returned:

```json
{"build":"4652d1a08c9a6121be620957ec9e9d843b122037","ok":true}
```

The live root HTML, hashed JS, hashed CSS, and hero image match the local candidate byte-for-byte:

| File | SHA-256 |
| --- | --- |
| `index.html` | `e6fcf610287cd418070034b8cd9e1aba4d643a4da8bf9b2d22052d10913ef927` |
| `assets/index-C8qgZKOV.js` | `53026161976daa1a6ce997e42c7b9079373b2c5ff76f560aa7aa8b35296fd721` |
| `assets/index-Bc4ZUE6J.css` | `c704d4ef7cf136c3bad2b0e084bf1109f35425790a1cae27faeecbdceb579492` |
| `paper-cut-hero.webp` | `fb0d415bec60a017de28d29ee05795fbf156fc3ab069dab80d3bd7dce49a252b` |

## Required repairs

1. Keep the SQLite deployment at exactly one serving replica with durable `/data`, or move workspace state to a shared database before using multiple replicas. Re-run the fresh-token 24-request consistency check and the live browser suite.
2. Make the three-watch limit atomic in the database/transaction layer and add a concurrent integration test.
3. Register and tag tests for every remaining public claim, especially cross-token isolation, redirect rejection, and the CLI demo's no-network statement, or remove those statements.
4. Do not hydrate/create workspaces on legal-only routes, and make all mobile targets at least `44 × 44 px`.

## Release decision

**FAIL.** The exact candidate is live, but its hosted workspace is not reliably usable and multiple mandatory contract gates remain violated.
