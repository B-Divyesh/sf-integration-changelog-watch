import { expect, test } from '@playwright/test'
import { readFile } from 'node:fs/promises'

test('@claim:sample-action-cards opens the seeded demo workspace', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 })
  await page.goto('/')
  await page.getByRole('button', { name: 'Try it with sample data' }).click()
  await expect(page).toHaveURL(/\/demo$/)
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible()
  await expect(page.getByRole('heading', { level: 1, name: 'Sample action cards' })).toBeVisible()
  await expect(page.getByText('Maya · Payments').first()).toBeVisible()
  await expect(page.getByText('stripe-node 16.2').first()).toBeVisible()
  await expect(page.getByText('pnpm test:stripe')).toBeVisible()
  const boxes = await Promise.all([
    page.getByRole('heading', { name: 'Stripe retires legacy webhook event format' }).boundingBox(),
    page.getByText('Maya · Payments', { exact: true }).first().boundingBox(),
    page.getByText('stripe-node 16.2', { exact: true }).first().boundingBox(),
    page.getByText('pnpm test:stripe', { exact: true }).boundingBox(),
  ])
  const viewport = page.viewportSize()!
  const firstView = boxes.map(box => Boolean(box && box.y >= 0 && box.y + box.height <= viewport.height))
  expect(firstView).toEqual([true, true, true, true])
})

test('an in-flight real workspace hydration cannot overwrite the demo sample', async ({ page }) => {
  await page.route('**/api/workspaces', route => route.fulfill({ status: 201, contentType: 'application/json', body: JSON.stringify({ token: 'f'.repeat(64) }) }))
  let releaseRead!: () => void
  const delayedRead = new Promise<void>(resolve => { releaseRead = resolve })
  for (const path of ['**/api/watches', '**/api/actions']) {
    await page.route(path, async route => {
      await delayedRead
      await route.fulfill({ contentType: 'application/json', body: '[]' })
    })
  }
  const watchesRequested = page.waitForRequest('**/api/watches')
  await page.goto('/')
  await watchesRequested
  await page.getByRole('button', { name: 'Try it with sample data' }).click()
  releaseRead()
  await expect(page.getByRole('heading', { level: 1, name: 'Sample action cards' })).toBeVisible()
  await expect(page.getByText('Maya · Payments').first()).toBeVisible()
})

test('@claim:csv-export downloads one row per demo action', async ({ page }) => {
  await page.goto('/demo')
  const download = page.waitForEvent('download')
  await page.getByRole('button', { name: 'Export action cards as CSV' }).click()
  const text = await readFile(await (await download).path(), 'utf8')
  expect(text.split('\n')).toHaveLength(3)
  expect(text).toContain('Stripe retires legacy webhook event format')
})

test('keyboard navigation exposes the skip link and can acknowledge a demo action', async ({ page }) => {
  await page.goto('/demo')
  await page.keyboard.press('Tab')
  await expect(page.getByRole('link', { name: 'Skip to content' })).toBeFocused()
  await page.keyboard.press('Enter')
  await expect(page.locator('#main')).toBeFocused()
  await page.getByRole('button', { name: 'Acknowledge action' }).focus()
  await page.keyboard.press('Space')
  await expect(page.getByText('No actions need acknowledgement')).toBeVisible()
  await expect(page.locator('[data-action="a1"]')).toBeFocused()
})

test('@claim:online-feed-scans offline scan gives a useful next step without making a request', async ({ page, context }) => {
  await page.goto('/demo')
  await context.setOffline(true)
  await page.getByRole('button', { name: 'Scan watched feeds' }).click()
  await expect(page.locator('#notice')).toContainText('You are offline. Connect, then scan again.')
})

test('@claim:demo-local demo stays same-origin and does not write real workspace data', async ({ page }) => {
  const requests: string[] = []
  page.on('request', request => requests.push(request.url()))
  await page.goto('/demo')
  await page.getByRole('button', { name: 'Acknowledge action' }).click()
  await expect(page.getByText('No actions need acknowledgement')).toBeVisible()
  const origin = new URL(page.url()).origin
  expect(requests.every(url => new URL(url).origin === origin)).toBe(true)
  expect(await page.evaluate(() => localStorage.getItem('icw:workspace'))).toBeNull()
  expect(await page.evaluate(() => localStorage.getItem('demo:integration-changelog-watch'))).not.toBeNull()
})

test('@claim:demo-isolation-transitions demo makes no API call, resets its sample, and discards it before real work starts', async ({ page }) => {
  const apiRequests: string[] = []
  page.on('request', request => {
    if (new URL(request.url()).pathname.startsWith('/api/')) apiRequests.push(request.url())
  })
  await page.goto('/demo')
  await page.getByRole('button', { name: 'Acknowledge action' }).click()
  await expect(page.getByText('No actions need acknowledgement')).toBeVisible()
  await page.getByRole('button', { name: 'Reset demo' }).click()
  await expect(page.getByRole('heading', { level: 1, name: 'Sample action cards' })).toBeVisible()
  expect(apiRequests).toEqual([])
  await page.getByRole('button', { name: 'Start a private workspace' }).click()
  await expect(page).toHaveURL(/\/$/)
  await page.waitForFunction(() => Boolean(localStorage.getItem('icw:workspace-token')))
  expect(await page.evaluate(() => localStorage.getItem('demo:integration-changelog-watch'))).toBeNull()
})

test('@claim:watch-file-portability exports, previews, and imports the CLI watch schema inside demo storage', async ({ page }) => {
  await page.goto('/demo')
  const download = page.waitForEvent('download')
  await page.getByRole('button', { name: 'Export watch file' }).click()
  const contents = await readFile(await (await download).path(), 'utf8')
  expect(JSON.parse(contents).watches).toHaveLength(3)
  const file = { name: 'watches.json', mimeType: 'application/json', buffer: Buffer.from(JSON.stringify({ watches: [{ vendor: 'Linear', url: 'https://linear.app/changelog/rss.xml', keywords: 'breaking', owner: 'Nora', version: 'linear-sdk 1.0', command: 'pnpm test:linear' }] })) }
  await page.locator('#watch-file').setInputFiles(file)
  await expect(page.getByRole('heading', { name: 'Review 1 imported watch' })).toBeVisible()
  await page.getByRole('button', { name: 'Import 1 watch' }).click()
  await expect(page.getByText('Linear', { exact: true })).toBeVisible()
  expect(await page.evaluate(() => localStorage.getItem('demo:integration-changelog-watch'))).toContain('Linear')
  expect(await page.evaluate(() => localStorage.getItem('icw:workspace'))).toBeNull()
  await page.locator('#watch-file').setInputFiles({ name: 'broken.json', mimeType: 'application/json', buffer: Buffer.from('{') })
  await expect(page.locator('#notice')).toContainText('The watch file cannot be imported.')
})
