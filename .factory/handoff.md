# Handoff — adversarial review 2

## Outcome

Review completed without product-code changes. The live product at `https://integration-changelog-watch.sociobot.in` **fails** this round with one blocking routing defect and one minor copy defect. Full evidence is in `.factory/review-2.md`.

## Verified

- Fresh 390 px and desktop first-read checks clearly identify the job, audience, and **Try it with sample data** action.
- The direct one-click demo starts on realistic populated action cards. Its banner, reset, private-workspace exit, storage separation, and request privacy were checked in a fresh browser context.
- All 21 literal `.factory/claims.json` commands passed from a clean clone after `npm ci`: `npm run test:claims`. The runner also confirmed no claim server leaked port 8080. Output is in `.factory/qa-artifacts/review-2-claims.log`.
- Full Axe WCAG 2 A/AA scan on landing, demo, Privacy, Terms, and 404 found zero violations. Metadata, legal links, 404, headers, CSP, assets, and discovered links were checked live.
- Every prior finding in review 1 and polish 1 was rechecked live and in code; all remain fixed.

## Known gaps / next steps

1. **Blocking F-2-1:** A fresh direct visit to `/#how` remains at the top of the landing page rather than scrolling to `#how`. Render-time hash handling and a cold-link browser regression test are required.
2. **Minor F-2-2:** Replace the audience shorthand “auth” with “authentication” on the landing page and README.

## Reproduce

```sh
npm ci
npm run test:claims
```

For the blocking finding, open `https://integration-changelog-watch.sociobot.in/#how` in a fresh 390 px browser context and verify that the How it works section enters the viewport. It currently does not.
