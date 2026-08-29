# Adversarial first-read review 3

- Product: Integration Changelog Watch
- Live URL: <https://integration-changelog-watch.sociobot.in>
- Review date: 2026-08-29 UTC
- Repository review base: `34b9c8ecebbe3f0cf22793737996960ddb11441f`
- Live `/health` build: `818b868c0ba7ecece8fdae9b4abb4d6b927bdae1`
- Verdict: **FAIL**

There are four findings. One is blocking because an earlier copy finding remains half-fixed. All 21 registered claim commands pass, but one README claim is not registered. A pass requires zero findings and no untested claim.

`.factory/brief.json` is absent. The review used the product contract, `.factory/design.md`, the live product, README, claims registry, and full review/polish/handoff history. The missing optional brief is noted because it limits opportunity-traceability, but it is not itself a finding.

## Cold first screen, before scrolling

Fresh Chromium contexts were used at 390 × 844 and 1440 × 900.

| Question | Answer at both widths | Exact first-screen text |
| --- | --- | --- |
| What does this do? | It turns matching vendor changes into assigned action cards. | “Turn vendor changes into assigned action cards” |
| For whom? | Engineers maintaining payment, authentication, analytics, or messaging integrations. | “For engineers who maintain payment, authentication, analytics, or messaging integrations.” |
| What should I click first? | **Try it with sample data**. | “Try it with sample data” and “See matched notices, owners, versions, and checks.” |

All three questions are answerable without scrolling, so there is no blocking first-read finding. The facts below the action omit required online and price information; that is F-3-4.

## Findings

### Blocking

#### F-1-14 (reopened; round-3 alias F-3-1) — The earlier jargon finding is only partially fixed

- Exact quote/location: landing section heading and README heading, **“Hosted workspace scope”**.
- History: review 1 explicitly included this quote in F-1-14 and proposed **“Free hosted limits”**. Polish 1 and review 2 marked F-1-14 fixed, but the exact phrase remains in `frontend/src/main.ts`, `README.md`, and the live page.
- Why this fails: “scope” is abstract project language. The section actually explains a three-watch limit and a four-watch CLI alternative. The history rule makes a half-fixed earlier finding blocking under its original ID.
- Concrete fix: change both headings to **“Hosted workspace limits”**. Keep the two tested capacity sentences below it. Add a copy assertion that rejects “Hosted workspace scope” so the regression cannot be marked fixed again.

### Major

#### F-3-2 — The Azure Files locking sentence is an unlisted claim

- Exact quote/location: README, Deploy: **“SQLite uses Azure Files-compatible dot-file locks, so do not raise the replica count.”**
- Why this fails: `.factory/claims.json` has no claim entry for Azure Files-compatible locking. `claim_port_only_startup_configuration` incidentally asserts `vfs=unix-dotfile`, while `single-replica-durable-data` separately asserts one replica. That is not the required one claim → one registered test mapping for this combined sentence.
- Concrete fix: split the sentence. Add `azure-files-dotfile-locking` to `.factory/claims.json` with a dedicated `@claim:azure-files-dotfile-locking` test that asserts the runtime SQLite URL uses `vfs=unix-dotfile`. Keep the replica warning under `single-replica-durable-data`, or remove the sentence.

#### F-3-3 — A product called “Watch” cannot watch without a manual visit

- Exact quote/location: landing limitation, **“It does not scan automatically.”** The only live action is **“Scan watched feeds.”**
- Why this fails: a normal integration owner expects a changelog watch to notice changes without requiring them to remember to open the dashboard and press a button. The product honestly states the limitation, but the missing sync is the obvious next capability implied by its name and job.
- Concrete fix: add an opt-in scan schedule per watch. Show last run, next run, and failures; deduplicate notices with the existing notice key; create action cards server-side; and offer an optional notification destination. Add clean-workspace claims for schedule opt-in, deduplication, failure visibility, and no scan before consent. This deterministic workflow does not need AI.

### Minor

#### F-3-4 — The three first-screen facts omit online and price truth

- Exact location: the three lines below the primary action are **“You choose the matching keywords.”**, **“Scans run only when you request them.”**, and **“Your workspace is separated from other visitors.”**
- Why this fails: the mandatory first-screen shape calls for privacy, offline/online, and price facts. The visitor gets one privacy fact and two feature facts, but cannot tell whether scanning works offline or whether the hosted workspace costs money.
- Concrete fix: keep the privacy fact and add plain, tested truths such as **“Scanning public feeds requires internet.”** and **“No account or payment is required.”** Register both claims; the latter test should prove that a fresh visitor can create and use the three-watch workspace without signup or billing requests.

## Copy audit

