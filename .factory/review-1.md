# Adversarial first-read review 1

- Product: Integration Changelog Watch
- Live URL: <https://integration-changelog-watch.sociobot.in>
- Review date: 2026-08-29 UTC
- Live build shown in the footer: `99f0ca341a13140030b4f50272b4b399c54cbd57`
- Repository review base: `f4c3449008d124f172cfc799c51a959ec2e95449`
- Verdict: **FAIL**

There are 19 findings, including two blocking findings. All 13 listed claim commands pass, but the one-click demo does not put sample work in the first phone viewport and a previously recorded accessibility defect remains live. A pass requires zero findings.

## Cold first screen, before scrolling

Fresh Chromium contexts were used at 390 × 844 and 1440 × 900. Screenshots are in `review-1-artifacts/cold-mobile.png` and `review-1-artifacts/cold-desktop.png`.

| Question | 390 px phone | Desktop |
| --- | --- | --- |
| What does this do? | It scans vendor change notices and turns matches into work with an owner and a check. | Same. The empty action queue also begins at the bottom of the viewport. |
| For whom? | Engineers maintaining payment, authentication, analytics, or messaging integrations. | Same. |
| What should I click first? | **Try it with sample data**. | **Try it with sample data**. |

The text that makes this answerable is: “Turn vendor changes into owned actions”, “For engineers who maintain payment, auth, analytics, or messaging integrations”, and “Try it with sample data”. The cold first screen therefore does not receive a blocking first-read finding. “Owned actions” remains a copy finding because it is less concrete than the rest of the explanation.

## Findings

### Blocking

#### F-1-1 — The demo opens on another marketing screen, not visible sample work

- Location/quote: click **“Try it with sample data”** on `/` at 390 px. `/demo` shows **“Demo — sample data, nothing is saved”**, then repeats **“Turn vendor changes into owned actions”** and even repeats **“Try it with sample data”**.
- Evidence: `review-1-artifacts/demo-first-mobile.png` and `review-1-artifacts/demo-mobile.json`. At 844 px high, no sample action title, owner, dependency version, or check is in the viewport. The first dashboard text starts below the captured phone viewport. On desktop only the queue heading and controls reach the bottom edge; the realistic sample card is still below it.
- Why this fails: the required first screen after the one click does not show the product being used. A phone visitor spends the demo click on a near-duplicate landing page and must guess that the sample is below the illustration.
- Fix: make `/demo` begin with the demo banner and populated action board. Remove the landing hero and duplicate demo CTA from demo mode, or move the populated board ahead of them. Focus the demo’s product heading after navigation. Add a 390 × 844 test that asserts a sample action title, owner, version, and check all intersect the initial viewport without scrolling.

#### F-1-2 — The earlier accessibility finding remains live

- Earlier location: `.factory/handoff.md`, “Known gaps and next step”: **“Axe reports one moderate `landmark-complementary-is-top-level` issue on the demo.”** That earlier note did not assign an ID; this review assigns `F-1-2` so it can be tracked from now on.
- Live/code evidence: full Axe on `/demo` still reports `.watches`, rendered as `<aside class="watches" aria-label="Watched feeds">`, because the complementary landmark is contained in another landmark. Evidence: `review-1-artifacts/axe-demo.json`; source: `frontend/src/main.ts` in `dashboard()`.
- Why this fails: the history rule makes any unfixed earlier finding blocking, regardless of its earlier severity label. The nested landmark also gives screen-reader users an inaccurate page-region structure.
- Fix: make the watched-feeds panel a labelled `<section>` inside `<main>`, or move a genuinely complementary `<aside>` outside the main landmark. Run full Axe, not only the serious/critical filter, and require zero violations.

### Major — unlisted claims

#### F-1-3 — The core hosted scan-result promise has no claim entry

- Location/quote: landing empty state, **“Matched release notes will appear here.”**
- Why this fails: no `.factory/claims.json` entry proves that a hosted scan fetches a controlled feed, matches a keyword, and creates the promised action. `requested-scans` only proves that clicking sends one mocked scan request.
- Fix: add a claim entry and an end-to-end backend/browser test that scans a controlled public fixture, then asserts the new title, matched keyword, owner, version, and check in the action card.

#### F-1-4 — The free three-feed limit is not listed as a claim

