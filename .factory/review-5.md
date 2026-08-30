# Adversarial first-read review 5

- Product: Integration Changelog Watch
- Live URL: <https://integration-changelog-watch.sociobot.in>
- Review date: 2026-08-30 UTC
- Repository review base: `00c0ef8939d50bac496ff010b487f86953b64f08`
- Live `/health` build: `2e443911e1e63e4d998310c84df07d1bda558630`
- Verdict: **PASS**

There are zero findings. The live phone experience answers the first-read questions, the one-click demo is isolated and immediately useful, every registered claim passed from a clean clone, and all earlier findings are fixed in both the live service and current source.

## Cold first screen

Fresh Chromium contexts at 390 × 844 and 1440 × 900 were used with empty browser storage. Screenshots were captured at `/tmp/icw-live-mobile.png` and `/tmp/icw-live-desktop.png` during this review.

| Question | Answer visible before scrolling |
| --- | --- |
| What does this do? | It turns vendor changes into assigned action cards. |
| For whom? | Engineers who maintain payment, authentication, analytics, or messaging integrations. |
| What should I click first? | **Try it with sample data**. The adjacent text says it will show matched notices, owners, versions, and checks. |

The exact first-screen copy is “Turn vendor changes into assigned action cards”, “For engineers who maintain payment, authentication, analytics, or messaging integrations.”, and “Try it with sample data”. The phone screen also states the online, payment, and workspace-separation facts. No blocking first-read finding applies.

## Demo and sandbox

- The landing CTA enters `/?demo=1` in one click. Direct `/demo` also works.
- At 390 × 844, the initial demo screen contains the persistent **“Demo — sample data, nothing is saved”** banner, **Reset demo**, **Start a private workspace**, the Stripe sample action title, matched keyword, owner, dependency version, and check command. Screenshot: `/tmp/icw-demo-mobile.png`.
- The sample uses three realistic feeds and two action cards. It is visibly a working action board, not a repeated marketing screen.
- A fresh request log for landing → demo → acknowledge → reset contained only same-origin document, JS, CSS, and image requests; it contained no `/api/` request, analytics, advertising, remote font, or third-party request.
- Acknowledging a sample writes only `demo:integration-changelog-watch`; no `icw:*` real-workspace key is written. **Reset demo** removes that key and restores the original unacknowledged Stripe sample. The existing `demo-isolation-transitions` claim also exercises explicit exit to a private workspace.
- The offline demo scan claim passed. The product makes the accurate online requirement explicit instead of making an offline-reload promise.

## Claims

`.factory/claims.json` contains 28 entries. From fresh clone `/tmp/icw-review5-CvacYH`, `npm ci && npm run test:claims` completed successfully. The runner invokes each literal registered command and checks that port 8080 is released after each one. Its final Playwright status is `passed` with no failed tests.

This covers the sample board, CSV export, demo isolation, workspace isolation, hosted scan matching, watch limits, keyword edits, manual and scheduled scans, webhook destination, watch-file portability, CLI workflows, API contract, container/runtime configuration, and persistence. Landing and README claim-like copy maps to these entries; no unlisted claim was found.

## Copy audit

Counts use whitespace-delimited words, treating code paths and URLs as one token. The inventory includes all prose sentences plus user-facing headings and actions so labels are not silently excluded. No sentence exceeds 22 words. No jargon, marketing adjective, mood heading, inconsistent core term, or non-result-naming button was found. The terms are consistent: **feed**, **watch**, **keywords**, **action card**, **owner**, **workspace**, **schedule**, and **CLI**.

### Landing page prose

| ID | Words | Copy |
| --- | ---: | --- |
| L1 | 7 | Turn vendor changes into assigned action cards |
| L2 | 10 | For engineers who maintain payment, authentication, analytics, or messaging integrations. |
| L3 | 7 | See matched notices, owners, versions, and checks. |
| L4 | 5 | Scanning public feeds requires internet. |
| L5 | 6 | No account or payment is required. |
| L6 | 7 | Your workspace is separated from other visitors. |
| L7 | 10 | Matched release notes appear here after you scan a feed. |
| L8 | 7 | Give each vendor change a next step |
| L9 | 4 | Watch a public feed. |
| L10 | 11 | Paste a changelog or RSS address you are allowed to read. |
| L11 | 3 | Choose your keywords. |
| L12 | 9 | Use keywords like “webhook”, “deprecation”, or an API version. |
| L13 | 4 | Run the right check. |
| L14 | 11 | Each matching notice includes an owner, dependency version, and check command. |
| L15 | 8 | The hosted workspace holds up to three watches. |
| L16 | 8 | Use the local CLI for a four-watch mapping. |
| L17 | 12 | Turn on a schedule for any watch when you want automatic scans. |
| L18 | 10 | Scheduled watches show the last run, next run, and errors. |
| L19 | 9 | Add an optional public webhook destination for run summaries. |
| L20 | 7 | Private, loopback, and link-local addresses are blocked. |
| L21 | 6 | Vendor notices become assigned action cards. |

