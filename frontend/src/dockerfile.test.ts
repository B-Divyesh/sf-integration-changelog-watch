import { readFileSync } from 'node:fs'
import { describe, expect, it } from 'vitest'

describe('container build contract', () => {
  const dockerfile = readFileSync(new URL('../../Dockerfile', import.meta.url), 'utf8')

  it('uses the current stable Rust builder compatible with the locked ICU dependency graph', () => {
    expect(dockerfile).toContain('FROM rust:1-alpine AS build')
    expect(dockerfile).toContain('COPY package.json package-lock.json vite.config.ts tsconfig.json ./')
    expect(dockerfile).toContain('RUN npm ci --ignore-scripts && npm run build')
    expect(dockerfile).toContain('COPY Cargo.toml Cargo.lock ./')
    expect(dockerfile).toContain('RUN cargo build --release --locked')
  })

  it('ships a repair deployment that cannot restore the generic multi-replica topology', () => {
    const deploy = readFileSync(new URL('../../deploy/deploy-repair.sh', import.meta.url), 'utf8')
    expect(deploy).toContain('"maxReplicas":1')
    expect(deploy).toContain('"minReplicas":1')
    expect(deploy).toContain('"storageType":"AzureFile"')
    expect(deploy).toContain('"mountPath":"/data"')
    expect(deploy).toContain('terminationGracePeriodSeconds')
  })
})
