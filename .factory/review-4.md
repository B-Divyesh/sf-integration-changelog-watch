# Adversarial first-read review 4

- Product: Integration Changelog Watch
- Live URL: <https://integration-changelog-watch.sociobot.in>
- Review date: 2026-08-29 UTC
- Repository review base: `77935bc14edfd00e8cf8c19e6ba6d671198e9702`
- Live `/health` build: `2c6f43ab489e9c34cb25513407ef24ddbebaaf88`
- Verdict: **FAIL**

There is one blocking finding. The landing-page demo CTA can write a real
workspace token after the visitor has entered the demo. A pass requires zero
findings. `.factory/brief.json` is absent, so scope was checked against the
product contract, live product, design thesis, README, claims registry, and
the complete review/polish/handoff history.

## Cold first screen

Fresh Chromium contexts at 390 × 844 and 1440 × 900 were used before
scrolling.

| Question | Answer | Exact text that made it clear |
| --- | --- | --- |
| What does this do? | It turns vendor changes into assigned action cards. | “Turn vendor changes into assigned action cards” |
| For whom? | Engineers maintaining payment, authentication, analytics, or messaging integrations. | “For engineers who maintain payment, authentication, analytics, or messaging integrations.” |
| What should I click first? | **Try it with sample data**. | “Try it with sample data” and “See matched notices, owners, versions, and checks.” |

All three answers are available without scrolling at both widths. Mobile has no
horizontal overflow. This is not a first-read failure.

## Findings

### Blocking

#### F-4-1 — The one-click demo CTA can create persistent real-workspace state

- Location: landing **“Try it with sample data”** CTA; `frontend/src/main.ts`,
  `ensureWorkspace()` and the initial `hydrateReal()` call.
- Reproduction against the live product: in a fresh browser, delay
  `POST /api/workspaces` for 1.2 seconds, load `/`, click **Try it with sample
  data** as soon as it is visible, then release the request. The browser is at
  `/demo` with the **“Demo — sample data, nothing is saved”** banner visible,
  yet `localStorage` contains a real
  `icw:workspace-token` (`1de805…2cbb35e` in this run). The request also
  provisions that real workspace on the server.
- Why this fails: the sample-data path is advertised as an isolated sandbox.
  A visitor can enter it in one click and still receive real persistent state
  during demo mode. This contradicts the demo contract that nothing in demo
  mode reads or writes real storage, and makes the banner materially
  misleading under an ordinary network race.
- Concrete fix: do not create or hydrate a private workspace during landing
  render. Create it only after an explicit private-workspace action. Also
  cancel or ignore an in-flight workspace creation when routing to demo, and
  ensure it cannot write `icw:workspace-token` after `demo` becomes true. Add
  a Playwright regression test which delays `/api/workspaces`, clicks the demo
  CTA, releases the delayed request, then asserts no `/api/` request and no
  `icw:*` key exist while the demo banner is present. Extend or replace
  `@claim:demo-isolation-transitions` so this exact CTA race is covered.

## Copy audit

Counts are whitespace-delimited; commands, URLs, paths, and hyphenated words
each count as one. The landing inventory is the fresh `/` page. Dynamic sample
records are audited under the demo check rather than treated as landing copy.
No prose sentence exceeds 22 words. No banned marketing term, empty slogan,
or inconsistent core term was found. All buttons name their result.

### Landing sentences and headings

