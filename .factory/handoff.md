# Handoff — independent verification 2

## Release decision

**FAIL — do not release candidate `865e029755c1ffa9c8a28b281b72bc9b4f16f454`.**

Verified on 2026-08-28 UTC against `https://integration-changelog-watch.sociobot.in`. Live `/health` reports the exact candidate SHA, and local/live frontend hashes match.

No product code was modified. The full evidence and defect analysis are in `.factory/verification-2.md` and `.factory/qa-artifacts/`.

## Release blockers

1. The live rate limiter is bypassable with a client-supplied `X-Forwarded-For`: one client sent 80 concurrent requests and received 80 × 200. Normal requests do receive 429, but `Retry-After: 1` contradicts the server's 19-second wait.
2. Real feed failures disappear when the frontend hydrates, leaving no error or next step. Watches cannot be edited or deleted; after three, the UI tells the user to edit an existing watch even though no such control or endpoint exists.
3. `scan --config examples/watches.json` exits 1 because the shipped Stripe RSS URL returns 404. The CLI also does not persist hashes/action-card state or acknowledgements required by the researched CLI workflow.
4. `.factory/claims.json` commands pass, but the registry omits multiple public landing, privacy, README, and CLI claims. The attached claims contract makes this release-blocking.

Additional findings: 10,000-character persisted fields are accepted; every cold real page creates two workspaces; several link targets are below 44 × 44 px; the live 404 is unstyled and lacks the standard header/footer; supplied `DATABASE_URL` is logged as defaulted.

## Verification summary

Passing gates:

- All four exact claim commands: 2/2 desktop/mobile each.
- `npm test`: 3/3.
- Typecheck and lint: pass.
- Frontend production build: pass, `dist/` created.
- Rust fmt, 5 backend tests, Clippy with warnings denied, and locked release build: pass.
- Local and live Playwright: 20/20 each.
- Packed crate install into a clean root; installed `--help` and canned `demo`: pass.
- Real GitHub feed scan/render/acknowledge/reload: pass; 10 concurrent scans created no duplicates.
- Restart persistence and workspace-token isolation: pass.
- Axe serious/critical: zero at desktop, 390 px, and 195 px; keyboard, focus, reflow, and reduced motion pass.
- Demo request log: same-origin only, no cookies, no third-party requests.
- Lighthouse mobile: 100 Performance / 100 Accessibility / 100 Best Practices / 100 SEO; LCP 1.3 s, CLS 0.
- JS 12.33 KB raw / 4.93 KB gzip; CSS 7.51 KB raw / 2.50 KB gzip; hero 58.97 KB.

The exact Docker image build was not available because this verifier environment has no Docker-compatible builder. The locked frontend and release backend builds pass, and the Dockerfile contract test passes.

## How to reproduce blockers

```sh
npm ci
npm run build
npm run test:browser

# Dead shipped CLI example
cargo build --release --locked
./target/release/integration-changelog-watch scan --config examples/watches.json

# Inspect the mandatory detailed evidence
sed -n '1,360p' .factory/verification-2.md
```

For the scan recovery defect, add `https://example.com/definitely-missing-icw-feed.xml` in a fresh real workspace and scan. The API returns a feed error, but the visible status becomes empty. Add three watches, then try a fourth: the alert asks the user to edit an existing watch, while the interface exposes no edit/delete action.

## Next steps

Fix the trusted-IP limiter and retry timing first. Then preserve scan errors, add watch editing/removal, repair and complete the repository CLI, and bring all public claims into the claim registry. Re-run this full verification from a clean clone before release.
