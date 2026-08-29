# Polish 1 — adversarial review closure

Deployed product revision: `c2b3a9716b68357317753ab71eb78b7af3d12b9a`.
Live evidence: <https://integration-changelog-watch.sociobot.in/demo?verify=c2b3a97>, `.factory/qa-artifacts/polish-1-live/demo-first-mobile.png`, and `.factory/qa-artifacts/polish-1-live/verify.json`.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | `/demo` and `?demo=1` now start on the seeded action board; no hero repeats there. | `@claim:sample-action-cards`; `demo-first-mobile.json` shows title, owner, version, and check in 390×844. |
| F-1-2 | Replaced nested watched-feed `<aside>` with a labelled `<section>`. | Full Axe in `test:a11y`, 16 live tests pass with zero violations. |
| F-1-3 | Added hosted scan-result claim and fixture-backed action-card test. | `cargo test --locked claim_hosted_scan_creates_action_card_from_controlled_fixture`. |
| F-1-4 | Removed price wording and registered the three-watch capacity claim. | `@claim:hosted-watch-limit`. |
| F-1-5 | Rewrote copy to keywords and added create/edit/reload coverage. | `@claim:keyword-edit`. |
| F-1-6 | Registered and tested four local CLI mappings. | `@claim:cli-more-feeds`. |
| F-1-7 | Replaced the dense endpoint sentence with a table and tested every documented route/method. | `@claim:api-contract`. |
| F-1-8 | Rewrote the Rust wording to the durable builder fact and tested the Docker build contract. | `@claim:container-build-stage`. |
| F-1-9 | Removed untested free/paid/team-exclusion marketing; retained tested watch scope only. | Copy audit and `@claim:hosted-watch-limit`. |
| F-1-10 | Removed the duplicate collaboration-exclusion sentence. | README copy audit. |
| F-1-11 | Split the README verification instruction. | README and `.factory/copy-audit.md`. |
| F-1-12 | Replaced endpoint inventory prose with a method/path table. | README and `@claim:api-contract`. |
| F-1-13 | Split the CLI persistence explanation into two sentences. | README. |
| F-1-14 | Standardized on “keywords” and “assigned action cards.” | `.factory/copy-audit.md`. |
| F-1-15 | Removed redundant hero eyebrow and illustration caption. | Live cold screenshot and `verify.json`. |
| F-1-16 | Renamed the banner action to “Start a private workspace” with discard note. | `@claim:demo-isolation-transitions`. |
| F-1-17 | Route-specific description, OG, Twitter, title, and canonical metadata now update together. | Live `test:a11y` route metadata assertions. |
| F-1-18 | Added missing 404 preview/touch metadata and the standard How-it-works navigation link. | Live `test:a11y` 404 assertions. |
| F-1-19 | Added previewed JSON watch-file export/import using the CLI schema, isolated in demo storage. | `@claim:watch-file-portability`. |

The live full browser suite passed: 59 passed, 1 intentional duplicate-project skip. `verify-url.sh` recorded no console errors and the live mobile demo evidence confirms the first-viewport contract.
