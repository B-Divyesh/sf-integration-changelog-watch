# Independent verification — FAIL

Verified on 2026-08-28 UTC.

- Candidate: `9473e2873b15f9c0254adf7ac996ad41921c3625`
- Live URL: `https://integration-changelog-watch.sociobot.in`
- Work order: `integration-changelog-watch-verify-1`
- Result: **FAIL — do not release**

The live deployment is the candidate, but the real product cannot complete its core job. A real scan stores an action in the backend, then the frontend fails to render it. The public backend also has no tenant or authorization boundary and will fetch loopback/private-network URLs supplied by any caller. The advertised paid tier is unavailable and would not unlock its advertised features even if checkout worked.

## Mandatory gates

### First-read test — PASS

Cold at `/`, the first viewport says:

- What it does: “Turn vendor changes into owned actions.”
- For whom: engineers maintaining payment, auth, analytics, or messaging integrations.
- What to click first: “Try it with sample data,” followed by “See matched notices, owners, and checks.”

The action is visible at 390 px (`y=374`) and opens `/demo` in one click. The first demo render contains three watches, two action cards, owners, check commands, and the persistent “Demo — sample data, nothing is saved” banner with Reset and Start for real controls. The standard demo flow made same-origin requests only and used `demo:integration-changelog-watch`, not `icw:workspace`.

### Claims gate — FAIL

`.factory/claims.json` exists with two entries. The required clean-run chronology was:

1. Before dependency installation, each exact command stopped at `vite: not found`, as expected for the untouched clone.
2. `npm ci` passed with 58 packages and zero reported vulnerabilities.
3. Immediately after installation, both exact claim commands built the frontend and then failed with `Timed out waiting 30000ms from config.webServer.` A clean Rust build cannot start the Playwright server within its 30-second budget. No trace is produced because no test starts.
4. After `cargo test --locked` warmed the Rust target, both exact commands passed with 2/2 desktop/mobile cases each. This does not satisfy the required clean-clone claim gate.

| Claim | Exact command | Clean result | Warm result |
| --- | --- | --- | --- |
| `sample-action-cards` | `npm run build && npm run test:browser -- --grep @claim:sample-action-cards` | **FAIL**, web-server timeout | 2 passed |
| `csv-export` | `npm run build && npm run test:browser -- --grep @claim:csv-export` | **FAIL**, web-server timeout | 2 passed |

The claim registry also violates the one-test-per-claim rule: each ID is tagged once in `frontend/src/sample.test.ts` and again in `tests/browser/demo.spec.ts`.

Unlisted claim-like statements are another release-blocker. They include “Feeds are checked only when requested,” “Three watches are free,” “Each matching notice becomes a Markdown-ready action card,” the hash/excerpt retention statement, and “One purchase adds hosted history, team ownership, and more than three watches.” README adds scan, persistence, and three-watch claims. Several are false in the tested product.

## Release-blocking defects

### Critical — real scan results cannot be used

Using the production frontend and backend locally, I added a valid RSS watch through the real UI and scanned it. The API returned `{"new_actions":1,...}` and persisted the action, but the dashboard still showed “No action cards yet” and zero action articles. Local storage contained the server action.

The backend returns `source_url` and `created_at`; the frontend renders `url` and `seenAt`. The resulting render exception is swallowed during hydration. Reloading with the persisted action raises `Cannot read properties of undefined (reading 'replace')`, leaves `#app` empty, and produces a blank page. Backend IDs are numbers while acknowledgement looks them up against a string `data-id`, so real acknowledgement is also incompatible.

This breaks the brief's smallest useful product: a vendor change cannot become a visible, acknowledgeable action card outside the canned demo.

### Critical — public shared data with no tenant or authorization boundary

`GET/POST /api/watches`, `GET /api/actions`, `POST /api/actions/:id`, and `POST /api/scan` require no authentication or workspace identifier. The SQLite tables have no tenant column. An unrelated caller can read all owners, check commands, and watched URLs; add watches; trigger outbound scans; and acknowledge another user's actions. The three-watch quota is global across the deployment, not per user.

