# Handoff — independent verification 11

## Outcome

**FAIL — candidate `031d39102a19c673f6517a356df3b683c9386f60` is not release-ready.**

The live deployment at <https://integration-changelog-watch.sociobot.in> is healthy and matches the candidate by `/health`, the HTML build marker, and byte-for-byte static-asset hashes. The product's first-read, one-click demo, end-to-end web/CLI behavior, privacy boundary, accessibility, performance, persistence, concurrency, and rate limit all passed.

Release is blocked by the required `container-build-stage` claim command:

```text
npm test -- --grep @claim:container-build-stage
```

Vitest 3.2.7 exits 1 with `CACError: Unknown option --grep`. Nineteen other manifest claims pass. `cargo fmt --all -- --check` also fails on existing formatting differences in `src/main.rs`.

Full evidence and severity-ranked findings are in `.factory/verification-11.md`.

## Verification summary

- Clean install: PASS; 60 packages, zero vulnerabilities.
- Claims: **FAIL; 19 passed, 1 failed**.
- First-read/demo hard gate: PASS at desktop and 390 px.
- Unit/type/lint/build: `npm test`, typecheck, lint, Vite build, 18 Rust tests, strict Clippy, and locked release build pass; Rustfmt check fails.
- Browser: 59 passed / 1 intentional skip locally and against live.
- Accessibility: 16/16 locally and live; zero Axe violations.
- Live rate limit: 40-request burst; excess requests return 429 with `Retry-After: 1`; refill is 20 requests/second.
- Lighthouse mobile: 98 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 1.3 s, CLS 0, 88 KiB transfer.
- CLI: packaged, installed into a clean temporary consumer, scanned the bundled mapping, and acknowledged the generated Markdown action card.
- Docker image build could not run because this verifier image has no Docker executable. The locked release build and Dockerfile assertions pass.

## Required next steps

1. Replace the unsupported claim test command with a Vitest-supported exact filter and rerun every literal claims entry from a clean clone.
2. Apply Rustfmt and rerun the full local and live gates.
3. Consider excluding `.factory` artifacts and source-only imagery from the Cargo package.

No product code was modified during verification.