- Location/quote: landing, **“The hosted dashboard is a free private workspace for three feeds.”**
- Why this fails: `workspace-boundary` covers token isolation and blocked addresses, not price or the three-feed limit. A visitor can rely on both.
- Fix: split the sentence into independently testable facts. Add a capacity claim whose clean-workspace test saves three watches and confirms that the fourth receives the readable limit error. Add a tested pricing/source-of-truth assertion or remove “free”.

#### F-1-5 — Team-authored rules are an unlisted behavior claim

- Location/quote: landing, **“Rules are written by your team.”**
- Why this fails: no claim entry verifies that the user can create and edit the matching keywords represented by “rules”.
- Fix: rewrite as the concrete action **“You choose the matching keywords.”** Add a claim test that creates a watch with keywords, edits them, reloads, and confirms the saved value.

#### F-1-6 — “More feeds” in the CLI is not tested

- Location/quote: landing, **“Use the local CLI for repository-owned mappings with more feeds.”** README: **“Teams can keep larger mappings and action cards in their repository with the local CLI.”**
- Why this fails: `cli-repository-workflow` uses one watch. It does not prove that the CLI accepts more than the hosted limit of three.
- Fix: add one claim entry and run a temporary CLI mapping with at least four feeds, asserting four watches are processed, or remove the comparative capacity wording.

#### F-1-7 — The documented API surface is not in the claim registry

- Location/quote: README, **“The container exposes GET /health, POST /api/workspaces, GET|POST /api/watches, PUT|DELETE /api/watches/:id, GET /api/actions, POST /api/actions/:id, and POST /api/scan.”**
- Why this fails: this is a public integration contract. Existing claim tests exercise only part of it and there is no API-surface claim entry.
- Fix: list an API-contract claim and test every documented method, authentication boundary, success status, and representative error from a clean workspace. A generated API table would also be easier to scan.

#### F-1-8 — “Current stable Rust” is an unlisted, time-sensitive claim

- Location/quote: README, **“The container uses the current stable Rust image and builds with `cargo build --release --locked`.”**
- Why this fails: `.factory/claims.json` has no entry for this sentence. The existing Vitest checks a Dockerfile string, not whether the floating image is current or whether the image build succeeds.
- Fix: replace it with the durable fact **“The build stage uses the official `rust:1-alpine` image.”** If retaining the build promise, add a claim that builds the image in a clean Docker environment.

#### F-1-9 — The hosted-feature exclusions are not registered claims

- Location/quote: landing, **“It does not provide shared team workspaces, accounts, unlimited watches, or a paid plan.”**
- Why this fails: these scope and pricing assertions are information a visitor can rely on, but no claims entry points to a test or other checked source of truth.
- Fix: list a hosted-scope claim that crawls the product routes/API surface and verifies the stated limits, or consolidate these facts into a versioned scope document and add a claim test against it.

#### F-1-10 — The collaboration exclusion is another unlisted claim

- Location/quote: README, **“Hosted team collaboration is not offered or implied.”**
- Why this fails: it is distinct from token isolation and has no claim entry. It also repeats F-1-9 in different terms.
- Fix: delete the duplicate sentence and keep one tested hosted-scope statement.

### Minor — copy, metadata, and leverage

#### F-1-11 — README verification instructions exceed 22 words

- Location/quote: README R11, 29 words: **“Run `npm run typecheck` for TypeScript checks, `cargo test` for server checks, and `npm run test:browser` after `npm run build` for browser, keyboard, mobile, privacy, and accessibility coverage.”**
- Why this fails: it combines four commands and six coverage areas in one sentence.
- Rewrite: **“Run `npm run typecheck` and `cargo test` for code checks. After `npm run build`, run `npm run test:browser` for browser coverage.”**

#### F-1-12 — The API endpoint sentence exceeds 22 words

- Location/quote: README R16, 28 words, quoted in F-1-7.
- Why this fails: a dense inline endpoint inventory is difficult to verify on first read.
- Rewrite: **“The container exposes health, workspace, watch, action, and scan endpoints.”** Follow it with a method/path table.

#### F-1-13 — The CLI persistence sentence exceeds 22 words

- Location/quote: README R21, 26 words: **“`scan --config` reads a repository-owned JSON watch mapping, writes new Markdown action cards under `.integration-changelog-watch/actions/`, and stores hashes plus acknowledgement state in `.integration-changelog-watch/state.json`.”**
- Why this fails: it combines input, two outputs, and deduplication state.
- Rewrite: **“`scan --config` reads a JSON watch file and writes Markdown cards under `.integration-changelog-watch/actions/`. It stores hashes and acknowledgement state in `.integration-changelog-watch/state.json`.”**