| Location | Words | Exact copy | Result |
| --- | ---: | --- | --- |
| Hero heading | 7 | Turn vendor changes into assigned action cards | Plain job headline |
| Hero audience | 10 | For engineers who maintain payment, authentication, analytics, or messaging integrations. | Plain audience |
| CTA result | 7 | See matched notices, owners, versions, and checks. | `sample-action-cards` |
| Hero fact | 5 | Scanning public feeds requires internet. | `online-feed-scans` |
| Hero fact | 6 | No account or payment is required. | `no-account-or-payment` |
| Hero fact | 7 | Your workspace is separated from other visitors. | `workspace-boundary` |
| Queue status | 4 | No actions need acknowledgement | State label |
| Empty-state heading | 4 | No action cards yet | State heading |
| Empty-state text | 10 | Matched release notes appear here after you scan a feed. | `hosted-scan-result` |
| Watch state | 4 | Nothing is watched yet. | State text |
| Section label | 3 | How it works | Names its section |
| How heading | 7 | Give each vendor change a next step | Useful instruction heading |
| Step | 4 | Watch a public feed. | Instruction |
| Step detail | 11 | Paste a changelog or RSS address you are allowed to read. | Instruction |
| Step | 3 | Choose your keywords. | Instruction |
| Step detail | 9 | Use keywords like “webhook”, “deprecation”, or an API version. | Concrete example |
| Step | 4 | Run the right check. | Instruction |
| Step detail | 11 | Each matching notice includes an owner, dependency version, and check command. | `hosted-scan-result` |
| Section heading | 3 | Hosted workspace limits | Names its section |
| Limit | 8 | The hosted workspace holds up to three watches. | `hosted-watch-limit` |
| CLI alternative | 8 | Use the local CLI for a four-watch mapping. | `cli-more-feeds` |
| Section heading | 2 | Scheduled scans | Names its section |
| Schedule instruction | 12 | Turn on a schedule for any watch when you want automatic scans. | `scheduled-scan-consent` |
| Schedule result | 10 | Scheduled watches show the last run, next run, and errors. | `scheduled-run-status` |
| Webhook option | 9 | Add an optional public webhook destination for run summaries. | `scheduled-notification-destination` |
| Section heading | 2 | Source safeguards | Names its section |
| Safeguard | 7 | Private, loopback, and link-local addresses are blocked. | `workspace-boundary` |
| Image alt | 9 | Paper release-note cards move into an assigned action card. | Purposeful alt text |
| Footer | 6 | Vendor notices become assigned action cards. | `hosted-scan-result` |

Controls and navigational labels: **Try it with sample data**, **Scan watched
feeds**, **Add a watch**, **Add your first watch**, **Export watch file**,
**Import watch file**, and **Export action cards as CSV** are result-naming
verbs. **Demo**, **How it works**, **Privacy**, and **Terms** are links. The
only headings that are not sentences name their sections or current state.

### README sentences and headings