Headings/actions checked separately: **Changelog Watch** (2), **Demo** (1), **How it works** (3), **Privacy** (1), **Assigned action cards** (3), **Scan watched feeds** (3), **Add a watch** (3), **Add your first watch** (4), **Export watch file** (3), **Import watch file** (3), **Export action cards as CSV** (5), **Hosted workspace limits** (3), **Scheduled scans** (2), and **Source safeguards** (2). All are useful labels or result-naming verbs.

### README prose

| ID | Words | Copy |
| --- | ---: | --- |
| R1 | 7 | Turn vendor changes into assigned action cards. |
| R2 | 12 | It is for engineers who maintain payment, authentication, analytics, or messaging integrations. |
| R3 | 19 | Add a public changelog or RSS feed, keywords, an owner, the affected dependency version, and a local check command. |
| R4 | 12 | Scan when you are ready to review notices and create action cards. |
| R5 | 10 | Turn on a per-watch schedule when you need automatic scans. |
| R6 | 7 | The container uses durable `/data` by default. |
| R7 | 10 | On a host without that mount, use `DATABASE_URL='sqlite:changelog-watch.db?mode=rwc' cargo run`. |
| R8 | 7 | For frontend development, use `npm run dev`. |
| R9 | 10 | Run `npm test` and `npm run typecheck` for code checks. |
| R10 | 11 | After `npm run build`, run `npm run test:browser` for browser coverage. |
| R11 | 7 | The exact claim commands live in `.factory/claims.json`. |
| R12 | 6 | Open `http://localhost:8080/?demo=1` for a one-click sandbox. |
| R13 | 6 | Demo data uses `demo:integration-changelog-watch` browser storage. |
| R14 | 6 | Start a private workspace discards it. |
| R15 | 11 | The container exposes health, workspace, watch, schedule, action, and scan endpoints. |
| R16 | 4 | Create a workspace first. |
| R17 | 9 | Send its browser-held bearer token with every workspace request. |
| R18 | 14 | Source URLs must be public `http` or `https` addresses; private network addresses are rejected. |
| R19 | 13 | Use Export watch file to download your watches in the CLI JSON schema. |
| R20 | 10 | Use Import watch file to preview one to three watches. |
| R21 | 9 | A rejected private-workspace import leaves the current watches unchanged. |
| R22 | 6 | Demo imports stay in demo storage. |
| R23 | 7 | `demo` prints the bundled Markdown action-card sample. |
| R24 | 13 | `scan --config` reads a JSON watch file and writes Markdown cards under `.integration-changelog-watch/actions/`. |
| R25 | 8 | It stores hashes and acknowledgement state in `.integration-changelog-watch/state.json`. |
| R26 | 12 | Each card prints its hash-derived action ID; pass that ID to `ack`. |
| R27 | 10 | Acknowledgement updates both the state file and the Markdown card. |
| R28 | 12 | The shipped example uses `examples/sample-feed.xml`, so it works without a network request. |
| R29 | 8 | The hosted workspace holds up to three watches. |
| R30 | 8 | Use the local CLI for a four-watch mapping. |
| R31 | 12 | Turn on a schedule for any watch when you want automatic scans. |
| R32 | 10 | Scheduled watches show the last run, next run, and errors. |
| R33 | 9 | Add an optional public webhook destination for run summaries. |
| R34 | 12 | Build the repository image with `docker build --build-arg BUILD_SHA=dev -t integration-changelog-watch .`. |
| R35 | 8 | The build stage uses the official `rust:1-alpine` image. |
| R36 | 8 | It starts with only `PORT` required (default `8080`). |
| R37 | 20 | The shipped Container App configuration uses one replica and mounts durable Azure Files at `/data`, where SQLite persists at `/data/changelog-watch.db`. |
| R38 | 17 | A production topology guard closes workspace APIs and restores that configuration if a generic deploy removes it. |
| R39 | 7 | SQLite uses the Azure Files-compatible `unix-dotfile` VFS. |
| R40 | 9 | Keep one replica so workspace state has one owner. |
| R41 | 10 | See `/privacy` and `/terms` for data handling and source safeguards. |
| R42 | 2 | MIT licensed. |
| R43 | 4 | Built by Param Factory. |