#### F-1-14 — Core terms are abstract and inconsistent

- Locations/quotes: **“owned actions”**, **“Your owned queue”**, README **“matching words”**, landing **“Rules are written by your team”**, **“Match your words”**, and the watch form’s **“Keywords”**.
- Why this fails: “owned” is management shorthand, while “rules”, “words”, “matching words”, and “keywords” name the same concept differently.
- Fix: use **“assigned action cards”** for the output and **“keywords”** everywhere for matching. Example headline: **“Turn vendor changes into assigned action cards.”** Example step: **“Choose your keywords.”**

#### F-1-15 — Two decorative labels carry no usable information

- Locations/quotes: hero eyebrow **“INTEGRATION CHANGELOG WATCH”** repeats the wordmark; figure caption **“Original paper-cut illustration.”** describes provenance rather than product use.
- Why this fails: neither line helps the visitor understand the job or next action. The image already has useful alt text, and provenance belongs in the design record.
- Fix: remove both visible labels. Keep the image alt text and `.factory/design.md` provenance.

#### F-1-16 — “Start for real” does not name its result

- Location/quote: demo banner button **“Start for real”**.
- Why this fails: it does not say that the action discards demo state and opens a private workspace.
- Rewrite: **“Start a private workspace”**. Keep a short adjacent note: **“Discards this demo.”**

#### F-1-17 — Route metadata describes the landing page on legal and demo routes

- Location: live `/demo`, `/privacy`, and `/terms`.
- Evidence: `review-1-artifacts/structure.json`. Titles and canonicals change correctly, but every route retains meta/OG text **“Turn vendor release notes into owned integration actions”** and OG title **“Integration Changelog Watch — Track vendor changes.”**
- Why this fails: shared previews for legal and demo URLs misdescribe the linked page.
- Fix: update description, `og:title`, `og:description`, `twitter:title`, and `twitter:description` on every route alongside `document.title` and canonical. Add route-level assertions.

#### F-1-18 — The designed 404 is missing standard metadata and consistent navigation

- Location/evidence: live unknown route and `frontend/public/404.html`; see `review-1-artifacts/structure.json`.
- Exact difference: normal header is **“Demo · How it works · Privacy”**; 404 header is **“Demo · Privacy”**. The 404 omits Open Graph, Twitter-card, and apple-touch metadata.
- Why this fails: the 404 is visually on-brand but does not use the consistent site skeleton or complete metadata.
- Fix: use the same header links and metadata set as the app shell, with a 404-specific title and description.

#### F-1-19 — Watch configuration cannot move between the dashboard and repository workflow

- Location: product workflow; the dashboard creates watches one prompt at a time, while the CLI uses `watches.json`. Only action cards export as CSV.
- Why this matters: a team evaluating both advertised modes would expect to reuse the same vendor, URL, keywords, owner, version, and check fields rather than re-enter them.
- Fix: add **Import watch file** and **Export watch file** for the documented JSON schema. Validate before import, preview changes, keep demo imports in the demo namespace, and add round-trip and malformed-file claims/tests. An AI feature is not required for this deterministic workflow, and no provider key is embedded.

## Copy audit

Counts treat hyphenated compounds as one word and ignore punctuation. The landing inventory includes headings, labels, links, actions, alt text, and the three demo-banner strings so non-sentence UI copy is not silently skipped. Sample record values are test data rather than landing copy.

### Landing page and demo-entry copy

