# Independent verification 2 — FAIL

Verified independently on 2026-08-28 UTC.

- Candidate: `865e029755c1ffa9c8a28b281b72bc9b4f16f454`
- Live URL: `https://integration-changelog-watch.sociobot.in`
- Work order: `integration-changelog-watch-verify-2`
- Result: **FAIL — do not release**

The candidate is deployed and most automated quality gates pass. The release still fails the supplied acceptance contract. Public claims are missing from the required claim registry, the deployed rate limiter can be bypassed with a supplied forwarding header, feed failures disappear from the UI, full workspaces cannot be repaired, and the shipped CLI example no longer works.

No product code was changed during this verification.

## Mandatory first-read gate — PASS

A fresh desktop browser context opened `/` with no existing storage. The first viewport answers all three required questions:

- What it does: **“Turn vendor changes into owned actions.”**
- For whom: **“For engineers who maintain payment, auth, analytics, or messaging integrations.”**
- What to click first: **“Try it with sample data”**, followed by “See matched notices, owners, and checks.”

At 390 px, the sample button is fully inside the first viewport at `y=373.94`, is 241.67 × 46 px, and opens `/demo` in one click. The demo immediately shows three watches, two action cards, owners, commands, and the persistent sample-data banner.

Evidence: `qa-artifacts/live-first-read.txt`, `qa-artifacts/live-first-read-desktop.png`, `qa-artifacts/live-first-read-mobile.txt`, and `qa-artifacts/live-first-read-mobile.png`.

## Claims gate

### Declared commands — PASS after clean dependency installation

`.factory/claims.json` exists and contains four well-formed entries. After `npm ci`, every exact command passed against `/demo`; each tagged case ran in desktop and mobile Chromium.

| Claim | Exact result | Evidence |
| --- | --- | --- |
| `sample-action-cards` | PASS, 2/2 | `qa-artifacts/claim-sample-action-cards.log` |
| `csv-export` | PASS, 2/2 | `qa-artifacts/claim-csv-export.log` |
| `demo-local` | PASS, 2/2 | `qa-artifacts/claim-demo-local.log` |
| `workspace-boundary` | PASS, 2/2 | `qa-artifacts/claim-workspace-boundary.log` |

Each claim ID occurs in exactly one tagged browser test.

### Claim inventory — FAIL

The claims contract also requires every public statement a visitor may rely on to appear in `.factory/claims.json` with one matching sandbox test. The registry omits several current statements:

- Landing page: “Scans run only when you request them.”
- Landing page: “Each matching notice gets an owner and a check command.” The registered sample claim proves canned cards, not a real match/scan.
- Privacy page: “No analytics, advertising scripts, or third-party fonts run here.”
- Privacy page: “The server stores watches and action cards only inside the workspace token you create.”
- README: credentialed and redirecting sources are rejected.
- README/CLI help: `demo` does not contact a feed.
- README: the SQLite database persists at `/data/changelog-watch.db` when mounted.
- README/CLI help: `scan --config` turns the repository mapping into Markdown output.

The tests and manual checks prove some of these statements, but the mandatory registry is still incomplete. Under the supplied claims contract, this alone is release-blocking.

## Release-blocking findings

### Major — forwarding-header input bypasses the deployed rate limit

The configured local allowance is 20 requests per second with a burst of 40. Against a fresh local limiter key, 60 simultaneous requests produced exactly 40 × 200 and 20 × 429.

The deployed service behaved differently when the single client supplied a fixed `X-Forwarded-For: 198.51.100.77`: **80 simultaneous `/health` requests all returned 200**. Repeating without a synthetic forwarding header did enforce the limiter: 120 simultaneous requests returned 77 × 200 and 43 × 429 as the bucket refilled during the burst. This shows that a client-controlled forwarded header changes extraction so the documented single-client allowance is not enforced behind the factory ingress.

The 429 response also contains contradictory recovery timing:

```text
Retry-After: 1
X-RateLimit-After: 19
Too Many Requests! Wait for 19s
```

The code hard-codes `Retry-After: 1`, so a compliant client retries well before the server says capacity returns.

Evidence: `qa-artifacts/local-rate-limit.txt`, `qa-artifacts/live-rate-limit.txt`, and `qa-artifacts/live-rate-limit-direct.txt`.

### Major — feed failures and full workspaces have no recovery path

A real UI scan of `https://example.com/definitely-missing-icw-feed.xml` returned a useful backend result:

```json
{
  "new_actions": 0,
  "failures": ["Missing Feed: The feed returned an error response."],
  "message": "Scan finished with 1 feed error(s). Fix the listed address and scan again."
}
```

