# Adversarial first-read review 2

- Product: Integration Changelog Watch
- Live URL: <https://integration-changelog-watch.sociobot.in>
- Review date: 2026-08-29 UTC
- Repository review base: `f2dfddaefe7cb044c8dc4c445cec3fefc99514a1`
- Live `/health` build: `34d2c65449dfc26b6d8ae606044bf072fa9b626f`
- Verdict: **FAIL**

There are two findings. One is blocking: a documented deep link does not reach its target when opened cold. A pass requires zero findings.

## Cold first screen

Fresh Chromium contexts at 390 × 844 and 1440 × 900 showed the same answers before scrolling.

| Question | Answer from the first screen |
| --- | --- |
| What does this do? | It turns matching vendor-change notices into action cards with an owner, dependency version, and check. |
| For whom? | Engineers maintaining payment, authentication, analytics, or messaging integrations. |
| What should I click first? | **Try it with sample data**. |

The text that makes this clear is “Turn vendor changes into assigned action cards,” “For engineers who maintain payment, auth, analytics, or messaging integrations,” and “Try it with sample data.” The cold first screen is clear enough to avoid a first-read blocking finding. Evidence: `qa-artifacts/review-2-cold-mobile.png` and `qa-artifacts/review-2-cold-desktop.png`.

## Findings

### Blocking

#### F-2-1 — Cold `/#how` deep link does not reach the How it works section

- Location/quote: header link **“How it works”** has `href="/#how"`. A fresh direct request to `https://integration-changelog-watch.sociobot.in/#how` completes with `scrollY: 0`; the rendered `#how` target begins at `y: 1983.734375` on a 390 px viewport.
- Why this fails: a shared link to a real section appears to work, but a visitor who opens or bookmarks it lands at the hero instead. This is broken deep-link routing, which is blocking under the site-structure contract.
- Concrete fix: after the SPA renders the landing route, if `location.hash` names an in-page target, move that target into view. Preserve this on `popstate` and history navigation. Add a browser regression test that opens `/#how` in a fresh context and asserts the target intersects the viewport.

### Minor

#### F-2-2 — “auth” is unexplained shorthand in the audience sentence

- Location/quote: landing hero and README: **“For engineers who maintain payment, auth, analytics, or messaging integrations.”**
- Why this fails: “auth” is jargon in the sentence that must tell an unfamiliar visitor who the product is for. The full word fits within the plain-words limit.
- Concrete rewrite: **“For engineers who maintain payment, authentication, analytics, or messaging integrations.”**

## Copy audit

Counts treat a hyphenated compound, a command, a path, and a URL as one word. Headings are included where they carry a sentence-like claim. No item exceeds 22 words. `F-2-2` is the only jargon flag. Buttons are listed separately and all name their result.

### Landing and demo entry

| # | Words | Text | Result |
| --- | ---: | --- | --- |
| L1 | 7 | Turn vendor changes into assigned action cards | Pass |
| L2 | 10 | For engineers who maintain payment, auth, analytics, or messaging integrations. | F-2-2 |
| L3 | 7 | See matched notices, owners, versions, and checks. | Listed: `sample-action-cards` |
| L4 | 5 | You choose the matching keywords. | Listed: `keyword-edit` |
| L5 | 7 | Scans run only when you request them. | Listed: `requested-scans` |
| L6 | 7 | Your workspace is separated from other visitors. | Listed: `workspace-boundary` |
| L7 | 4 | No actions need acknowledgement | Pass empty-state label |
| L8 | 10 | Matched release notes appear here after you scan a feed. | Listed: `hosted-scan-result` |
| L9 | 4 | Nothing is watched yet. | Pass empty state |
| L10 | 7 | Give each vendor change a next step | Pass heading |
| L11 | 4 | Watch a public feed. | Pass instruction |
| L12 | 11 | Paste a changelog or RSS address you are allowed to read. | Pass instruction |
| L13 | 3 | Choose your keywords. | Pass instruction |
| L14 | 9 | Use keywords like “webhook”, “deprecation”, or an API version. | Pass example |
| L15 | 4 | Run the right check. | Pass instruction |
| L16 | 11 | Each matching notice includes an owner, dependency version, and check command. | Listed: `hosted-scan-result` |
| L17 | 8 | The hosted workspace holds up to three watches. | Listed: `hosted-watch-limit` |
| L18 | 8 | Use the local CLI for a four-watch mapping. | Listed: `cli-more-feeds` |
| L19 | 5 | It does not scan automatically. | Listed: `requested-scans` |
| L20 | 7 | Private, loopback, and link-local addresses are blocked. | Listed: `workspace-boundary` |
| L21 | 6 | Vendor notices become assigned action cards. | Listed: `hosted-scan-result` |
| D1 | 6 | Demo — sample data, nothing is saved | Listed: `demo-local` |
| D2 | 3 | Discards this demo. | Listed: `demo-isolation-transitions` |

