# Handoff — polish round 4

## Outcome

**PASS — every adversarial finding from rounds 1–4 is closed.**

The application repair revision `f2285321d9f76bfcf286a2bd6c441258df593f9f` is live at <https://integration-changelog-watch.sociobot.in>. The deployed image is `sociobotregistry.azurecr.io/sf-integration-changelog-watch:f2285321d9f7`, digest `sha256:885b03933636c1c724f22bda76ce75ec32894ad4700b1645bba535cb02f1418e`.

## What changed

- Removed automatic private-workspace creation from the cold landing page.
- Made `/?demo=1` the landing CTA and header demo route while retaining `/demo` as a real direct route.
- Abort real-workspace requests on route changes and prevent stale responses from writing a token or cached real data.
- Create a workspace only after an explicit real-workspace action, including **Start a private workspace**.
- Expanded `demo-isolation-transitions` to delay the workspace endpoint and prove the landing CTA makes no API call or `icw:*` write.
- Updated tests that previously depended on eager creation to start or seed a workspace explicitly.
- Removed the last `rules`/`owned actions` terminology leaks from Terms, Cargo metadata, and CLI Markdown output.
- Updated the demo document, copy audit, 85-character verb-first catalog description, claims registry, 404 demo link, and route metadata coverage.
- Preserved the warm paper, indigo thread, clipped-card, editorial-serif visual system in `.factory/design.md`.

The full finding-by-finding matrix is in `.factory/polish-4.md`.

## Verification

Final clean clone: `/tmp/icw-polish4-clean-IhdgJA` at `f2285321d9f76bfcf286a2bd6c441258df593f9f`.

```sh
npm ci
npm run test:claims
npm test
npm run typecheck
npm run build
cargo test --locked
cargo fmt --all -- --check
npm run test:browser
npm run test:a11y
```

Results:

- 28/28 literal claim commands passed with port 8080 released between commands.
- 10/10 Vitest tests and 28/28 Rust tests passed.
- Production build: JS 23.28 kB raw / 7.84 kB gzip; CSS 9.03 kB raw / 2.79 kB gzip.
- Full browser suite: 69 passed; three documented project-specific probes skipped.
- Accessibility/routing suite: 20/20 passed with zero Axe violations.
- Live demo/privacy/offline subset: 8/8 passed.
- Live rate-limit probe: passed; a 100-request burst produced 429 plus `Retry-After: 1`, while `/health` stayed 200.
- `verify-url.sh` passed on `/` and `/?demo=1` with zero console errors.
- Lighthouse mobile: 100 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 1.20 s, CLS 0, TBT 0, 93,893 transferred bytes.
- Live `/health` returned `f2285321d9f76bfcf286a2bd6c441258df593f9f`.

Primary evidence is under `.factory/qa-artifacts/polish-4-live/`, especially `cold-live-audit.json`, `cold-home-mobile.png`, `cold-demo-mobile.png`, the 404/legal screenshots, both `verify.json` files, and `lighthouse-mobile.json`.

## Run and deploy

Use the commands above for local verification. Run locally with `cargo run` after `npm run build`; use `DATABASE_URL='sqlite:changelog-watch.db?mode=rwc'` without a `/data` mount.

Deployment uses `./deploy/deploy-repair.sh`. It builds from a pushed clean revision, preserves the single Azure Files-backed replica, and checks live build identity.

## Known gaps and next steps

None. The product intentionally makes no offline-reload claim; the tested offline state explains that public-feed scans need a connection. No runtime AI or paid feature is implied by this deterministic workflow.