| Location | Words | Exact copy | Result |
| --- | ---: | --- | --- |
| Title | 3 | Integration Changelog Watch | Product name |
| Introduction | 7 | Turn vendor changes into assigned action cards. | `hosted-scan-result` |
| Audience | 12 | It is for engineers who maintain payment, authentication, analytics, or messaging integrations. | Plain audience |
| Instruction | 19 | Add a public changelog or RSS feed, keywords, an owner, the affected dependency version, and a local check command. | Instruction |
| Instruction | 12 | Scan when you are ready to review notices and create action cards. | `hosted-scan-result` |
| Heading | 2 | Run locally | Names its section |
| Command comment | 2 | creates dist/ | Command outcome |
| Deployment fact | 7 | The container uses durable `/data` by default. | `database-persistence` |
| Instruction | 10 | On a host without that mount, use `DATABASE_URL='sqlite:changelog-watch.db?mode=rwc' cargo run`. | Instruction |
| Instruction | 7 | For frontend development, use `npm run dev`. | Instruction |
| Instruction | 10 | Run `npm test` and `npm run typecheck` for code checks. | Instruction |
| Instruction | 11 | After `npm run build`, run `npm run test:browser` for browser coverage. | Instruction |
| Instruction | 7 | The exact claim commands live in `.factory/claims.json`. | Instruction |
| Demo instruction | 6 | Open `http://localhost:8080/demo` for a one-click sandbox. | `demo-local` |
| Demo fact | 6 | Demo data uses `demo:integration-changelog-watch` browser storage. | `demo-local` |
| Demo transition | 6 | Start a private workspace discards it. | `demo-isolation-transitions`; see F-4-1 |
| Heading | 2 | Workspace API | Names its section |
| API summary | 11 | The container exposes health, workspace, watch, schedule, action, and scan endpoints. | `api-contract` |
| Instruction | 4 | Create a workspace first. | `api-contract` |
| Instruction | 9 | Send its browser-held bearer token with every workspace request. | `api-contract` |
| Source rule | 14 | Source URLs must be public `http` or `https` addresses; private network addresses are rejected. | `workspace-boundary` |
| Heading | 2 | Watch files | Names its section |
| Instruction | 13 | Use **Export watch file** to download your watches in the CLI JSON schema. | `watch-file-portability` |
| Instruction | 10 | Use **Import watch file** to preview one to three watches. | `watch-file-portability` |
| Import result | 9 | A rejected private-workspace import leaves the current watches unchanged. | `watch-file-rejection-preserves-watches` |
| Demo import | 6 | Demo imports stay in demo storage. | `watch-file-portability` |
| Heading | 2 | CLI demo | Names its section |
| CLI output | 7 | `demo` prints the bundled Markdown action-card sample. | `cli-demo-local` |
| CLI output | 13 | `scan --config` reads a JSON watch file and writes Markdown cards under `.integration-changelog-watch/actions/`. | `cli-repository-workflow` |
| CLI state | 8 | It stores hashes and acknowledgement state in `.integration-changelog-watch/state.json`. | `cli-repository-workflow` |
| CLI instruction | 12 | Each card prints its hash-derived action ID; pass that ID to `ack`. | `cli-repository-workflow` |
| CLI result | 10 | Acknowledgement updates both the state file and the Markdown card. | `cli-repository-workflow` |
| CLI locality | 12 | The shipped example uses `examples/sample-feed.xml`, so it works without a network request. | `cli-shipped-mapping-local` |
| Heading | 3 | Hosted workspace limits | Names its section |
| Limit | 8 | The hosted workspace holds up to three watches. | `hosted-watch-limit` |
| CLI alternative | 8 | Use the local CLI for a four-watch mapping. | `cli-more-feeds` |
| Heading | 2 | Scheduled scans | Names its section |
| Schedule instruction | 12 | Turn on a schedule for any watch when you want automatic scans. | `scheduled-scan-consent` |
| Schedule result | 10 | Scheduled watches show the last run, next run, and errors. | `scheduled-run-status` |
| Webhook option | 9 | Add an optional public webhook destination for run summaries. | `scheduled-notification-destination` |
| Heading | 1 | Deploy | Names its section |
| Instruction | 12 | Build the repository image with `docker build --build-arg BUILD_SHA=dev -t integration-changelog-watch .`. | Instruction |
| Build fact | 8 | The build stage uses the official `rust:1-alpine` image. | `container-build-stage` |
| Startup fact | 8 | It starts with only `PORT` required (default `8080`). | `port-only-startup` |
| Topology fact | 20 | The shipped Container App configuration uses one replica and mounts durable Azure Files at `/data`, where SQLite persists at `/data/changelog-watch.db`. | `single-replica-durable-data`, `database-persistence` |
| Guard fact | 17 | A production topology guard closes workspace APIs and restores that configuration if a generic deploy removes it. | `single-replica-durable-data` |
| Locking fact | 7 | SQLite uses the Azure Files-compatible `unix-dotfile` VFS. | `azure-files-dotfile-locking` |
| Instruction | 9 | Keep one replica so workspace state has one owner. | `single-replica-durable-data` |
| Link instruction | 10 | See `/privacy` and `/terms` for data handling and source rules. | Link instruction |
| License | 2 | MIT licensed. | `LICENSE` |
| Attribution | 4 | Built by Param Factory. | Attribution |

The API table contains labels and concise result fragments rather than further
sentences. The command examples are commands, not prose. There are no
copy findings beyond F-4-1's misleading demo wording in this execution path.

## Demo, claims, and privacy verification

- The direct `/demo` entry starts with a persistent banner, **Reset demo**,
  **Start a private workspace**, and realistic Stripe/Auth0 sample action
  cards. At 390 px, the first viewport contains the Stripe title, matched
  keyword, owner, dependency version, and check.
- Direct fresh `/demo` requests are only the same-origin document, JavaScript,
  and CSS. Acknowledge and Reset operate on the demo namespace and no API,
  analytics, advertising, third-party font, or cross-origin request occurs.
- Reset restores the shipped sample. Direct demo storage is initially empty and
  becomes `demo:integration-changelog-watch` only after a demo mutation.
- F-4-1 means the *landing CTA path* fails the same isolation check under an
  in-flight request. It is therefore not acceptable to treat the direct URL as
  sufficient proof of the advertised one-click path.
- `.factory/claims.json` contains 28 entries. From a fresh clone at the review
  base, the exact claim runner completed; its constituent test suites also
  passed independently: 69 Playwright tests (3 documented skips), 28 Rust
  tests, and the relevant Vitest container contract. No registered claim test
  failed. The review's claim completeness audit found no additional
  claim-like landing or README sentence without a registry entry.

## History recheck