Buttons: **Try it with sample data**, **Scan watched feeds**, **Add a watch**, **Add your first watch**, **Export watch file**, **Import watch file**, **Export action cards as CSV**, **Reset demo**, **Start a private workspace**, and **Acknowledge action** are all result-naming verbs. “Open vendor notice” is an external link, not a button.

### README

| # | Words | Text | Result |
| --- | ---: | --- | --- |
| R1 | 7 | Turn vendor changes into assigned action cards. | Listed: `hosted-scan-result` |
| R2 | 12 | It is for engineers who maintain payment, auth, analytics, or messaging integrations. | F-2-2 |
| R3 | 19 | Add a public changelog or RSS feed, keywords, an owner, the affected dependency version, and a local check command. | Pass instruction |
| R4 | 12 | Scan when you are ready to review notices and create action cards. | Listed: `hosted-scan-result` |
| R5 | 7 | The container uses durable `/data` by default. | Listed: `database-persistence` |
| R6 | 16 | On a host without that mount, use `DATABASE_URL='sqlite:changelog-watch.db?mode=rwc' cargo run`. | Pass instruction |
| R7 | 7 | For frontend development, use `npm run dev`. | Pass instruction |
| R8 | 10 | Run `npm test` and `npm run typecheck` for code checks. | Pass instruction |
| R9 | 12 | After `npm run build`, run `npm run test:browser` for browser coverage. | Pass instruction |
| R10 | 9 | The exact claim commands live in `.factory/claims.json`. | Pass instruction |
| R11 | 9 | Open `http://localhost:8080/demo` for a one-click sandbox. | Listed: `demo-local` |
| R12 | 7 | Demo data uses `demo:integration-changelog-watch` browser storage. | Listed: `demo-local` |
| R13 | 6 | Start a private workspace discards it. | Listed: `demo-isolation-transitions` |
| R14 | 10 | The container exposes health, workspace, watch, action, and scan endpoints. | Listed: `api-contract` |
| R15 | 4 | Create a workspace first. | Listed: `api-contract` |
| R16 | 9 | Send its browser-held bearer token with every workspace request. | Listed: `api-contract` |
| R17 | 14 | Source URLs must be public `http` or `https` addresses; private network addresses are rejected. | Listed: `workspace-boundary` |
| R18 | 13 | Use **Export watch file** to download your watches in the CLI JSON schema. | Listed: `watch-file-portability` |
| R19 | 10 | Use **Import watch file** to preview one to three watches. | Listed: `watch-file-portability` |
| R20 | 9 | A rejected private-workspace import leaves the current watches unchanged. | Listed: `watch-file-rejection-preserves-watches` |
| R21 | 6 | Demo imports stay in demo storage. | Listed: `watch-file-portability` |
| R22 | 7 | `demo` prints the bundled Markdown action-card sample. | Listed: `cli-demo-local` |
| R23 | 14 | `scan --config` reads a JSON watch file and writes Markdown cards under `.integration-changelog-watch/actions/`. | Listed: `cli-repository-workflow` |
| R24 | 10 | It stores hashes and acknowledgement state in `.integration-changelog-watch/state.json`. | Listed: `cli-repository-workflow` |
| R25 | 12 | Each card prints its hash-derived action ID; pass that ID to `ack`. | Listed: `cli-repository-workflow` |
| R26 | 10 | Acknowledgement updates both the state file and the Markdown card. | Listed: `cli-repository-workflow` |
| R27 | 14 | The shipped example uses `examples/sample-feed.xml`, so it works without a network request. | Listed: `cli-shipped-mapping-local` |
| R28 | 8 | The hosted workspace holds up to three watches. | Listed: `hosted-watch-limit` |
| R29 | 8 | Use the local CLI for a four-watch mapping. | Listed: `cli-more-feeds` |
| R30 | 13 | Build the repository image with `docker build --build-arg BUILD_SHA=dev -t integration-changelog-watch .`. | Pass instruction |
| R31 | 9 | The build stage uses the official `rust:1-alpine` image. | Listed: `container-build-stage` |
| R32 | 8 | It starts with only `PORT` required (default `8080`). | Listed: `port-only-startup` |
| R33 | 22 | The shipped Container App configuration uses one replica and mounts durable Azure Files at `/data`, where SQLite persists at `/data/changelog-watch.db`. | Listed: `single-replica-durable-data`, `database-persistence` |
| R34 | 17 | A production topology guard closes workspace APIs and restores that configuration if a generic deploy removes it. | Listed: `single-replica-durable-data` |
| R35 | 13 | SQLite uses Azure Files-compatible dot-file locks, so do not raise the replica count. | Operational warning; topology claim covers the enforced configuration |
| R36 | 10 | See `/privacy` and `/terms` for data handling and source rules. | Pass link instruction |
| R37 | 2 | MIT licensed. | Confirmed by `LICENSE` |
| R38 | 4 | Built by Param Factory. | Pass attribution |

