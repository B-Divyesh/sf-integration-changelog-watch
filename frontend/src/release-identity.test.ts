import { spawnSync } from 'node:child_process'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const script = resolve(process.cwd(), 'scripts/release-identity.mjs')
const expected = 'a'.repeat(40)

function verify(args: string[], input?: string) {
  return spawnSync(process.execPath, [script, ...args], {
    cwd: process.cwd(),
    encoding: 'utf8',
    input,
  })
}

describe('release identity verifier', () => {
  it('accepts only a published source SHA that exactly matches origin main', () => {
    expect(verify(['published', expected, expected]).status).toBe(0)
    const mismatch = verify(['published', expected, 'b'.repeat(40)])
    expect(mismatch.status).toBe(1)
    expect(mismatch.stderr).toContain('Push the exact commit first')
  })

  it('rejects a live deployment when either health or the HTML build marker is stale', () => {
    const matching = `{"ok":true,"build":"${expected}"}\n<!doctype html><html data-build="${expected}"></html>`
    expect(verify(['live', expected], matching).status).toBe(0)

    const staleHealth = `{"ok":true,"build":"${'b'.repeat(40)}"}\n<!doctype html><html data-build="${expected}"></html>`
    expect(verify(['live', expected], staleHealth).stderr).toContain('Live /health build did not match')

    const staleHtml = `{"ok":true,"build":"${expected}"}\n<!doctype html><html data-build="${'b'.repeat(40)}"></html>`
    expect(verify(['live', expected], staleHtml).stderr).toContain('Live HTML data-build marker did not match')
  })
})
