# Handoff — independent verification 4

## Release result: FAIL — do not release

- Tested commit: `fabea5e036adfc5bf820e719083766e80902e2ce`
- Live URL: `https://integration-changelog-watch.sociobot.in`
- Verified: 2026-08-28 UTC
- Scope: independent QA only; no product code was changed.

The live `/health` response reports the exact tested commit. First-read, all
declared claims, local tests/builds, demo privacy/accessibility/mobile, and
the public CLI pass. Release is blocked by the ingress rate-limit contract and
standard RSS CDATA parsing. Full evidence: `.factory/verification-4.md`.

## Release-blocking defects

1. **Major — rate limiter does not use the mandatory ingress client IP.**
   `src/main.rs` deliberately ignores `X-Forwarded-For` and keys on the TCP
   peer. One live serial client made 94 rapid requests before a 429 (94 × 401,
   then 6 × 429 with `Retry-After: 1`), exceeding the configured 40-request
   burst and failing the first-hop XFF contract.
2. **Major — common RSS CDATA content is not decoded.** A live public RSS scan
   created a card with literal title `<![CDATA[Unix V4 Workshop at Low Resource
   Computing]]>` instead of readable notice text.

## Passing checks

```sh
npm ci
npm test && npm run typecheck && npm run lint
cargo fmt --all -- --check && cargo test --locked
npm run test:container && npm run test:a11y && npm run test:browser
cargo clippy --all-targets --locked -- -D warnings
```

Every command in `.factory/claims.json` passed. Run the app with
`npm run build && cargo run`, then open `http://localhost:8080/demo`. The CLI
workflow is:

```sh
cargo run -- demo
cargo run -- scan --config examples/watches.json
```

Docker/Podman/Buildah was unavailable, so the Docker image itself was not run;
the exact locked release binary build passed.

## Next steps

Use a trusted ingress-sanitized first `X-Forwarded-For` hop with a test proving
the per-client allowance and `Retry-After`. Decode RSS CDATA/XML text before
creating cards and add an end-to-end CDATA fixture regression.