No remaining claim-like landing or README sentence lacks a suitable registry entry. There are no marketing adjectives, mood headings, or inconsistent core product terms. “RSS,” “API,” “CLI,” Docker, and SQLite are necessary technical terms in instructions or engineering documentation; `auth` is the avoidable abbreviation in F-2-2.

## Demo and sandbox

The one-click path and direct `/demo` path pass the sandbox review.

- The first phone viewport immediately shows the persistent **“Demo — sample data, nothing is saved”** banner, reset/exit controls, and a realistic Stripe action card with its matched keyword, owner, dependency version, and check. Evidence: `qa-artifacts/review-2-demo-direct-mobile.png`.
- In a fresh direct demo context, initial requests are only the demo document, same-origin JS, and same-origin CSS. Acknowledge sends no additional request and writes only `demo:integration-changelog-watch`.
- **Reset demo** removes that key and restores the seeded sample. **Start a private workspace** removes demo storage before creating `icw:workspace` and `icw:workspace-token`.
- No analytics, advertising, third-party fonts, API calls, or non-same-origin requests occur in the demo flow. The product makes no published offline-reload claim.

## Claims

From clean clone `/tmp/icw-review-2-lnIaoq`, `npm ci` followed by `npm run test:claims` ran every literal command in `.factory/claims.json`. All 21 passed and the runner confirmed port 8080 was released between commands. Log: `qa-artifacts/review-2-claims.log`.

| Claim IDs passed |
| --- |
| `sample-action-cards`, `csv-export`, `demo-local`, `demo-isolation-transitions`, `workspace-boundary`, `hosted-scan-result`, `hosted-watch-limit` |
| `keyword-edit`, `requested-scans`, `redirecting-feeds`, `watch-file-portability`, `watch-file-rejection-preserves-watches`, `cli-more-feeds` |
| `cli-repository-workflow`, `cli-demo-local`, `cli-shipped-mapping-local`, `api-contract`, `container-build-stage`, `database-persistence` |
| `port-only-startup`, `single-replica-durable-data` |

## History recheck

Read in full: `.factory/review-1.md`, `.factory/polish-1.md`, and the previous `.factory/handoff.md`. Every earlier finding was confirmed fixed on the live site and in current code.

| Earlier findings | Current verification |
| --- | --- |
| F-1-1, F-1-2 | Demo starts on populated work at 390 px; full Axe WCAG 2 A/AA returns zero violations on `/`, `/demo`, `/privacy`, `/terms`, and the 404. |
| F-1-3 to F-1-10 | The claims registry now covers the published hosted result, capacity, keyword, CLI, API, container, and scope behavior; current copy removes the prior unlisted statements. |
| F-1-11 to F-1-16 | Copy is within the cap, terms are consistent, decorative labels are gone, and the banner uses **Start a private workspace**. |
| F-1-17, F-1-18 | Route-specific title, description, canonical, OG/Twitter metadata, apple icon, and the shared 404 navigation are live. |
| F-1-19 | Demo and hosted UI expose watch-file import/export, with preview and registered portability/isolation tests. |

The current `/#how` defect is a new regression from the earlier deep-link pass, not an unresolved F-1 item.

## Structure, links, accessibility, and visual identity

Verified passes:

- `/`, `/demo`, `/privacy`, and `/terms` return 200. An unknown route returns a designed 404 and a return-home action.
- Each tested route has `lang="en"`, exactly one `<main>`, exactly one `<h1>`, a route-specific title, description, canonical, OG/Twitter preview, favicon, and apple-touch icon. Titles meet the required patterns.
- `robots.txt`, `sitemap.xml`, favicon, apple icon, and social card return 200. Every discovered internal link and both vendor-notice links return 200.
- Header/footer are consistent, include Privacy and Terms, and include a skip link. Privacy navigation moves focus to its h1; Back returns to `/` with focus on the landing h1. F-2-1 is the sole routing failure.
- No successful-route console errors occurred. Full Axe reports zero WCAG 2 A/AA violations on all five tested routes. The 390 px layout, keyboard controls, focus styling, and reduced-motion CSS checks pass.
- The warm-paper, indigo-thread, clipped-card interface and original paper-cut art match `.factory/design.md` and are distinct from a generic SaaS template.
- The request log and CSP confirm no third-party runtime scripts, analytics, remote fonts, embedded provider key, payment integration, or runtime AI feature. No obviously valuable AI step is implied by this deterministic watch-and-scan workflow; import/export is already provided.

## What would make this perfect

Repair F-2-1 with a cold-hash regression test, replace “auth” with “authentication,” then rerun this full review from fresh phone and desktop contexts. A perfect result has zero findings, including no broken direct deep links.