Counts use whitespace-delimited words; hyphenated compounds, paths, and URLs each count as one token. No sentence exceeds 22 words and no banned marketing adjective appears. F-3-1 is the only landing/README jargon flag. F-3-2 is the only unlisted claim in these two sources.

### Landing page

| ID | Words | Exact copy | Result |
| --- | ---: | --- | --- |
| L1 | 7 | Turn vendor changes into assigned action cards | Pass headline |
| L2 | 10 | For engineers who maintain payment, authentication, analytics, or messaging integrations. | Pass audience sentence |
| L3 | 7 | See matched notices, owners, versions, and checks. | Listed: `sample-action-cards` |
| L4 | 5 | You choose the matching keywords. | Listed: `keyword-edit`; see F-3-4 for fact selection |
| L5 | 7 | Scans run only when you request them. | Listed: `requested-scans`; see F-3-3/F-3-4 |
| L6 | 7 | Your workspace is separated from other visitors. | Listed: `workspace-boundary` |
| L7 | 3 | Assigned action cards | Pass label |
| L8 | 4 | No actions need acknowledgement | Pass empty summary |
| L9 | 2 | Action cards | Pass heading |
| L10 | 4 | No action cards yet | Pass empty heading |
| L11 | 10 | Matched release notes appear here after you scan a feed. | Listed: `hosted-scan-result` |
| L12 | 2 | Watched feeds | Pass heading |
| L13 | 4 | Nothing is watched yet. | Pass empty state |
| L14 | 3 | How it works | Pass section label |
| L15 | 7 | Give each vendor change a next step | Pass heading |
| L16 | 4 | Watch a public feed. | Pass instruction |
| L17 | 11 | Paste a changelog or RSS address you are allowed to read. | Pass instruction |
| L18 | 3 | Choose your keywords. | Pass instruction |
| L19 | 9 | Use keywords like “webhook”, “deprecation”, or an API version. | Pass example |
| L20 | 4 | Run the right check. | Pass instruction |
| L21 | 11 | Each matching notice includes an owner, dependency version, and check command. | Listed: `hosted-scan-result` |
| L22 | 3 | Hosted workspace scope | **F-1-14 / F-3-1: jargon** |
| L23 | 8 | The hosted workspace holds up to three watches. | Listed: `hosted-watch-limit` |
| L24 | 8 | Use the local CLI for a four-watch mapping. | Listed: `cli-more-feeds` |
| L25 | 5 | What this does not do | Pass heading |
| L26 | 5 | It does not scan automatically. | Listed: `requested-scans`; F-3-3 missed leverage |
| L27 | 7 | Private, loopback, and link-local addresses are blocked. | Listed: `workspace-boundary` |
| L28 | 6 | Vendor notices become assigned action cards. | Listed: `hosted-scan-result` |
| L29 | 9 | Paper release-note cards move into an assigned action card. | Pass image alt text |

Controls and links: **Try it with sample data** (5), **Scan watched feeds** (3), **Add a watch** (3), **Add your first watch** (4), **Export watch file** (3), **Import watch file** (3), and **Export action cards as CSV** (5) all use result-naming verbs. **Demo**, **How it works**, **Privacy**, and **Terms** are links, not buttons.

### README

