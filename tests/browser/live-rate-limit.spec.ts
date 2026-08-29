import { expect, test } from '@playwright/test'

test('@live:rate-limit-spoof-resistance uses the ingress client and keeps health available', async ({ request }, testInfo) => {
  test.skip(process.env.ICW_LIVE_RATE_LIMIT_PROBE !== '1', 'Run only as an isolated post-deploy probe against the hosted ingress.')
  // This test intentionally drains one full per-client bucket. The mobile
  // project would be the same hosted client and make the 40/40 assertion race
  // with desktop, so one browser project is the complete live contract.
  test.skip(testInfo.project.name === 'mobile', 'Desktop owns the isolated per-client rate-limit probe.')

  // Azure Container Apps appends the actual client address at the right of
  // XFF. These different left prefixes must therefore share one finite
  // client bucket. A 100-request burst is large enough to observe 429 even
  // while the 20-request/second refill runs during a real ingress fan-out.
  // Keep this separate from the regular suite because it deliberately drains
  // the hosted client bucket.
  const responses = await Promise.all(Array.from({ length: 100 }, async (_, index) => {
    const prefix = `203.0.${Math.floor(index / 254)}.${(index % 254) + 1}`
    const response = await request.get('/api/watches', { headers: { 'x-forwarded-for': prefix } })
    return { status: response.status(), retryAfter: response.headers()['retry-after'] }
  }))
  const allowed = responses.filter(response => response.status === 401)
  const limited = responses.filter(response => response.status === 429)
  expect(allowed.length).toBeGreaterThanOrEqual(40)
  expect(limited.length).toBeGreaterThan(0)
  expect(allowed.length + limited.length).toBe(100)
  expect(limited.every(response => response.retryAfter === '1')).toBe(true)

  const health = await request.get('/health')
  expect(health.status()).toBe(200)
  expect(await health.json()).toMatchObject({ ok: true })
})
