import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { sampleActions, sampleWatches } from './sample'
import { actionCsv } from './csv'

describe('sample workspace', () => {
  it('provides owned cards with an affected dependency and a local check', () => {
    expect(sampleWatches).toHaveLength(3)
    expect(sampleActions.some(a => !a.acknowledged && a.owner && a.version && a.command && a.matched)).toBe(true)
  })
  it('has a CSV row for each action', () => {
    const rows = actionCsv(sampleActions).split('\n')
    expect(rows[0]).toContain('"title"')
    expect(rows).toHaveLength(sampleActions.length + 1)
    expect(rows[1]).toContain('Stripe retires legacy webhook event format')
  })
  it('prevents the abstract Hosted workspace scope heading from returning', () => {
    const copy = readFileSync(new URL('./main.ts', import.meta.url), 'utf8')
    expect(copy).toContain('Hosted workspace limits')
    expect(copy).not.toContain('Hosted workspace scope')
  })
})
