# Polish 4 — cumulative adversarial review closure

Application repair revision: `f2285321d9f76bfcf286a2bd6c441258df593f9f`.

Live product: <https://integration-changelog-watch.sociobot.in>. Deployed image: `sociobotregistry.azurecr.io/sf-integration-changelog-watch:f2285321d9f7`, digest `sha256:885b03933636c1c724f22bda76ce75ec32894ad4700b1645bba535cb02f1418e`.

Evidence paths below are relative to `.factory/qa-artifacts/polish-4-live/`.

## Finding closure

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Kept `/?demo=1` and `/demo` on the populated board, ahead of landing copy. | `@claim:sample-action-cards`; `cold-demo-mobile.png`; live `/?demo=1` shows the title, keyword, owner, version, and check inside 390×844. |
| F-1-2 | Kept watched feeds as a labelled section inside `main`. | `demo has no accessibility violations`; `demo/screenshot-mobile.png`; live Axe reports zero violations on `/demo`. |
| F-1-3 | Kept controlled-feed matching and all assigned-card fields. | `claim_hosted_scan_creates_action_card_from_controlled_fixture`; `cold-demo-mobile.png`; live `/?demo=1`. |
| F-1-4 | Kept unproved price wording removed and the three-watch capacity explicit. | `@claim:hosted-watch-limit`; `home/screenshot-desktop.png`; live `/` states the tested limit. |
| F-1-5 | Kept `keywords` as the matching term and removed the remaining Terms-page “matching rules” leak. | `@claim:keyword-edit` and `keeps concrete, consistent product terms in public copy`; `terms-mobile.png`; live `/terms`. |
| F-1-6 | Kept the tested four-watch local CLI alternative. | `@claim:cli-more-feeds`; `home/screenshot-desktop.png`; live `/` names a four-watch mapping. |
| F-1-7 | Kept the method/path API table and schedule routes covered. | `@claim:api-contract`; `home/screenshot-desktop.png`; live `/health` returns the repair SHA. |
| F-1-8 | Kept durable `rust:1-alpine` wording and the locked release build. | `@claim:container-build-stage`; `home/screenshot-desktop.png`; ACR build `ch1at` passed. |
| F-1-9 | Kept unsupported plan/team exclusions out of public copy. | `keeps concrete, consistent product terms in public copy`; `cold-home-mobile.png`; live `/` cold read. |
| F-1-10 | Kept the duplicate collaboration exclusion removed. | `npm test` copy regression; `cold-home-desktop.png`; live `/` and README audit. |
| F-1-11 | Kept README verification commands in short sentences. | `.factory/copy-audit.md`; `cold-home-desktop.png`; clean-clone `npm test`. |
| F-1-12 | Kept the API inventory in a readable table. | `@claim:api-contract`; `home/screenshot-desktop.png`; live API contract and `/health`. |
| F-1-13 | Kept CLI input, card output, and state as separate sentences. | `@claim:cli-repository-workflow`; `home/screenshot-desktop.png`; final clean-clone claim run. |
| F-1-14 / F-3-1 | Standardized public and CLI output on `keywords` and `assigned action cards`; updated Cargo metadata too. | `keeps concrete, consistent product terms in public copy`; `cold-home-mobile.png`, `terms-mobile.png`; live `/` and `/terms`. |
| F-1-15 | Kept the redundant eyebrow and provenance caption absent. | `cold-home-mobile.png`; live `/` first screen. |
| F-1-16 | Kept **Start a private workspace** with **Discards this demo.** | `@claim:demo-isolation-transitions`; `cold-demo-mobile.png`; live `/?demo=1`. |
| F-1-17 | Kept route-specific title, description, canonical, Open Graph, and Twitter metadata; added `/?demo=1` coverage. | `every app route supplies route-specific titles, descriptions, previews, and canonicals`; `home/verify.json`, `demo/verify.json`; live `/`, `/?demo=1`, `/demo`, `/privacy`, `/terms`. |
| F-1-18 | Kept the styled 404 with shared navigation, metadata, footer, one h1, and one main. | `missing routes return the product-styled 404 screen`; `404-mobile.png`; live `/missing-polish-4` returns 404. |
| F-1-19 | Kept previewed JSON watch-file import/export and atomic rejection. | `@claim:watch-file-portability`, `@claim:watch-file-rejection-preserves-watches`; `demo/screenshot-desktop.png`; live `/?demo=1`. |
| F-2-1 | Kept cold-fragment restoration and Back-navigation focus. | `a cold How it works deep link reaches its target and survives history navigation`; `home/screenshot-mobile.png`; live `/#how`. |
| F-2-2 | Kept the full word `authentication` in the audience sentence. | `cold-live-audit.json`; `cold-home-mobile.png`; live `/`. |
| F-3-2 | Kept the exact Azure Files `unix-dotfile` claim and dedicated test. | `claim_azure_files_dotfile_locking`; `home/screenshot-desktop.png`; live `/health` on the one-replica deployment. |
| F-3-3 | Kept owner-consented schedules, deduplication, visible last/next/error state, stop controls, and optional public webhooks. | Four `scheduled-*` claim tests; `home/screenshot-desktop.png`; live `/` scheduled-scans section. |
| F-3-4 | Kept online, price/account, and workspace-separation facts on the first screen. | `@claim:online-feed-scans`, `@claim:no-account-or-payment`, `@claim:workspace-boundary`; `cold-home-mobile.png`; live `/`. |
| F-4-1 | Removed eager workspace creation. Route changes now abort real reads, invalidate stale responses, and reject late token writes. The landing CTA now enters `/?demo=1`. | `@claim:demo-isolation-transitions` delays `/api/workspaces` and asserts zero API calls and zero `icw:*` keys; `cold-demo-mobile.png`, `cold-live-audit.json`; live CTA check reports `workspaceCreates: 0`, `apiRequests: []`, `realKeys: []`. |

## Verification

- Fresh clone `/tmp/icw-polish4-clean-IhdgJA` at `f228532…`: all 28 literal claim commands passed without leaking port 8080.
- The same clean clone passed 10 Vitest tests, TypeScript checks, the production build, Rustfmt, 28 Rust tests, 69 browser tests with three documented project skips, and 20 accessibility/routing tests.
- Live: 20/20 accessibility/metadata/routing tests and 8/8 demo/privacy/offline tests passed. Full Axe WCAG 2 A/AA found zero violations.
- The isolated live rate-limit probe passed a 100-request burst, observed 429 responses with `Retry-After: 1`, and kept `/health` available.
- `verify-url.sh` found no console errors on `/` or `/?demo=1`. Route checks returned 200 for `/`, `/demo`, `/?demo=1`, `/privacy`, and `/terms`; the styled unknown route returned 404.
- Cold live evidence reports no horizontal overflow, no initial storage, no demo API request, no `icw:*` key, correct h1 focus, and all sample fields inside 390×844.
- Lighthouse mobile: performance 100, accessibility 100, best practices 100, SEO 100; LCP 1.20 s, CLS 0, TBT 0, transfer 93,893 bytes. See `lighthouse-mobile.json`.

No finding from reviews 1–4 remains open.