Read in full: `.factory/review-1.md`, `.factory/review-2.md`,
`.factory/review-3.md`, `.factory/polish-1.md`, `.factory/polish-2.md`,
`.factory/polish-3.md`, and the prior `.factory/handoff.md`.

| Earlier finding | Live and code confirmation |
| --- | --- |
| F-1-1 | Fixed: `/demo` starts on populated sample action cards, with all required first-card fields in the 390 px viewport. |
| F-1-2 | Fixed: watched feeds are a labelled `<section>`; the live Axe suite reports zero violations. |
| F-1-3 | Fixed: fixture-backed hosted scan match claim and test exist. |
| F-1-4 | Fixed: unsupported price wording is gone; the three-watch limit is tested. |
| F-1-5 | Fixed: live wording uses `keywords`; create/edit/reload coverage exists. |
| F-1-6 | Fixed: the four-watch CLI mapping has a registered test. |
| F-1-7 | Fixed: documented API methods are in a table and `api-contract` covers them. |
| F-1-8 | Fixed: the durable `rust:1-alpine` wording and contract test remain. |
| F-1-9, F-1-10 | Fixed: unsupported hosted-plan and collaboration exclusions remain absent. |
| F-1-11, F-1-12, F-1-13 | Fixed: README instructions are split and the API remains a table. |
| F-1-14 / F-3-1 | Fixed: `Hosted workspace limits` replaces the abstract prior heading. |
| F-1-15 | Fixed: redundant hero eyebrow and illustration-provenance caption remain absent. |
| F-1-16 | Fixed: the control says `Start a private workspace` and says it discards the demo. |
| F-1-17 | Fixed: live app routes update title, description, canonical, OG, and Twitter metadata. |
| F-1-18 | Fixed: the styled 404 has the common navigation, metadata, footer, one h1, main, and return-home route. |
| F-1-19 | Fixed: previewed watch-file import/export and atomic rejection remain present and tested. |
| F-2-1 | Fixed: cold `/#how` reaches its target and history restores it. |
| F-2-2 | Fixed: the first-screen and README audience use `authentication`, not `auth`. |
| F-3-2 | Fixed: the exact `azure-files-dotfile-locking` claim and dedicated Rust test exist. |
| F-3-3 | Fixed: opt-in scheduled scans, visible state, deduplication, and optional public webhook destination exist and are claimed. |
| F-3-4 | Fixed: first-screen facts now state online, price/account, and workspace separation facts. |

F-4-1 is a newly observed race in the demo boundary. It does not reopen an
earlier finding ID because the earlier reviews did not test the landing CTA
while workspace creation was in flight.

## Structure, accessibility, and visual identity

- `/`, `/demo`, `/privacy`, and `/terms` return 200. A nonexistent route
  returns the designed 404 with a return-home route; the browser's expected
  failed-document 404 console message is its only console error.
- Each checked route has `lang="en"`, one main, one h1, a route-specific title,
  description, canonical, OG/Twitter fields, favicon, and apple-touch icon.
  Titles use the product/route pattern and are under 60 characters.
- `robots.txt`, `sitemap.xml`, hero, social card, favicon, apple-touch icon,
  all internal links, and the two sample vendor links returned 200. Header,
  footer, Privacy, Terms, skip link, history focus, and route announcement are
  consistent.
- `PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run
  test:a11y` passed 20/20. Keyboard, focus, reduced motion, mobile width, and
  no-success-route-console-error checks pass.
- The paper-cut operations-board identity is visibly distinct from a generic
  SaaS template and conforms to `.factory/design.md`: warm paper, indigo
  thread, clipped cards, editorial serif headings, and original supporting
  art. No runtime AI feature is implied or needed for this deterministic
  watch/scan workflow; import/export and opt-in scheduling are already present.

## Local verification

From fresh clone `/tmp/icw-review4-clean`:

```sh
npm ci
npm run test:claims
npm test
npm run typecheck
npm run build
cargo test --locked
cargo fmt --all -- --check
npm run test:browser
PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run test:a11y
```

All completed successfully: 10 Vitest tests, 28 Rust tests, 69 browser tests
with 3 documented skips, and 20 live accessibility/metadata/routing tests.

## What would make this perfect

Make the demo CTA race-free so a sample visitor never creates, reads, or
persists a real workspace. Add the delayed-request regression test to the
registered demo-isolation claim, then rerun the full cold phone/desktop,
claims, privacy-request, history, routing, and accessibility checklist. A
perfect result has zero findings.