The live unauthenticated endpoints returned 200. Mutation was reproduced only against the local candidate binary to avoid altering live customer state. This conflicts with the advertised hosted history/team ownership and the privacy page.

### Critical — server-side request forgery

URL validation is only `starts_with("http")`. The API accepted both `httpjunk` and `http://127.0.0.1:19090/internal`. Scanning caused the backend to request the loopback endpoint and create an action from its response; the fixture hit log recorded `/internal`. There is no scheme parser, DNS/IP allow/deny policy, redirect revalidation, or private-network block. Any unauthenticated caller can make the live service probe internal endpoints.

### Major — scanner misses announced changes

A feed fixture containing two matching RSS items produced one action, not two. The loop exits after the first matching item even when that item already exists, then hashes the whole feed, so later matches can be missed permanently.

In a separate clean database, three supported-looking inputs were scanned together:

- RSS with two matching items: one action.
- A public changelog HTML page with a matching heading: zero actions.
- A representative Atom entry using `<title type="html">`, `<summary>`, and `<link href>`: zero actions.

The backend also sets every action's source URL to the feed URL rather than the item's notice URL. Network/feed failures are silently skipped and still return `200 Scan complete. 0 new action card(s).`, leaving no actionable recovery message.

### Major — the required CLI/repository workflow is absent

The researched smallest product is a CLI with an optional dashboard and a repository-owned watch mapping that opens Markdown action cards. This repository ships only a web server/browser UI. There is no CLI command, bundled CLI demo, repository configuration format, or Markdown action-card output. Running the compiled binary with `--help` ignored the argument, started the HTTP server on port 8080, and had to be terminated after two seconds (`exit=124`). The CSV demo export does not satisfy the repository-owned CLI/Markdown workflow.

### Major — paid tier is dead and cannot unlock features

The live Buy link returns 404 with `{"error":"enabled factory product","status":404}`. License verification itself responds to an invalid token, but no product feature reads the cached valid verdict. The backend neither accepts nor verifies a license and always rejects a fourth watch with 402 once the global count reaches three. Hosted history and team ownership are not tenant-scoped features. Therefore the advertised “More watches for $39 once” purchase is both a dead link and functionally disconnected.

### Major — required clean checks do not pass

- `npx tsc --noEmit` fails: Node types are absent, library types fail, and `new FormData(e.currentTarget)` has an `EventTarget | null` type error.
- `cargo fmt --all -- --check` fails across `src/main.rs`.
- `cargo clippy --all-targets --locked -- -D warnings` fails on `clippy::possible_missing_else`.
- Both exact claim commands fail on a cold Rust target as described above.

## Other defects

### Accessibility and interaction

- Axe WCAG 2 A/AA found zero serious/critical issues on the demo at desktop and 390 px. Title, `lang`, one `main`, one `h1`, alt text, skip link, and a visible 3 px dashed focus ring passed.
- Normal 390 px layout has no horizontal overflow. At the 200% reflow equivalent (195 CSS px), `scrollWidth` is 278 and navigation, vendor links, acknowledgement, and license input extend outside the viewport.
- Visible controls below 44 px include Reset demo and Start for real (34 px high), vendor-notice links (24 px high), footer links (16 px high), and the mobile Demo nav item (42.8 px wide).
- Activating Acknowledge action with Space works, but the full rerender moves focus to `body`; the keyboard user loses their place and returns to the top of the tab order.
- Reduced-motion emulation changes computed smooth scrolling to `auto`; there is no flashing or autoplay.

### HTTP, routing, and recovery

- A missing route returns a 404 with a zero-byte body. The shipped styled `404.html` is not served by the Rust fallback; if served under the current CSP, its inline style would also be blocked.
- Hashed JS/CSS, HTML, and the hero image have no `Cache-Control` header. The API also has no private/no-store policy for owner and watch data.
- CSP, `X-Content-Type-Options`, and `Referrer-Policy` are present. HSTS and Permissions-Policy are absent.
- A live API burst returned 40 × 200 then 60 × 429. `Retry-After` is present, satisfying the basic limiter gate, but it is hard-coded to `1` while `X-RateLimit-After` and the body said 8 seconds. A local write burst returned 40 × 400 then 20 × 429, with `Retry-After: 1` while the body said 19 seconds. Writes use the same burst threshold as reads.
- The normal invalid-field request returns 400 and a useful message; a fourth watch returns 402. Malformed URLs are nevertheless accepted, and scan errors are not surfaced.