| ID | Words | Exact copy | Flag |
| --- | ---: | --- | --- |
| L01 | 3 | Skip to content | — |
| L02 | 2 | Changelog Watch | — |
| L03 | 1 | Demo | — |
| L04 | 3 | How it works | — |
| L05 | 1 | Privacy | — |
| L06 | 3 | INTEGRATION CHANGELOG WATCH | F-1-15 decorative duplicate |
| L07 | 6 | Turn vendor changes into owned actions | F-1-14 jargon |
| L08 | 10 | For engineers who maintain payment, auth, analytics, or messaging integrations. | — |
| L09 | 5 | Try it with sample data | —; result-naming verb |
| L10 | 7 | See matched notices, owners, versions, and checks. | Covered by `sample-action-cards` and its assertions |
| L11 | 6 | Rules are written by your team. | F-1-5 unlisted claim; F-1-14 terminology |
| L12 | 7 | Scans run only when you request them. | Covered by `requested-scans` |
| L13 | 7 | Your workspace is separated from other visitors. | Covered by `workspace-boundary` |
| L14 | 9 | Paper release-note cards travel into a small action card. | Useful image alt text |
| L15 | 3 | Original paper-cut illustration. | F-1-15 non-informational caption |
| L16 | 3 | Your owned queue | F-1-14 jargon; use “Assigned action cards” |
| L17 | 4 | No actions need acknowledgement | — |
| L18 | 3 | Scan watched feeds | —; result-naming verb |
| L19 | 3 | Add a watch | —; result-naming verb |
| L20 | 2 | Action cards | — |
| L21 | 4 | No action cards yet | — |
| L22 | 10 | Add a feed, rule, owner, dependency version, and check command. | F-1-14 “rule”; use “keywords” |
| L23 | 6 | Matched release notes will appear here. | F-1-3 unlisted claim |
| L24 | 4 | Add your first watch | —; result-naming verb |
| L25 | 2 | Watched feeds | — |
| L26 | 4 | Nothing is watched yet. | — |
| L27 | 5 | Export action cards as CSV | Covered by `csv-export`; result-naming verb |
| L28 | 3 | How it works | — |
| L29 | 7 | Give each vendor change a next step | — |
| L30 | 4 | Watch a public feed. | — |
| L31 | 11 | Paste a changelog or RSS address you are allowed to read. | — |
| L32 | 3 | Match your words. | F-1-14 vague/inconsistent; use “Choose keywords.” |
| L33 | 9 | Use rules like “webhook”, “deprecation”, or an API version. | F-1-14 terminology; use “keywords” |
| L34 | 4 | Run the right check. | — |
| L35 | 11 | Each matching notice includes an owner, dependency version, and check command. | Covered by `sample-action-cards` assertions |
| L36 | 3 | Hosted workspace scope | F-1-14 jargon; use “Free hosted limits” |
| L37 | 11 | The hosted dashboard is a free private workspace for three feeds. | F-1-4 unlisted claims |
| L38 | 14 | It does not provide shared team workspaces, accounts, unlimited watches, or a paid plan. | F-1-9 unlisted claims |
| L39 | 10 | Use the local CLI for repository-owned mappings with more feeds. | F-1-6 unlisted claim; F-1-14 jargon |
| L40 | 5 | What this does not do | — |
| L41 | 5 | It does not scan automatically. | Covered by `requested-scans` |
| L42 | 7 | Private, loopback, and link-local addresses are blocked. | Covered by `workspace-boundary` |
| L43 | 6 | Vendor notices become owned integration actions. | F-1-14 jargon |
| L44 | 1 | Terms | — |
| L45 | 4 | Built by Param Factory | — |
| L46 | 2 | build `99f0ca341a13140030b4f50272b4b399c54cbd57` | —; live build identifier |
| D01 | 6 | Demo — sample data, nothing is saved | Covered by `demo-local` and `demo-isolation-transitions` |
| D02 | 2 | Reset demo | —; result-naming verb |
| D03 | 3 | Start for real | F-1-16 non-result-naming button |

No landing sentence exceeds 22 words and no banned marketing adjective appears.

### README copy

