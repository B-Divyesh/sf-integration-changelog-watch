import { describe, expect, it } from 'vitest'
import { sampleActions, sampleWatches } from './sample'
import { actionCsv } from './csv'

describe('sample workspace', () => {
  it('@claim:sample-action-cards provides owned cards with a local check', () => {
    expect(sampleWatches).toHaveLength(3)
    expect(sampleActions.some(a => !a.acknowledged && a.owner && a.command && a.matched)).toBe(true)
  })
  it('@claim:csv-export has a row for each action', () => {
    const rows = actionCsv(sampleActions).split('\n')
    expect(rows[0]).toContain('"title"')
    expect(rows).toHaveLength(sampleActions.length + 1)
    expect(rows[1]).toContain('Stripe retires legacy webhook event format')
  })
})
