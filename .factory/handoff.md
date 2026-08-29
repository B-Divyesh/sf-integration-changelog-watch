# Handoff — independent verification 6

## Release state

**FAIL — do not release.** Verified candidate `5e14f6f44e25eb3c733c1708522be0ca2197cade` at `https://integration-changelog-watch.sociobot.in` on 2026-08-29 UTC. No product code or deployment setting was changed.

The exact candidate image and static assets are live, all nine declared claim commands pass locally after `npm ci`, and local build/test/accessibility/CLI gates pass. Production fails under its configured scale-out: Azure reports `minReplicas=1`, `maxReplicas=3`, three running replicas, and no shared volume. Each replica uses local SQLite.

Fresh live evidence after scale-out:

- a new token received 16 × 200 and 32 × 401 across 48 reads;
- all five cold `/` visits created a workspace but then showed **“Your private workspace could not load”** with a console 401;
- targeted live workspace Playwright: 3 failed, 1 skipped;
- a 120-request/811 ms single-client burst received no 429 despite the code's 40-request bucket; a 400-request/3,174 ms burst finally produced 111 × 429, all with `Retry-After: 1`, after 289 requests were accepted by the per-replica limiter.

The claims inventory also omits public promises for demo API isolation/transitions, the shipped CLI scan's no-network behavior, PORT-only startup, and parts of the stated non-goals. The server has no graceful SIGTERM shutdown path. SPA and 404 footer versions are inconsistent (`v2` vs `v3`).

## Verification summary

- First-read and one-click demo gate: PASS.
- All nine exact claim commands after clean install: PASS locally.
- `npm test`, typecheck, lint, production build: PASS.
- Rust format, 11 locked tests, warnings-as-errors Clippy, locked release build: PASS.
- Local browser: 39 passed, 1 intentional skip; accessibility: 12 passed.
- Initial low-load live browser: 39 passed, 1 intentional skip; post-scale workspace rerun: FAIL.
- Clean packed CLI install and `demo`/scan/deduplicate/ack flow: PASS.
- Live normal scan/ack/deduplicate/delete flow on one replica: PASS; invalid empty/overlong/private inputs: PASS.
- Axe: zero serious/critical at desktop and 390 px. No demo third-party requests or console errors.
- Lighthouse mobile: 91 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 1.6 s, CLS 0.
- Live `/health`: exact candidate SHA. Local/live index, JS, CSS, and hero hashes match.

Full evidence and required repairs are in `.factory/verification-6.md`.

## Required next steps

1. Move workspace state and rate-limit state to shared durable infrastructure, or force exactly one replica and attach a verified durable `/data` volume.
2. Re-run the live cold-page, 48-read consistency, workspace claims, restart persistence, and limiter tests under scale.
3. Add claim entries/tests for every remaining public promise or remove those promises.
4. Add graceful SIGTERM/SIGINT shutdown and a regression test.
5. Use one real version/build identity in every footer.

The tree remains buildable. This verification changed documentation only.
