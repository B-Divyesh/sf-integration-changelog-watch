# Handoff — perfection loop round 2

## Outcome

All 23 cumulative findings in `.factory/review-1.md` and `.factory/review-2.md` are closed. Repair revision `55b093c06cc0c83180cf3cb6a51d4301d4cf24b9` is live at <https://integration-changelog-watch.sociobot.in>.

The cold `/#how` route now restores its target after the SPA renders and after history navigation. The first-screen audience copy now says “authentication.” The existing isolated `/demo` and `?demo=1` paths, claims, legal routes, metadata, designed 404, mobile layout, import/export, and paper-cut visual identity remain intact.

## Exact verification evidence

- Clean clone of `55b093c06cc0…`: `npm ci && npm run test:claims` passed all 21 literal claim commands. Output: `.factory/qa-artifacts/polish-2/clean-claims.log`.
- Unit and compile: `npm test` passed 7 tests; `npm run typecheck` passed; `npm run build` produced `dist/` with 19.82 kB raw / 6.95 kB gzip JS and 8.90 kB raw / 2.76 kB gzip CSS.
- Backend: `cargo test --locked` passed all 23 tests. Output: `.factory/qa-artifacts/polish-2/cargo-test.log`.
- Browser: `npm run test:browser` passed 65 tests in desktop Chromium and mobile Chromium. Three intentional live/project probes were skipped. Output: `.factory/qa-artifacts/polish-2/browser-suite.log`.
- Live accessibility and structure: `PLAYWRIGHT_BASE_URL=https://integration-changelog-watch.sociobot.in npm run test:a11y` passed 20 tests, including full Axe WCAG 2 A/AA, route metadata, focus, mobile touch targets, 200% reflow, and 404 checks.
- Live demo/privacy/offline: the live `tests/browser/demo.spec.ts` run passed 16 tests. Both direct demo forms show populated work in the first 390×844 viewport, make no API call, and use only same-origin requests.
- Live ingress rate limit: the isolated probe passed with 40 allowed responses, 40 `429` responses carrying `Retry-After: 1`, and an available `/health` route.
- Cold route: `.factory/qa-artifacts/polish-2/live-cold-check.json` records `/#how` at `scrollY: 1984` on mobile and `1341` on desktop, intersecting both viewports with no console errors.
- First screen: `.factory/qa-artifacts/polish-2/live-first-screen.json` records the heading, full audience sentence, and sample-data action inside both first viewports with no horizontal overflow.
- URL verifier: `.factory/qa-artifacts/polish-2/live-verify/verify.json` records HTTP 200, correct demo title/lang/main/h1, complete alt/button labels, and zero console errors.
- Lighthouse mobile: 100 performance, 100 accessibility, 100 best practices, and 100 SEO; LCP 1.21 s, CLS 0, TBT 0, total transfer 90,553 bytes. Summary: `.factory/qa-artifacts/polish-2/lighthouse-summary.json`.
- Deployment: the checked-in repair script built and deployed `sociobotregistry.azurecr.io/sf-integration-changelog-watch:55b093c06cc0` at digest `sha256:efa092a7857e17f46967f9be9d0e1d28c7f5b46809667aee0c770c44e307ca61`. `/health` returns the full repair SHA.

Screenshots: `live-cold-mobile.png`, `live-cold-desktop.png`, `live-demo-mobile.png`, `live-demo-desktop.png`, `live-how-mobile.png`, and `live-how-desktop.png` under `.factory/qa-artifacts/polish-2/`.

## Run locally

```sh
npm ci
npm test
npm run typecheck
npm run build
cargo test --locked
npm run test:browser
npm run test:claims
```

## Known gaps and next steps

None. No review finding or test failure remains.
