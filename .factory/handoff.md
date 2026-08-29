# Handoff — adversarial first-read review 3

## Outcome

**FAIL — four findings, including one blocking reopened history finding.**

The review is in `.factory/review-3.md`. No product code was changed.

## What was done

- Audited the live landing page cold at 390 × 844 and 1440 × 900.
- Exercised the one-click and direct demo, reset, acknowledgement, storage isolation, and request privacy.
- Ran all 21 literal `.factory/claims.json` commands from clean clone `/tmp/icw-review-3-jDUdzM`; all passed.
- Rechecked every F-1 and F-2 finding against the live product and current source.
- Audited landing and README copy sentence by sentence.
- Crawled routes and links; checked metadata, 404, hash/back/focus behavior, assets, CSP, and visual identity.
- Ran full Axe checks, `/opt/fleet/lib/verify-url.sh`, the 20-test live accessibility suite, `npm test`, typecheck, build, and all 23 Rust tests.

## Findings left

- Blocking: F-1-14/F-3-1 — “Hosted workspace scope” remains from the earlier jargon finding.
- Major: F-3-2 — the Azure Files dot-file locking sentence has no exact claims entry/test mapping.
- Major: F-3-3 — the product lacks opt-in scheduled scanning implied by “Watch.”
- Minor: F-3-4 — the first-screen facts omit online and price truth.

## Verification summary

- Registered claims: 21/21 passed.
- Live accessibility/metadata/routing suite: 20/20 passed.
- Vitest: 9/9 passed.
- Rust: 23/23 passed.
- Build: passed; production JS 6.95 kB gzip.
- Live `/health`: `818b868c0ba7ecece8fdae9b4abb4d6b927bdae1`.

## Next step

Resolve all four findings, add the specified claim and regression tests, deploy, and repeat the full review from fresh browser contexts. PASS requires zero findings.