### Design and metadata

- The paper-cut visual system is distinctive and its generated asset has prompt provenance. The optimized hero is 58,974 bytes and showed no obvious visual artifact.
- `.factory/design.md` says cards slide for 220 ms, hover-lift, and have an explicit reduced-motion replacement; the stylesheet implements none of those rules.
- The Open Graph image is the 1536×1024 hero rather than the required 1200×630 social image. Twitter title/description metadata is absent. Privacy and Terms initially ship the home canonical URL in raw HTML.

## Passing evidence

- `npm ci` — pass; 58 packages, zero npm audit findings.
- `npm test` — pass, 3/3 Vitest tests.
- `npm run build` — pass; `dist/` created.
- `cargo test --locked` — pass, 2/2 unit tests after a 1m28s clean compilation.
- `cargo build --release --locked` — pass from clean release target in 5m08s.
- `npm run test:container` — pass once release artifacts were warm.
- `npm run test:browser` — pass, 14/14 after warm build.
- Live browser suite — pass, 14/14 with `PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in`.
- Docker build — not executable in this verifier container because neither Docker, Podman, nor Buildah is installed. The Dockerfile was inspected: pinned lockfiles, Rust 1.88 builder, non-root runtime, `ARG BUILD_SHA=dev`, and no `.git` dependency are present.
- CLI consumer/pack test — not applicable because the required CLI was not implemented; `--help` starts the web server instead of exposing a CLI.
- Default runtime — pass: the release binary started with only `PORT` plus `BUILD_SHA`; `/health` returned the supplied build.
- Persistence — pass: watches/actions survived a backend restart against the same SQLite file.
- Concurrency smoke — ten simultaneous first scans produced one action and no duplicate in this run.
- Demo privacy — only `https://integration-changelog-watch.sociobot.in` was requested; no third-party font/script/analytics request occurred.
- Bundle budgets — JS 13,165 bytes raw / 5.20 KB gzip; CSS 6,608 bytes raw / 2.29 KB gzip; hero WebP 58,974 bytes; no webfont.
- Lighthouse mobile — Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 1.0 s, LCP 1.3 s, TBT 40 ms, CLS 0, Speed Index 1.0 s.
- Legal and navigation routes `/demo`, `/privacy`, and `/terms` return 200. Stripe and Auth0 sample links return 200.
- PWA update/offline-reload testing — not applicable; the product does not ship or claim a PWA/service worker.

## Deployment identity

Live `/health` returned:

```json
{"build":"9473e2873b15f9c0254adf7ac996ad41921c3625","ok":true}
```

The live and locally built SHA-256 values match exactly for `index.html`, the hashed JS, the hashed CSS, and `paper-cut-hero.webp`. This is fresh evidence that the vanity URL serves the candidate, not the earlier repair build.

## Required next steps

1. Add real tenant/auth boundaries before exposing any hosted watches/actions, and block private/link-local/loopback destinations with redirect and DNS-rebinding protections.
2. Define one shared API schema and add an end-to-end test that creates a watch, scans a multi-item fixture, renders every action, acknowledges it, and reloads.
3. Parse supported RSS/Atom safely, either remove the changelog-HTML promise or implement it, preserve item permalinks, and report per-feed failures.
4. Register and test the Sociobot paid product, then enforce license capabilities server-side per tenant; otherwise remove all paid claims and links.
5. Make every listed claim pass from a cold clone, keep exactly one tagged test per claim, and add tests for every remaining public/README claim.
6. Fix type/format/lint gates, 200% reflow, target sizes, focus restoration, real 404 routing, cache policy, and accurate rate-limit recovery headers.