| ID | Words | Exact copy | Result |
| --- | ---: | --- | --- |
| R1 | 7 | Turn vendor changes into assigned action cards. | Listed: `hosted-scan-result` |
| R2 | 12 | It is for engineers who maintain payment, authentication, analytics, or messaging integrations. | Pass audience sentence |
| R3 | 19 | Add a public changelog or RSS feed, keywords, an owner, the affected dependency version, and a local check command. | Pass instruction |
| R4 | 12 | Scan when you are ready to review notices and create action cards. | Listed: `hosted-scan-result` |
| R5 | 7 | The container uses durable `/data` by default. | Listed: `database-persistence` |
| R6 | 10 | On a host without that mount, use `DATABASE_URL='sqlite:changelog-watch.db?mode=rwc' cargo run`. | Pass instruction |
| R7 | 7 | For frontend development, use `npm run dev`. | Pass instruction |
| R8 | 10 | Run `npm test` and `npm run typecheck` for code checks. | Pass instruction |
| R9 | 11 | After `npm run build`, run `npm run test:browser` for browser coverage. | Pass instruction |
| R10 | 7 | The exact claim commands live in `.factory/claims.json`. | Pass instruction |
| R11 | 6 | Open `http://localhost:8080/demo` for a one-click sandbox. | Listed: `demo-local` |
| R12 | 6 | Demo data uses `demo:integration-changelog-watch` browser storage. | Listed: `demo-local` |
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
| R23 | 13 | `scan --config` reads a JSON watch file and writes Markdown cards under `.integration-changelog-watch/actions/`. | Listed: `cli-repository-workflow` |
| R24 | 8 | It stores hashes and acknowledgement state in `.integration-changelog-watch/state.json`. | Listed: `cli-repository-workflow` |
| R25 | 12 | Each card prints its hash-derived action ID; pass that ID to `ack`. | Listed: `cli-repository-workflow` |
| R26 | 10 | Acknowledgement updates both the state file and the Markdown card. | Listed: `cli-repository-workflow` |
| R27 | 12 | The shipped example uses `examples/sample-feed.xml`, so it works without a network request. | Listed: `cli-shipped-mapping-local` |
| R28 | 8 | The hosted workspace holds up to three watches. | Listed: `hosted-watch-limit` |
| R29 | 8 | Use the local CLI for a four-watch mapping. | Listed: `cli-more-feeds` |
| R30 | 12 | Build the repository image with `docker build --build-arg BUILD_SHA=dev -t integration-changelog-watch .`. | Pass instruction |
| R31 | 8 | The build stage uses the official `rust:1-alpine` image. | Listed: `container-build-stage` |
| R32 | 8 | It starts with only `PORT` required (default `8080`). | Listed: `port-only-startup` |
| R33 | 20 | The shipped Container App configuration uses one replica and mounts durable Azure Files at `/data`, where SQLite persists at `/data/changelog-watch.db`. | Listed: `single-replica-durable-data`, `database-persistence` |
| R34 | 17 | A production topology guard closes workspace APIs and restores that configuration if a generic deploy removes it. | Listed: `single-replica-durable-data` |
| R35 | 13 | SQLite uses Azure Files-compatible dot-file locks, so do not raise the replica count. | **F-3-2: unlisted combined claim** |
| R36 | 10 | See `/privacy` and `/terms` for data handling and source rules. | Pass instruction |
| R37 | 2 | MIT licensed. | Confirmed by `LICENSE` |
| R38 | 4 | Built by Param Factory. | Pass attribution |
| R39 | 2 | creates `dist/` | Pass code comment |
| R40 | 13 | Copy the action ID printed in the new Markdown card, then acknowledge it. | Pass code comment |

README headings/fragments: **Integration Changelog Watch** (3), **Run locally** (2), **Workspace API** (2), **Watch files** (2), **CLI demo** (2), **Hosted workspace scope** (3; F-1-14/F-3-1), and **Deploy** (1). API result cells are concrete: service/build identity; workspace token creation; watch read/create/import/update/delete; action read/acknowledge; and requested scan.

Terminology is otherwise consistent: vendor source → **feed**; saved monitor → **watch**; match input → **keywords**; output → **action card**; assignee → **owner**; isolated hosted data → **workspace**; local tool → **CLI**.

## Demo and sandbox

The one-click and direct demo paths pass.

- `/` → **Try it with sample data** opens `/demo` in one click.
- At 390 × 844, the first viewport contains the banner, sample heading, Stripe title, owner **Maya · Payments**, dependency **stripe-node 16.2**, and check **pnpm test:stripe**.
- The sample has two realistic actions and three vendor watches. It is not placeholder text.
- **Reset demo** removes `demo:integration-changelog-watch` and restores the seeded Stripe action.
- Acknowledge writes only `demo:integration-changelog-watch`. Pre-existing `icw:workspace` and `icw:workspace-token` values remain byte-for-byte unchanged through acknowledge and reset.
- Before leaving demo, requests are only `/demo` plus same-origin JS and CSS. There are no API, analytics, font-CDN, advertising, or cross-origin requests.
- **Start a private workspace** removes demo state before leaving the sandbox. The registered transition test passes from a fresh browser.
- No offline-reload claim is published. The demo sample itself is bundled and requires no data request.

## Claims

From clean clone `/tmp/icw-review-3-jDUdzM`, `npm ci` and `npm run test:claims` ran every literal command in `.factory/claims.json`. All 21 passed; the runner confirmed port 8080 was released between commands.

| Claim | Result |
| --- | --- |
| `sample-action-cards` | PASS |
| `csv-export` | PASS |
| `demo-local` | PASS |
| `demo-isolation-transitions` | PASS |
| `workspace-boundary` | PASS |
| `hosted-scan-result` | PASS |
| `hosted-watch-limit` | PASS |
| `keyword-edit` | PASS |
| `requested-scans` | PASS |
| `redirecting-feeds` | PASS |
| `watch-file-portability` | PASS |
| `watch-file-rejection-preserves-watches` | PASS |
| `cli-more-feeds` | PASS |
| `cli-repository-workflow` | PASS |
| `cli-demo-local` | PASS |
| `cli-shipped-mapping-local` | PASS |
| `api-contract` | PASS |
| `container-build-stage` | PASS |
| `database-persistence` | PASS |
| `port-only-startup` | PASS |
| `single-replica-durable-data` | PASS |

