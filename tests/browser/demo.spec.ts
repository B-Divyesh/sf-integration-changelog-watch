import { expect, test } from '@playwright/test'
import { readFile } from 'node:fs/promises'

test('@claim:sample-action-cards opens the seeded demo workspace', async ({ page }) => {
  await page.goto('/')
  await page.getByRole('button', { name: 'Try it with sample data' }).click()
  await expect(page).toHaveURL(/\/demo$/)
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible()
  await expect(page.getByRole('heading', { name: /action needs an owner/i })).toBeVisible()
  await expect(page.getByText('Maya · Payments').first()).toBeVisible()
  await expect(page.getByText('pnpm test:stripe')).toBeVisible()
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
  await expect(page.getByText('No actions need an owner')).toBeVisible()
  await expect(page.locator('[data-action="a1"]')).toBeFocused()
})

test('offline scan gives a useful next step without making a request', async ({ page, context }) => {
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
  await expect(page.getByText('No actions need an owner')).toBeVisible()
  const origin = new URL(page.url()).origin
  expect(requests.every(url => new URL(url).origin === origin)).toBe(true)
  expect(await page.evaluate(() => localStorage.getItem('icw:workspace'))).toBeNull()
  expect(await page.evaluate(() => localStorage.getItem('demo:integration-changelog-watch'))).not.toBeNull()
})
