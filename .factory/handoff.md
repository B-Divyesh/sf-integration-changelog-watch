# Handoff — adversarial review 1

## Outcome

**FAIL.** The full report is `.factory/review-1.md`. No product code was modified.

The cold first screen clearly states the job, audience, and first action. All 13 listed claim commands pass from a clean clone, and demo storage/request isolation works. The release still has two blocking findings: `/demo` repeats the landing hero instead of showing realistic sample work in the first 390 px viewport, and the moderate Axe landmark issue recorded by the earlier handoff remains live.

## Work completed

- Opened the live site cold in fresh 390 × 844 and 1440 × 900 browser contexts before scrolling.
- Audited every landing/README copy unit with word counts, terminology, claim coverage, headings, and action labels.
- Exercised demo entry, acknowledge, Reset demo, Start for real, storage namespaces, and a direct-demo request log.
- Ran every exact `.factory/claims.json` command after `npm ci` in clean clone `/tmp/icw-review-1-9qlY2f`; all passed.
- Rechecked the earlier handoff, route titles/metadata, deep links, browser Back and focus, the designed 404, crawled links, mobile reflow/touch targets, full Axe, privacy requests, visual identity, and missed leverage.
- Added review evidence under `.factory/review-1-artifacts/`.

## Verification run

```sh
npm test
npm run typecheck
npm run build
PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run test:a11y
```

These commands passed. The separate full-Axe inspection still reports `landmark-complementary-is-top-level` for `.watches`; the shipped accessibility test filters out moderate findings.

## Required next work

Resolve all findings in `.factory/review-1.md`, starting with F-1-1 and F-1-2. Then rerun the review from scratch. Do not treat the passing listed claims as a release pass while the demo first-view contract, historical accessibility finding, unlisted claims, and minor copy/metadata findings remain.