The README headings — **Run locally**, **Workspace API**, **Watch files**, **CLI demo**, **Hosted workspace limits**, **Scheduled scans**, and **Deploy** — name their sections plainly. Its buttons are references to the same result-naming UI controls audited above.

## Structure, accessibility, and links

- `/`, `/?demo=1`, `/demo`, `/privacy`, and `/terms` return 200. An unknown route returns the designed 404 with a return-home action.
- Each app route has one h1 and one main, `lang="en"`, a route-specific title, meta description, canonical, Open Graph and Twitter title/description, favicon, and apple touch icon. `/demo` uses the required “Demo — Product” title; legal routes use “Privacy/Terms — Product”.
- A cold `/#how` request scrolls to the target (`scrollY: 1269` at the reviewed viewport). Back navigation restores the route heading focus.
- Internal routes, `robots.txt`, `sitemap.xml`, favicon, apple-touch icon, social card, and 404 stylesheet responded successfully. The two demo vendor-notice links (`docs.stripe.com/changelog` and `auth0.com/changelog`) each returned 200.
- Full Axe WCAG 2 A/AA scans on `/`, `/demo`, `/privacy`, `/terms`, and a 404 returned zero violations. Successful routes produced no console errors. Mobile and desktop screenshots show the warm-paper, indigo-thread, clipped-card system defined in `.factory/design.md`, not a generic SaaS template.
- The only runtime requests during the reviewed demo flow were same-origin. CSP, request logs, and source inspection show no third-party analytics, fonts, payment provider, provider key, or decorative AI feature. An AI step is not missing from this deterministic feed-monitoring workflow; import/export and opt-in scheduling are present.

## History recheck

Every earlier review, polish report, and handoff was read. Each earlier finding was verified against the live service and current source rather than accepted from its marked status.

| Earlier finding | Current verification |
| --- | --- |
| F-1-1 | Demo opens on the populated sample action board; all required Stripe fields are in the initial 390 px viewport. |
| F-1-2 | The watched-feed panel is a labelled section; full Axe has zero violations. |
| F-1-3 | Controlled hosted feed matching is registered as `hosted-scan-result` and its backend test passed. |
| F-1-4 | The three-watch capacity is explicit and covered by `hosted-watch-limit`; unsupported pricing wording is absent. |
| F-1-5 | Public copy uses **keywords** and `keyword-edit` passed. |
| F-1-6 | The four-watch CLI workflow is covered by `cli-more-feeds`. |
| F-1-7 | README has a route table and `api-contract` passed. |
| F-1-8 | The durable `rust:1-alpine` statement has its own container-build claim. |
| F-1-9 | Unsupported hosted-plan/team claims remain removed. |
| F-1-10 | The duplicate collaboration exclusion remains removed. |
| F-1-11 | README verification instructions remain split and within the cap. |
| F-1-12 | The endpoint inventory remains a readable table. |
| F-1-13 | CLI input, output, and state statements remain split. |
| F-1-14 | **Keywords** and **assigned action cards** are used consistently; no “Hosted workspace scope” leak remains. |
| F-1-15 | The redundant hero eyebrow and provenance caption remain absent. |
| F-1-16 | The banner action is **Start a private workspace** with its discard explanation. |
| F-1-17 | Demo, legal, and landing metadata change together by route. |
| F-1-18 | The 404 has shared navigation, metadata, footer, one h1, and one main. |
| F-1-19 | Previewed watch-file import/export remains present and isolated in demo storage. |
| F-2-1 | Cold `/#how` positioning and history behavior pass. |
| F-2-2 | The audience sentence uses **authentication**, not “auth”. |
| F-3-2 | Azure Files `unix-dotfile` wording is registered as `azure-files-dotfile-locking`. |
| F-3-3 | Owner-consented schedules, deduplication, run status, stop controls, and optional webhook summaries are implemented and claimed. |
| F-3-4 | The first screen now presents online, payment, and privacy facts. |
| F-4-1 | The CTA enters demo without workspace/API activity; late real-workspace reads are aborted/guarded in source and the claim passed. |

## What would make this perfect

The reviewed product already meets the stated standard: there is no open product, copy, demo, claim, routing, privacy, accessibility, or visual-identity follow-up from this review. Keep the clean-clone claim runner and live route checks in the release gate so these verified properties remain true after future changes.
