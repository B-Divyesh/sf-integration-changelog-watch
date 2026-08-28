import { expect, test } from '@playwright/test'

test('real workspace renders the server action schema and acknowledges its numeric ID', async ({ page }) => {
  let acknowledged = false
  await page.route('**/api/workspaces', route => route.fulfill({ status: 201, contentType: 'application/json', body: JSON.stringify({ token: 'a'.repeat(64) }) }))
  await page.route('**/api/watches', route => route.fulfill({ contentType: 'application/json', body: JSON.stringify([{ id: 7, vendor: 'Vendor', url: 'https://vendor.example/feed', keywords: 'webhook', owner: 'Maya', version: '', command: 'npm test' }]) }))
  await page.route('**/api/actions', route => route.fulfill({ contentType: 'application/json', body: JSON.stringify([{ id: 42, watchId: 7, title: 'Webhook change', excerpt: 'A real feed notice', matched: 'webhook', url: 'https://vendor.example/notice', owner: 'Maya', command: 'npm test', acknowledged, seenAt: 'Today' }]) }))
  await page.route('**/api/actions/42', async route => {
    expect(route.request().method()).toBe('POST')
    expect(route.request().postDataJSON()).toEqual({ acknowledged: true })
    acknowledged = true
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify({}) })
  })
  await page.goto('/')
  await expect(page.getByRole('heading', { name: 'Webhook change' })).toBeVisible()
  await expect(page.getByRole('link', { name: /Open vendor notice/ })).toHaveAttribute('href', 'https://vendor.example/notice')
  await page.getByRole('button', { name: 'Acknowledge action' }).click()
  await expect.poll(() => acknowledged).toBe(true)
})

test('@claim:workspace-boundary workspace endpoints reject callers without a workspace token and block loopback feeds', async ({ page }) => {
  await page.goto('/')
  const result = await page.evaluate(async () => {
    const unauthenticated = await fetch('/api/watches')
    const workspace = await fetch('/api/workspaces', { method: 'POST' }).then(response => response.json()) as { token: string }
    const privateFeed = await fetch('/api/watches', {
      method: 'POST',
      headers: { authorization: `Bearer ${workspace.token}`, 'content-type': 'application/json' },
      body: JSON.stringify({ vendor: 'Blocked', url: 'http://127.0.0.1/internal', keywords: 'webhook', owner: 'Maya', version: '', command: 'npm test' }),
    })
    return { unauthenticated: unauthenticated.status, privateFeed: privateFeed.status }
  })
  expect(result).toEqual({ unauthenticated: 401, privateFeed: 400 })
})