The frontend writes that message, immediately calls `hydrateReal()`, and re-renders the dashboard. After 1.2 seconds, `#notice` is empty, no action is present, and the visible heading only says “No actions need an owner.” The user cannot tell that the scan failed.

The requested recovery is impossible. Watches have no edit or delete API and no edit/remove/delete control. After three watches, a fourth returns 409 and the UI says:

> This workspace already has three watches. Edit an existing watch before adding another.

There are zero edit controls. A mistyped or retired feed permanently consumes one of the three slots unless the user abandons the entire browser token and workspace.

Evidence: `qa-artifacts/local-scan-e2e.txt`, `qa-artifacts/local-scan-failure.png`, `qa-artifacts/local-boundaries.txt`, and `qa-artifacts/local-quota-dead-end.txt`.

### Major — the shipped repository CLI example is broken and the CLI omits required state

The clean installed CLI, `--help`, and canned `demo` command work. The documented real command does not:

```text
$ integration-changelog-watch scan --config examples/watches.json
scan failed: The feed returned an error response.
exit 1
```

The only example points to `https://docs.stripe.com/changelog/rss.xml`, which now returns 404. The other demo watch addresses also return 404, 404, and 403 respectively. A fresh consumer therefore cannot exercise the real repository-owned workflow with the shipped mapping.

The researched smallest product requires the CLI to store content hashes and support an acknowledgement record. `cli_scan` only fetches the current source and prints every match to stdout on each run. It writes no hash/state, opens no repository action-card file, and exposes no acknowledgement command. Hashes and acknowledgement exist only in the separate hosted dashboard, so the repository-owned CLI workflow is incomplete.

Evidence: `qa-artifacts/cli-scan.err`, the packed-crate consumer run, `examples/watches.json`, and `src/main.rs::cli_scan`.

## Other findings

### Moderate — anonymous storage input is insufficiently bounded

The API accepted a watch containing 10,000 characters in each of `vendor`, `keywords`, `owner`, and `command`, returning a 40,125-byte JSON body. Workspace creation is anonymous and the forwarding-header limiter can be bypassed, so storage growth is not constrained by practical field limits. Empty required values, malformed URLs, credentials, IPv4 loopback, link-local metadata addresses, and ordinary IPv6 loopback were correctly rejected.

### Moderate — every cold real visit creates two workspaces

The initial real dashboard calls `api('/api/watches')` and `api('/api/actions')` concurrently. Both enter `ensureWorkspace()` before either has stored a token. A cold page therefore sent two `POST /api/workspaces` requests and created two rows, abandoning one token. This doubles anonymous workspace churn and can briefly hydrate watches and actions from different new workspace tokens.

Evidence: `qa-artifacts/live-first-read.txt`.

### Minor — some touch targets remain below 44 × 44 px

Independent measurements found the wordmark link 171.33 × 24 px, the 390 px Demo link 42.77 × 44 px, and the Terms link 40.03 × 44 px. The visible buttons meet the target size. The visually hidden skip link is intentionally compact until focused.

### Minor — the live 404 is not the required product screen

The missing route correctly returns 404 with a title, `<main>`, `<h1>`, and a home link. It has no stylesheet, header, footer, metadata, or paper-cut identity. This conflicts with the site-structure requirement for a designed 404 and the standard skeleton on every route.

### Minor — startup configuration logging is inaccurate

With an explicitly supplied `DATABASE_URL`, startup still logs `"DATABASE_URL defaulted when absent"`. The server does start with only `PORT`, but the mandatory supplied-vs-generated configuration log is not truthful.

## End-to-end functional evidence

- A disposable local backend and real GitHub changelog feed produced 10 matched actions.
- Cards contained item permalinks rather than the feed URL.
- Acknowledging one real numeric action ID changed it to “Acknowledged,” preserved keyboard focus on that card, and remained acknowledged after reload.
- Ten concurrent scans created 10 distinct actions with no duplicates; one scan reported 10 new actions and the other nine reported zero.
- A watch and its token-authorized data survived a backend restart against the same SQLite file.
- A second token saw no first-token watches, and unauthenticated API access returned 401 with `Cache-Control: no-store, private`.
- The fourth watch returns 409 after exactly three accepted watches.

Evidence: `qa-artifacts/local-scan-e2e.txt`, `qa-artifacts/local-real-ack.txt`, `qa-artifacts/local-concurrency.txt`, `qa-artifacts/local-boundaries.txt`, and `qa-artifacts/local-persistence.txt`.

## Local quality gates