| ID | Words | Exact copy | Flag |
| --- | ---: | --- | --- |
| R01 | 3 | Integration Changelog Watch | — |
| R02 | 8 | Turn vendor changelog changes into owned integration actions. | F-1-14 jargon and repeated “changes” |
| R03 | 12 | It is for engineers who maintain payment, auth, analytics, or messaging integrations. | — |
| R04 | 20 | Add a public changelog or RSS feed, matching words, an owner, the affected dependency version, and a local check command. | F-1-14 terminology |
| R05 | 13 | Scan when you want to review notices and turn matches into action cards. | — |
| R06 | 2 | Run locally | — |
| R07 | 12 | The container starts with no database setting because it mounts durable `/data`. | Covered by startup/persistence claims |
| R08 | 20 | On a host without that mount, use an explicit local path: `DATABASE_URL='sqlite:changelog-watch.db?mode=rwc' cargo run`. | — |
| R09 | 7 | For frontend development, use `npm run dev`. | — |
| R10 | 9 | Run `npm test` for shipped sample and container-toolchain checks. | — |
| R11 | 29 | Run `npm run typecheck` for TypeScript checks, `cargo test` for server checks, and `npm run test:browser` after `npm run build` for browser, keyboard, mobile, privacy, and accessibility coverage. | F-1-11 over 22 words |
| R12 | 13 | The exact browser commands for each published claim live in `.factory/claims.json`. | — |
| R13 | 9 | Open `http://localhost:8080/demo` for a one-click sandbox. | — |
| R14 | 16 | Demo data uses the `demo:integration-changelog-watch` browser storage namespace and is discarded by **Start for real**. | Covered by demo isolation; F-1-16 button name |
| R15 | 2 | Workspace API | — |
| R16 | 28 | The container exposes `GET /health`, `POST /api/workspaces`, `GET|POST /api/watches`, `PUT|DELETE /api/watches/:id`, `GET /api/actions`, `POST /api/actions/:id`, and `POST /api/scan`. | F-1-7 unlisted claim; F-1-12 over 22 words |
| R17 | 14 | Create a workspace first, then send its browser-held bearer token with every workspace request. | —; appropriate API terminology |
| R18 | 14 | Source URLs must be public `http` or `https` addresses; private network addresses are rejected. | Covered by `workspace-boundary` |
| R19 | 2 | CLI demo | — |
| R20 | 7 | `demo` prints the bundled Markdown action-card sample. | Covered by `cli-demo-local` |
| R21 | 26 | `scan --config` reads a repository-owned JSON watch mapping, writes new Markdown action cards under `.integration-changelog-watch/actions/`, and stores hashes plus acknowledgement state in `.integration-changelog-watch/state.json`. | F-1-13 over 22 words |
| R22 | 12 | Each card prints its hash-derived action ID; pass that ID to `ack`. | Covered by `cli-repository-workflow` |
| R23 | 10 | Acknowledgement updates both the state file and the Markdown card. | Covered by `cli-repository-workflow` |
| R24 | 16 | The shipped example uses the bundled `examples/sample-feed.xml`, so it works without a network request. | Covered by `cli-shipped-mapping-local` |
| R25 | 2 | Hosted scope | — |
| R26 | 10 | The hosted dashboard is deliberately a free, private, three-watch workspace. | F-1-4 unlisted claims; “deliberately” adds no information |
| R27 | 13 | It has no account system, shared team workspaces, unlimited-watch tier, or paid plan. | F-1-9 unlisted claims |
| R28 | 12 | A browser-held workspace token is not a team identity or billing entitlement. | Covered by `workspace-boundary` for isolation; useful limitation |
| R29 | 15 | Teams can keep larger mappings and action cards in their repository with the local CLI. | F-1-6 unlisted “larger” claim |
| R30 | 8 | Hosted team collaboration is not offered or implied. | F-1-10 unlisted duplicate |
| R31 | 1 | Deploy | — |
| R32 | 13 | Build the repository image with `docker build --build-arg BUILD_SHA=dev -t integration-changelog-watch .`. | —; instruction, not a result claim |
| R33 | 15 | The container uses the current stable Rust image and builds with `cargo build --release --locked`. | F-1-8 unlisted/time-sensitive claim |
| R34 | 8 | It starts with only `PORT` required (default `8080`). | Covered by `port-only-startup` |
| R35 | 22 | The shipped Container App configuration uses one replica and mounts durable Azure Files at `/data`, where SQLite persists at `/data/changelog-watch.db`. | Covered by topology and persistence claims |
| R36 | 17 | A production topology guard closes workspace APIs and restores that configuration if a generic deploy removes it. | Covered by `single-replica-durable-data` |
| R37 | 13 | SQLite uses Azure Files-compatible dot-file locks, so do not raise the replica count. | Operational warning; topology claim covers the enforced one-replica result |
| R38 | 10 | See `/privacy` and `/terms` for data handling and source rules. | — |
| R39 | 2 | MIT licensed. | Confirmed by `LICENSE` |
| R40 | 4 | Built by Param Factory. | — |
| R41 | 2 | creates `dist/` | —; code-block comment |
| R42 | 13 | Copy the action ID printed in the new Markdown card, then acknowledge it: | —; code-block instruction |

Terminology target after fixes: vendor source → **feed**; saved monitor → **watch**; match input → **keywords**; output → **action card**; assignee → **owner**; isolated hosted data → **workspace**; local tool → **CLI**.

## Demo and sandbox evidence

