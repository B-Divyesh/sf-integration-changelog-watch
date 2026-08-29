# Polish 2 — cumulative adversarial review closure

Deployed repair revision: `55b093c06cc0c83180cf3cb6a51d4301d4cf24b9`.
Live product: <https://integration-changelog-watch.sociobot.in>.
Evidence paths below are relative to `.factory/qa-artifacts/polish-2/`.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Kept `/demo` and `?demo=1` on the populated action board, ahead of all marketing content. | `@claim:sample-action-cards`; `live-demo-mobile.png`; cold `/demo` and `/?demo=1` show the owner, version, and check within 844 px. |
| F-1-2 | Kept the watched-feed panel as a labelled section inside main. | `demo has no accessibility violations`; `live-demo-mobile.png`; live `/demo` full Axe WCAG 2 A/AA has zero violations. |
| F-1-3 | Kept the fixture-backed hosted scan-result claim and backend test. | `claim_hosted_scan_creates_action_card_from_controlled_fixture`; `live-demo-desktop.png`; live `/demo` renders the resulting action-card fields. |
| F-1-4 | Kept the tested three-watch limit and removed unproved price language. | `@claim:hosted-watch-limit`; `live-verify/screenshot-desktop.png`; live `/` states the three-watch limit. |
| F-1-5 | Kept “keywords” copy and create/edit/reload coverage. | `@claim:keyword-edit`; `live-verify/screenshot-desktop.png`; live `/` uses “keywords” consistently. |
| F-1-6 | Kept the four-watch local CLI claim and temporary-repository test. | `@claim:cli-more-feeds`; `live-verify/screenshot-desktop.png`; live `/` names the four-watch CLI mapping. |
| F-1-7 | Kept the method/path API table and full route contract test. | `@claim:api-contract`; `live-verify/screenshot-desktop.png`; live `/health` returns build `55b093c06cc0…`. |
| F-1-8 | Kept the durable `rust:1-alpine` wording and locked-build test. | `@claim:container-build-stage`; `live-verify/screenshot-desktop.png`; `deploy.log` records the successful official-image build. |
| F-1-9 | Kept unproved plan/team exclusions out of landing copy. | `@claim:hosted-watch-limit`; `live-cold-desktop.png`; live `/` presents only the tested hosted limit. |
| F-1-10 | Kept the duplicate collaboration exclusion removed from README. | Clean copy audit; `live-cold-desktop.png`; live `/` contains no collaboration claim. |
| F-1-11 | Kept README verification steps as short separate sentences. | `npm run typecheck`; `live-cold-desktop.png`; repository README copy audit has no sentence over 22 words. |
| F-1-12 | Kept the endpoint inventory in a method/path table. | `@claim:api-contract`; `live-verify/screenshot-desktop.png`; live `/health` and all documented API methods pass. |
| F-1-13 | Kept CLI input, output, and state details in separate sentences. | `@claim:cli-repository-workflow`; `live-verify/screenshot-desktop.png`; clean-clone claim command passes. |
| F-1-14 | Kept “assigned action cards” and “keywords” as the single product terms. | `@claim:keyword-edit`; `live-cold-mobile.png`; live `/` and `/demo` use the terminology table. |
| F-1-15 | Kept the duplicate eyebrow and provenance caption out of the hero. | `mobile navigation, legal return, and footer links meet the 44px touch target minimum`; `live-cold-mobile.png`; live `/` has no decorative copy. |
| F-1-16 | Kept “Start a private workspace” with “Discards this demo.” | `@claim:demo-isolation-transitions`; `live-demo-mobile.png`; live `/?demo=1` resets and exits without retaining demo state. |
| F-1-17 | Kept route-specific title, description, canonical, OG, and Twitter metadata. | `every app route supplies route-specific titles, descriptions, previews, and canonicals`; `live-verify/screenshot-desktop.png`; live `/`, `/demo`, `/privacy`, and `/terms` metadata all match. |
| F-1-18 | Kept the designed 404 with shared navigation and complete preview/touch metadata. | `missing routes return the product-styled 404 screen`; `live-verify/screenshot-desktop.png`; live `/missing-polish-2` returns 404 with one h1 and main. |
| F-1-19 | Kept previewed JSON watch-file import/export and atomic rejection behavior. | `@claim:watch-file-portability` and `@claim:watch-file-rejection-preserves-watches`; `live-demo-desktop.png`; live `/demo` import/export controls pass. |
| F-2-1 | Added post-render fragment restoration for cold loads and history navigation. | `a cold How it works deep link reaches its target and survives history navigation`; `live-how-mobile.png`; live `/#how` reports `scrollY: 1984` and target `y: -0.27`. |
| F-2-2 | Replaced “auth” with “authentication” on the first screen and in README. | `live-first-screen.json`; `live-cold-mobile.png`; live `/` shows the full word within the first phone viewport. |

## Cumulative verification

- Clean clone of `55b093c06cc0…`: all 21 literal `.factory/claims.json` commands passed. See `clean-claims.log`.
- Local: 7 Vitest tests, TypeScript typecheck, production build, 23 Rust tests, and 65 Playwright tests passed. Three intentional live/project probes skipped in the general browser run.
- Live: 20 accessibility/metadata/routing tests, 16 demo/privacy/offline tests, and the isolated rate-limit probe passed.
- `verify-url.sh` reports title, `lang`, one h1, main, image alt text, labelled buttons, and zero console errors on `/demo`.
- Lighthouse mobile: performance 100, accessibility 100, best practices 100, SEO 100; LCP 1.21 s, CLS 0, TBT 0, total transfer 90,553 bytes.
- Deployed image: `sociobotregistry.azurecr.io/sf-integration-changelog-watch:55b093c06cc0`, digest `sha256:efa092a7857e17f46967f9be9d0e1d28c7f5b46809667aee0c770c44e307ca61`.

No finding from review 1 or review 2 remains open.