| Gate | Result |
| --- | --- |
| `npm ci` | PASS, 60 packages, 0 vulnerabilities |
| `npm test` | PASS, 3/3 |
| `npm run typecheck` | PASS |
| `npm run lint` | PASS |
| `npm run build` | PASS; `dist/` created |
| `cargo fmt --all -- --check` | PASS |
| `cargo test --locked` | PASS, 5/5 |
| `cargo clippy --all-targets --locked -- -D warnings` | PASS |
| `cargo build --release --locked` | PASS |
| `npm run test:browser` | PASS, 20/20 local |
| Live Playwright suite | PASS, 20/20 |
| `cargo package --locked` | PASS; warned that package metadata is absent |
| Install packed crate into a clean root | PASS |
| Installed `--help` and `demo` | PASS |
| Installed real example scan | **FAIL**, dead Stripe feed |

The exact Docker image build could not be executed because this verifier image has no Docker, Podman, or Buildah. The checked Dockerfile uses pinned lockfiles, a Rust 1.88 builder, a non-root Alpine runtime, `ARG BUILD_SHA=dev`, and no `.git` dependency. The frontend build and exact locked release backend build both pass.

Evidence: `qa-artifacts/local-gates.log`, `qa-artifacts/cargo-package.log`, and `qa-artifacts/clean-consumer.txt`.

## Accessibility, mobile, privacy, and performance

- Axe WCAG 2 A/AA/2.1 AA: zero serious/critical findings on `/demo` at 1440 px, 390 px, and 195 px.
- Semantic smoke: `lang=en`, one `<h1>`, one `<main>`, ordered headings, and no image missing `alt`.
- Keyboard: first Tab exposes the skip link with a 3 px dashed focus ring; Enter focuses `#main`; Space acknowledges the sample action; real acknowledgement restores focus to the changed card.
- Reflow: no horizontal overflow at 390 px or the 195 px 200%-equivalent viewport.
- Reduced motion: transition and animation durations compute to `0s`; scroll behavior is `auto`.
- Console/page errors: none in cold, demo, accessibility, or real success flows.
- Demo privacy: only the product origin was contacted through acknowledge, CSV export, and reset; there were no cookies, and reset left local storage empty.
- Cold real page: requests were same-origin only. No analytics, remote font, advertising, Azure, billing, or third-party script request occurred.
- Security headers: CSP, HSTS, nosniff, Referrer-Policy, and Permissions-Policy are present.
- Cache policy: HTML `no-cache`; hashed JS/CSS `public, max-age=31536000, immutable`; images cache one week; private API responses `no-store, private`.
- Bundle sizes: JS 12,331 bytes raw / 4,934 gzip; CSS 7,507 raw / 2,502 gzip; hero WebP 58,974 bytes; no webfont.
- Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 1.0 s, LCP 1.3 s, TBT 60 ms, CLS 0, Speed Index 1.0 s.

Evidence: `qa-artifacts/live-accessibility-responsive.txt`, `qa-artifacts/live-demo-privacy.txt`, and `qa-artifacts/lighthouse-live.json`.

The product is not a PWA and makes no offline/PWA claim: no manifest and no service-worker registration were present. It does not require sign-in, so the Entra authority check is not applicable. No paid feature or unlock call is present.

## Deployment identity

Live `/health` returned the exact candidate:

```json
{"build":"865e029755c1ffa9c8a28b281b72bc9b4f16f454","ok":true}
```

The locally built and live files match byte-for-byte:

| File | SHA-256 |
| --- | --- |
| `index.html` | `14760d05b669ddb10627615ce7ebb61d51915871897a78644e656594c56ceb36` |
| `assets/index-CWYyw5pc.js` | `fcf81269c2d8eb365e1b8f472c05943e67eab537534a10b3c367f49df971837b` |
| `assets/index-CPi_kfO2.css` | `6062de6bb105a6f67d483dc7d6c306bee59960900575daf4840fd87fb2441431` |
| `paper-cut-hero.webp` | `fb0d415bec60a017de28d29ee05795fbf156fc3ab069dab80d3bd7dce49a252b` |
| `social-card.jpg` | `8c7368837b1c4c7c397306a16a51f1a16a05fa6ff42a2e9ebcb3c8a19f99e55b` |

This is fresh evidence that the live deployment is candidate `865e029`, not the previously reported repair build.

## Required release repairs

1. Derive the limiter key only from a trusted ingress-sanitized client IP; add an integration test for a client-supplied forwarding header; emit the real `Retry-After` value.
2. Preserve scan success/failure messages after hydration and add edit/delete operations for watches, including recovery from a full three-watch workspace.
3. Replace the dead example feed with a controlled public fixture or stable test feed, then make the CLI persist hashes/action-card state and acknowledgements as required by the brief.
4. Register and tag every public/README claim, or remove statements the sandbox does not prove.
5. Bound persisted field lengths and avoid the double workspace-creation race.
6. Finish touch targets, the product-styled 404, and accurate startup configuration logging.
