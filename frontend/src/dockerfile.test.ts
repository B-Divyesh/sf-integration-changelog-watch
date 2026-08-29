import { readFileSync } from 'node:fs'
import { describe, expect, it } from 'vitest'

describe('container build contract', () => {
  const dockerfile = readFileSync(new URL('../../Dockerfile', import.meta.url), 'utf8')

  it('@claim:container-build-stage uses the official Rust build stage and locked build command', () => {
    expect(dockerfile).toContain('FROM rust:1-alpine AS build')
    expect(dockerfile).toContain('COPY package.json package-lock.json vite.config.ts tsconfig.json ./')
    expect(dockerfile).toContain('RUN npm ci --ignore-scripts && npm run build')
    expect(dockerfile).toContain('COPY Cargo.toml Cargo.lock ./')
    expect(dockerfile).toContain('RUN cargo build --release --locked')
    expect(dockerfile).toContain('RUN printf \'%s\' "$BUILD_SHA" > /build-sha')
    expect(dockerfile).toContain('COPY --from=build /build-sha /app/build-sha')
  })

  it('keeps the container-build claim on Vitest’s supported exact-name filter', () => {
    const claims = JSON.parse(
      readFileSync(new URL('../../.factory/claims.json', import.meta.url), 'utf8'),
    ) as Array<{ id: string; test: string }>
    const claim = claims.find(({ id }) => id === 'container-build-stage')

    expect(claim?.test).toBe(
      "npm test -- --testNamePattern '^container build contract @claim:container-build-stage uses the official Rust build stage and locked build command$'",
    )
    expect(claim?.test).not.toContain('--grep')
  })

  it('ships a repair deployment that cannot restore the generic multi-replica topology', () => {
    const deploy = readFileSync(new URL('../../deploy/deploy-repair.sh', import.meta.url), 'utf8')
    expect(deploy).toContain('"maxReplicas":1')
    expect(deploy).toContain('"minReplicas":1')
    expect(deploy).toContain('"storageType":"AzureFile"')
    expect(deploy).toContain('"mountPath":"/data"')
    expect(deploy).toContain('terminationGracePeriodSeconds')
    expect(deploy).toContain('"name":"BUILD_SHA","value":"$sha"')
    expect(deploy).toContain('git -C "$root" status --porcelain')
    expect(deploy).toContain('git -C "$root" ls-remote origin refs/heads/main')
    expect(deploy).toContain('release-identity.mjs" published "$sha" "$remote_sha"')
    expect(deploy).toContain('release-identity.mjs" live "$sha"')
    expect(deploy).not.toContain('PREBUILT_IMAGE')
    expect(readFileSync(new URL('../../scripts/release-identity.mjs', import.meta.url), 'utf8')).toContain('data-build')
    expect(readFileSync(new URL('../../src/main.rs', import.meta.url), 'utf8')).toContain('reconcile_production_topology')
  })

  it('makes each Playwright claim command own and stop its port 8080 server', () => {
    const config = readFileSync(new URL('../../playwright.config.ts', import.meta.url), 'utf8')
    const packageJson = JSON.parse(
      readFileSync(new URL('../../package.json', import.meta.url), 'utf8'),
    ) as { scripts: Record<string, string> }
    const claimRunner = readFileSync(
      new URL('../../scripts/run-claims.mjs', import.meta.url),
      'utf8',
    )

    expect(config).toContain("command: 'cargo build --quiet && exec target/debug/integration-changelog-watch'")
    expect(config).toContain("gracefulShutdown: { signal: 'SIGTERM', timeout: 10_000 }")
    expect(packageJson.scripts['test:claims']).toBe('node scripts/run-claims.mjs')
    expect(claimRunner).toContain('await assertClaimPortReleased()')
  })

  it('ships the required square 180px touch icon', () => {
    const png = readFileSync(new URL('../public/apple-touch-icon.png', import.meta.url))
    expect(png.subarray(1, 4).toString()).toBe('PNG')
    expect(png.readUInt32BE(16)).toBe(180)
    expect(png.readUInt32BE(20)).toBe(180)
  })
})