F-3-2 is a claim-registry completeness failure, not a failure of these listed commands.

## History recheck

Read in full: `.factory/review-1.md`, `.factory/review-2.md`, `.factory/polish-1.md`, `.factory/polish-2.md`, and the pre-review `.factory/handoff.md`. Each earlier finding was checked on the live site and in current source.

| Earlier ID | Current verification |
| --- | --- |
| F-1-1 | Fixed: `/demo` begins with populated sample work and all required sample fields fit in the first phone viewport. |
| F-1-2 | Fixed: watched feeds use a labelled `<section>`; full Axe reports zero violations. |
| F-1-3 | Fixed: `hosted-scan-result` exists and its fixture-backed test passes. |
| F-1-4 | Fixed: the unproved price wording is absent and `hosted-watch-limit` passes. |
| F-1-5 | Fixed: live copy uses “keywords”; create/edit/reload coverage passes. |
| F-1-6 | Fixed: the four-watch CLI claim and temporary-repository test pass. |
| F-1-7 | Fixed: the API table remains and `api-contract` covers every documented method. |
| F-1-8 | Fixed: “current stable” is absent; the official `rust:1-alpine` contract test passes. |
| F-1-9 | Fixed: unproved team/plan exclusions remain removed. |
| F-1-10 | Fixed: the duplicate collaboration exclusion remains removed. |
| F-1-11 | Fixed: README verification commands are split below 22 words. |
| F-1-12 | Fixed: endpoint inventory remains in a table rather than one dense sentence. |
| F-1-13 | Fixed: CLI input, output, and state are separate sentences. |
| F-1-14 | **Half-fixed/blocking:** “keywords” and “assigned action cards” are consistent, but the exact flagged heading “Hosted workspace scope” remains live and in README. |
| F-1-15 | Fixed: the redundant hero eyebrow and illustration-provenance caption are absent. |
| F-1-16 | Fixed: the banner uses **Start a private workspace** and says it discards the demo. |
| F-1-17 | Fixed: every app route updates title, description, canonical, OG, and Twitter metadata. |
| F-1-18 | Fixed: the 404 has shared navigation, metadata, footer, one h1, one main, and a return-home action. |
| F-1-19 | Fixed: watch-file import/export, preview, malformed rejection, and demo isolation remain present and tested. |
| F-2-1 | Fixed: a cold `/#how` load reaches the target at `scrollY: 1984`; Back restores the hash and target position. |
| F-2-2 | Fixed: “authentication” replaces “auth” on the live first screen and in README. |

## Structure, links, accessibility, and identity

- `/`, `/demo`, `/privacy`, and `/terms` return 200. `/missing-review-3` returns a designed 404 with a way home.
- Each route has `lang="en"`, one `<main>`, one `<h1>`, ordered headings, route-specific title/description/canonical/OG/Twitter metadata, favicon, apple-touch icon, and social image.
- Titles follow the product/route pattern and remain below 60 characters.
- `robots.txt`, `sitemap.xml`, favicon, apple-touch icon, social card, and hero art return 200.
- All internal route links and both external sample vendor links return 200. Same-page skip links and `/#how` targets work.
- Route navigation focuses the new h1 and announces it. Back restores route, title, focus, and the cold-hash scroll position.
- Successful routes produce no console errors. The intentional 404 document produces only the browser's expected failed-document 404 message.
- Full Axe WCAG 2 A/AA checks return zero violations on `/`, `/demo`, `/privacy`, `/terms`, and the designed 404. The live 20-test accessibility/metadata/routing suite passes.
- `/opt/fleet/lib/verify-url.sh` passes: title, language, one h1, main, image alt text, labelled buttons, and zero landing-page console errors.
- Mobile width is 390 CSS pixels with no horizontal overflow. The reduced-motion rule removes transitions and animations. Existing keyboard/touch tests pass.
- The production JS is 6.95 kB gzip, below the 150 kB limit.
- The warm paper, indigo thread, clipped cards, Georgia/system type pairing, and original paper-cut art match `.factory/design.md`. The result is recognisable and is not a generic SaaS card/gradient template.
- CSP and request logs show no third-party scripts/fonts, analytics, embedded model/provider key, or payment integration. No decorative AI feature is present.

## Additional local verification

The clean clone also passed:

- `npm test`: 9 tests.
- `npm run typecheck`.
- `npm run build`: `dist/` produced.
- `cargo test --locked`: 23 tests.
- `PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run test:a11y`: 20 tests.

## What would make this perfect

Replace “Hosted workspace scope,” register the Azure Files locking claim with one dedicated test, state online and price facts on the first screen, and add opt-in scheduled monitoring with visible run status. Then rerun all 21 claim commands, the live route/accessibility audit, and the complete history check. A perfect result has zero findings and no unregistered claim.
