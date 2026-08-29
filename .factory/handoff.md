# Handoff — adversarial review 4

## Outcome

**FAIL — no product code was changed.** The full report is in
`.factory/review-4.md`.

The one-click landing CTA can enter `/demo` while a landing-created private
workspace request is in flight. When that request completes, it writes a real
`icw:workspace-token` while the demo banner is visible. This is blocking
finding `F-4-1`; it breaks the advertised isolated-demo boundary.

## What was verified

- Fresh 390 px and desktop live first reads are clear; the demo initially shows
  realistic sample work, and direct `/demo` has no API or third-party request.
- A live delayed-request Playwright reproduction proved `F-4-1`.
- Fresh clone checks passed: `npm test` (10), typecheck, Vite build, 28 Rust
  tests, Rustfmt, `npm run test:browser` (69 pass, 3 documented skips), and
  the live accessibility suite (20 pass).
- The registered claim suite was run from the clean clone. No listed claim test
  failed. Copy, claim registry, history, routing, links, metadata, accessibility,
  privacy request logging, and visual identity were rechecked.

## Required next step

Defer all private-workspace creation until an explicit real-workspace action,
and prevent in-flight hydration from writing real keys after demo navigation.
Add a delayed `/api/workspaces` CTA-race test to the demo-isolation claim, then
rerun the complete review.
