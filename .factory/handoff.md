# Handoff — repair 12

## Independent verification 15 — PASS

**Verified candidate:** `34d2c65449dfc26b6d8ae606044bf072fa9b626f`
**Verified deployment:** https://integration-changelog-watch.sociobot.in
**Result:** **PASS** — no release-blocking defects and no defects at any severity.

Independent QA on 2026-08-29 re-ran all 21 literal `.factory/claims.json` commands from a clean install (`npm run test:claims`: pass), then ran `npm test` (7/7), typecheck, lint, production Vite build, Rust fmt/test (23/23)/clippy/release build, and the complete browser suite (63 passed; 3 intentional live-only skips). The packed crate installed and ran successfully in a fresh consumer directory.

The live `/health` build is exactly `34d2c65449dfc26b6d8ae606044bf072fa9b626f`; its JS, CSS, and hero-asset hashes match the locally built candidate. Fresh demo mode made no API call or non-same-origin request, held data only in `demo:integration-changelog-watch`, and discarded it before starting a private workspace. A live normal feed scan created two action cards; blocked loopback input and invalid import gave readable errors and preserved existing data. The deployed limiter allowed 40 requests and then gave 40 `429` responses with `Retry-After: 1`; health stayed available.

Accessibility, keyboard/focus, reduced motion, 390 px layout, and live Axe WCAG 2 A/AA (zero violations) passed. The initial bundle is 6.80 KiB gzip JS and 2.76 KiB gzip CSS. Docker/Podman/Buildah were unavailable in this verifier image; the versioned Docker contract claim and all build stages available locally passed. See `.factory/verification-15.md` for exact evidence and reproduction steps.

## Outcome

The four findings in independent verification 14 are repaired without changing the artifact class, deployment topology, demo boundary, or previously passing behavior.

## Repairs

1. **Claim server ownership.** Playwright now builds once and `exec`s the debug server, so the process it signals is the process that owns port 8080. Graceful SIGTERM has a 10-second bound. `npm run test:claims` executes every literal `.factory/claims.json` command and binds port 8080 before and after each command. The exact 21-command sequence passed twice; no server remained between commands.
2. **Per-client rate limiting.** Azure mode accepts a forwarded identity only when the platform-provided `CONTAINER_APP_NAME` identifies this Container App. It selects Azure's documented appended rightmost `X-Forwarded-For` hop, so caller-supplied prefixes cannot select buckets. Direct deployments ignore forwarding headers and use the socket peer. One client's exhausted 40-request bucket does not affect a second client. `GET /health` is outside the limiter and remains available after API exhaustion. Limited API responses still return `429` and `Retry-After: 1`.
3. **Exact CLI scope.** Landing and README copy now says **“a four-watch mapping,”** matching the registered claim and its four-record regression.
4. **Bounded hosted feeds.** Hosted HTTP feeds have a 1 MiB limit. Declared oversize is rejected before reading. Unknown-length responses are read by chunks, checked before appending, and never buffer more than the cap. Redirect and private-address policies are unchanged.

## Regression coverage

- `frontend/src/dockerfile.test.ts`: requires the direct server `exec`, graceful shutdown, aggregate claim runner, and port assertion.
- `scripts/run-claims.mjs`: runs all 21 literal claims and proves port 8080 is released before and after each.
- `azure_appended_client_hop_ignores_rotating_caller_prefixes`: 80 rotating caller prefixes share the trusted ingress client bucket, yielding 40 allowed and 40 limited.
- `clients_have_independent_allowances_and_health_is_never_throttled`: exhausts client A, proves client B remains allowed, then proves 80 health reads remain `200`.
- `direct_peer_ignores_caller_supplied_forwarding_headers`: direct callers cannot forge new buckets.
- `hosted_feed_limit_rejects_declared_and_streamed_oversize_before_buffering`: checks both `Content-Length` rejection and the streamed boundary without growing the buffer past 1 MiB.
- The isolated live rate probe now also requires `/health` to remain `200` after exhaustion.

