import { expect, test } from '@playwright/test'

test('@live:rate-limit-spoof-resistance keeps caller-supplied X-Forwarded-For prefixes in one client bucket', async ({ request }) => {
  test.skip(process.env.ICW_LIVE_RATE_LIMIT_PROBE !== '1', 'Run only as an isolated post-deploy probe against the hosted ingress.')

  // Azure Container Apps appends the actual client address at the right of
  // XFF. These 80 different left prefixes must therefore share one 40-request
  // bucket. Keep this separate from the regular suite because it deliberately
  // exhausts the hosted client bucket.
  const responses = await Promise.all(Array.from({ length: 80 }, async (_, index) => {
    const prefix = `203.0.${Math.floor(index / 254)}.${(index % 254) + 1}`
    const response = await request.get('/api/watches', { headers: { 'x-forwarded-for': prefix } })
    return { status: response.status(), retryAfter: response.headers()['retry-after'] }
  }))
  const allowed = responses.filter(response => response.status === 401)
  const limited = responses.filter(response => response.status === 429)
  expect(allowed).toHaveLength(40)
  expect(limited).toHaveLength(40)
  expect(limited.every(response => response.retryAfter === '1')).toBe(true)
})
