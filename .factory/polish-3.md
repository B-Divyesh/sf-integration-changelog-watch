# Polish 3 — cumulative adversarial review closure

Source repair deployed: `b0a9016c89ab64be7553f6dbbe92700e25348640`.

Live product: <https://integration-changelog-watch.sociobot.in>. The deployed ACR image is `sociobotregistry.azurecr.io/sf-integration-changelog-watch:b0a9016c89ab`, digest `sha256:a1d9201026a11616831e52fd04e8dc9c972b3ec289fa44b1751afb6e4d8ed444`.

## Finding closure

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Kept `/demo` and `?demo=1` on the populated board, ahead of landing copy. | `@claim:sample-action-cards`; [live mobile demo screenshot](qa-artifacts/polish-3-live/demo-mobile.png); live cold check shows title, owner, dependency, and check. |
| F-1-2 | Kept watched feeds as a labelled section, not a nested complementary landmark. | Live Axe WCAG 2 A/AA: 0 violations on `/demo`; `npm run test:a11y`. |
| F-1-3 | Kept the controlled-feed match recorder and its action-card fields. | `claim_hosted_scan_creates_action_card_from_controlled_fixture`; live `/demo`. |
| F-1-4 | Kept the tested three-watch limit and removed unproved price language. | `@claim:hosted-watch-limit`; live `/` says the tested capacity. |
| F-1-5 | Kept `keywords` as the sole matching term. | `@claim:keyword-edit`; `.factory/copy-audit.md`. |
| F-1-6 | Kept the tested four-watch local CLI path. | `@claim:cli-more-feeds`; live `/` and README. |
| F-1-7 | Updated the API table and contract test for schedule routes too. | `@claim:api-contract`; live API probe creates, starts, and stops a schedule. |
| F-1-8 | Kept durable Rust-image wording and the Dockerfile contract test. | `@claim:container-build-stage`; ACR build `ch17n` succeeded. |
| F-1-9 | Kept unproved plan and team exclusions out of product copy. | `.factory/copy-audit.md`; live `/` cold check. |
| F-1-10 | Kept the duplicate collaboration exclusion out of README. | README audit; clean-clone claim suite. |
| F-1-11 | Kept verification instructions as short separate sentences. | README and `.factory/copy-audit.md`. |
| F-1-12 | Kept the endpoint inventory as a method/path table. | `@claim:api-contract`; README table. |
| F-1-13 | Kept CLI inputs, outputs, and state in separate sentences. | `@claim:cli-repository-workflow`; README. |
| F-1-14 / F-3-1 | Renamed both headings to **Hosted workspace limits** and added a regression assertion rejecting the old wording. | `sample workspace prevents the abstract Hosted workspace scope heading from returning`; live `/` screenshot and copy audit. |
| F-1-15 | Kept decorative duplicate labels out of the first screen. | [live home mobile screenshot](qa-artifacts/polish-3-live/home-mobile.png). |
| F-1-16 | Kept **Start a private workspace** plus the discard note. | `@claim:demo-isolation-transitions`; live `/demo`. |
| F-1-17 | Kept route-specific title, description, canonical, OG, and Twitter metadata. | Live `npm run test:a11y`: 20 passed. |
| F-1-18 | Kept the designed 404 with shared navigation and complete metadata. | Live Axe/route check on `/missing-polish-3`: 404, 0 violations. |
| F-1-19 | Kept previewed watch-file export/import and isolated demo imports. | `@claim:watch-file-portability`; `@claim:watch-file-rejection-preserves-watches`. |
| F-2-1 | Kept post-render fragment restoration for cold `/#how` links and Back navigation. | Live `a cold How it works deep link reaches its target and survives history navigation`. |
| F-2-2 | Kept **authentication** in the audience sentence. | [live home mobile screenshot](qa-artifacts/polish-3-live/home-mobile.png); copy audit. |
| F-3-2 | Split the lock statement and registered an exact Azure Files VFS claim. | `@claim:azure-files-dotfile-locking` → `claim_azure_files_dotfile_locking`. |
| F-3-3 | Added an owner-consented per-watch scheduler, persisted last/next/error state, notice-key deduplication, stop controls, and an optional public webhook run summary. | `@claim:scheduled-scan-consent`, `@claim:scheduled-scan-deduplication`, `@claim:scheduled-run-status`, `@claim:scheduled-notification-destination`; live API check saved then removed a 60-minute schedule. |
| F-3-4 | Replaced the hero facts with online, price, and privacy truths. | `@claim:online-feed-scans`; `@claim:no-account-or-payment`; [live home mobile screenshot](qa-artifacts/polish-3-live/home-mobile.png). |

## Verification

- Fresh clone `/tmp/icw-polish-3-clean-ONSrVZ` at `b0a9016…`: `npm ci`, typecheck, Vitest, 28 Rust tests, build, full browser suite, and accessibility suite passed. The claim runner's `test-results/.last-run.json` reports `passed` after all 28 literal registry commands.
- Local full browser suite: 69 passed, 3 intentional project skips. Live accessibility/metadata/routing suite: 20 passed.
- Live Axe WCAG 2 A/AA returned zero violations on `/`, `/demo`, `/privacy`, `/terms`, and `/missing-polish-3`.
- `verify-url.sh https://integration-changelog-watch.sociobot.in/demo` passed with no console errors; its evidence is under `qa-artifacts/polish-3-live/`.
- Cold live mobile recheck found no horizontal overflow. The home retains the paper-cut operations-board visual identity, and the demo starts with usable sample work.

No review finding from rounds 1, 2, or 3 remains open.