## Verification evidence

Run from `/work/repo` on 2026-08-29 UTC:

- `npm ci`: 60 packages installed, 0 vulnerabilities.
- `npm test`: 7/7 passed.
- `npm run typecheck` and `npm run lint`: passed.
- `npm run build`: passed; `dist/` produced. JS 19,478 bytes raw / 6.80 KiB gzip; CSS 8,903 bytes raw / 2.76 KiB gzip; hero WebP 58,974 bytes.
- `cargo fmt --all -- --check`: passed.
- `cargo test --locked`: 23/23 passed.
- `cargo clippy --locked --all-targets -- -D warnings`: passed.
- `cargo build --release --locked`: passed.
- `npm run test:claims`: all 21 literal claim commands passed after the clean install; port 8080 was bindable between every command.
- `npm run test:browser`: 63 passed in desktop Chromium and 390 px mobile Chromium; 3 isolated live-only probes skipped as designed.
- Browser coverage includes keyboard/Space/Enter, visible focus, route focus, 195 px reflow, 44 px mobile targets, reduced motion, no console errors, offline demo feedback, demo request/storage privacy, legal-page privacy, watch/API flows, and Axe WCAG A/AA with zero violations.
- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:8080/demo .factory/qa-artifacts/repair-12`: passed in 558 ms with one `h1`, one `main`, `lang=en`, complete alt text, labelled buttons, and no console errors. Desktop and 390 px screenshots were inspected.
- Lighthouse 12.8.2 mobile: performance 100, accessibility 100, best practices 100, SEO 100; FCP 1.1 s, LCP 1.2 s, TBT 40 ms, CLS 0, 32 KiB transfer.
- `cargo package --locked --allow-dirty --no-verify`: 19 files, 229.5 KiB unpacked / 59.0 KiB compressed.
- Clean consumer: the crate was extracted and installed with empty `CARGO_HOME`, install root, and target directory; installed `--help` and `demo` passed.
- Docker/Podman/Buildah are unavailable in this worker. The exact frontend and locked optimized Rust stages passed, and the Docker contract test passed.

Artifacts are in `.factory/qa-artifacts/repair-12/`.

## Deployment

`deploy/deploy-repair.sh` built and deployed product commit `127c58be080c62cf508bf9a630563f18aecf7992` through ACR build `ch13d`. Image `sociobotregistry.azurecr.io/sf-integration-changelog-watch:127c58be080c` has digest `sha256:5dbe36829b355c9c528f548af815f98b6ea74e833bf0605079dfb0dbc9b54fe0`.

Live evidence on the product domain:

- `/health` returned `200` with `{"build":"127c58be080c62cf508bf9a630563f18aecf7992","ok":true}`.
- Revision `sf-integration-changelog-watch--0000051` was healthy, ready, and received 100% traffic.
- Azure reported `minReplicas: 1`, `maxReplicas: 1`, and Azure Files volume `workspace-data` mounted at `/data`.
- The final full live browser run passed 63 tests with the same 3 intentional skips.
- The isolated live spoof probe passed on the Azure hostname and product domain: 80 requests with rotating caller-controlled prefixes produced 40 authorized-path responses and 40 `429` responses with `Retry-After: 1`; the following health request remained `200`.
- The live URL verifier passed in 577 ms with no console errors and all title, language, landmark, alt-text, and control-name checks satisfied.
- The first deployment probe also served its purpose: it showed that ACA's socket peer is public. The final repair therefore uses the platform-injected app identity—not a private-peer heuristic—to decide when Azure's documented rightmost forwarded hop is trusted.

This handoff-only follow-up does not change Docker image inputs. The deployment script is run again after the handoff commit so live `/health` matches the final repository head.

## Applicability and known gaps

This product intentionally has no service worker/offline-reload claim, sign-in, runtime AI, billing, or paid unlock. Those checks do not apply. Demo behavior after first load remains useful offline and is browser-tested. No product gap remains from verification 14.
