import { describe, expect, it } from 'vitest'
import { sampleActions, sampleWatches } from './sample'
import { actionCsv } from './csv'

describe('sample workspace', () => {
  it('provides owned cards with a local check', () => {
    expect(sampleWatches).toHaveLength(3)
    expect(sampleActions.some(a => !a.acknowledged && a.owner && a.command && a.matched)).toBe(true)
  })
  it('has a CSV row for each action', () => {
    const rows = actionCsv(sampleActions).split('\n')
    expect(rows[0]).toContain('"title"')
    expect(rows).toHaveLength(sampleActions.length + 1)
    expect(rows[1]).toContain('Stripe retires legacy webhook event format')
  })
})