- One click exists from `/` and direct `/demo` works.
- Sample data is realistic: three vendor watches and two action cards with owners, versions, and commands.
- The required persistent banner, **Reset demo**, and **Start for real** are present.
- Acknowledge writes only `demo:integration-changelog-watch`. Reset removes that key and restores the seed. Starting the real workspace removes the demo key.
- In a direct fresh `/demo` context, the complete standard-flow request list is the demo document, same-origin JS, same-origin CSS, and the same-origin hero image. There is no `/api/` or third-party request. See `review-1-artifacts/structure.json`.
- A pre-existing real workspace and token were byte-for-byte unchanged through acknowledge, reset, and exit in the landing-to-demo flow. See `review-1-artifacts/demo-mobile.json`.
- The sandbox isolation passes. The initial visibility failure is F-1-1.

## Claims run

Every command was run exactly as listed in `.factory/claims.json` from clean clone `/tmp/icw-review-1-9qlY2f` after `npm ci`. Browser claims ran in both configured projects.

| Claim ID | Result | Evidence from exact command |
| --- | --- | --- |
| `sample-action-cards` | PASS | 2 Playwright tests passed |
| `csv-export` | PASS | 2 Playwright tests passed |
| `demo-local` | PASS | 2 Playwright tests passed |
| `workspace-boundary` | PASS | 2 Playwright tests passed |
| `redirecting-feeds` | PASS | 1 Cargo test passed |
| `requested-scans` | PASS | 2 Playwright tests passed |
| `cli-repository-workflow` | PASS | 2 Playwright tests passed in temporary repositories |
| `cli-demo-local` | PASS | 2 Playwright tests passed with the recording proxy |
| `database-persistence` | PASS | 1 Cargo test passed |
| `demo-isolation-transitions` | PASS | 2 Playwright tests passed |
| `cli-shipped-mapping-local` | PASS | 2 Playwright tests passed with the recording proxy |
| `port-only-startup` | PASS | 1 Cargo test passed |
| `single-replica-durable-data` | PASS | 1 Cargo test passed |

There is no failing listed claim test. F-1-3 through F-1-10 are claim-like copy not adequately represented in the registry.

## History recheck

No earlier `.factory/review-*.md` or `.factory/polish-*.md` exists. The earlier `.factory/handoff.md` was read in full.

- Its release-ready assertions for all 13 claims, first-read clarity, demo storage isolation, route operation, mobile layout, request privacy, and serious/critical accessibility checks were independently reconfirmed.
- Its known moderate Axe issue is still present and is blocking F-1-2 under this review’s explicit history rule.
- Its Docker-engine limitation is an environment note, not a product finding. The claim-specific locked Cargo tests pass; this review did not represent that as a successful Docker image build.

## Structure, links, accessibility, and visual identity

Verified passes:

- `/`, `/demo`, `/privacy`, and `/terms` return 200; an unknown route returns the designed 404 with a return-home action.
- Each tested route has `lang="en"`, one `<main>`, and one `<h1>`. Titles follow the required product/route pattern and remain below 60 characters.
- Canonicals change by SPA route. The favicon, 180 px apple-touch icon, social image, `robots.txt`, and `sitemap.xml` load with 200 responses.
- All discovered internal links and both sample vendor links returned 200. There are no dead links in the crawled UI.
- Clicking Privacy moves focus to its `<h1>`; browser Back restores `/`, its title, and focus on the landing `<h1>`. `/#how` deep-links to the section.
- Keyboard, 390 px touch-target, 195 px reflow, designed-404, and live accessibility tests pass. Full Axe has only F-1-2.
- No console errors occur on successful routes. The expected main-document 404 resource message is the only console error for the intentional unknown URL.
- The first-load production JS is 5.54 kB gzip, below the 150 kB requirement.
- The warm-paper, clipped-card, indigo-thread visual identity and original paper-cut art are distinct from a generic gradient-card SaaS template and match `.factory/design.md`.
- No analytics, advertising code, third-party font, AI provider key, Azure model key, or payment-provider integration is present.

Metadata and 404 differences remain as F-1-17 and F-1-18.

## Local quality checks

- `npm test`: PASS, 5 tests.
- `npm run typecheck`: PASS.
- `npm run build`: PASS; `dist/` produced, JS 14.42 kB raw / 5.54 kB gzip.
- `PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run test:a11y`: PASS under its serious/critical-only assertion; a separate full Axe run exposes F-1-2.

## What would make this perfect

Put the populated action board in the first demo viewport, clear the remaining Axe violation, register and test every published claim, simplify the flagged copy, make route previews and the 404 shell complete, and let watch configuration round-trip between the hosted dashboard and CLI JSON. After those changes, rerun this entire checklist from fresh phone/desktop contexts and require zero findings—including zero full-Axe violations and no unlisted claim.
