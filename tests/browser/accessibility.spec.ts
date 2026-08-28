import { expect, test } from '@playwright/test'
import AxeBuilder from '@axe-core/playwright'

test('demo has no serious accessibility violations', async ({ page }) => {
  const consoleErrors: string[] = []
  page.on('console', message => {
    if (message.type() === 'error') consoleErrors.push(message.text())
  })
  await page.goto('/demo')
  await expect(page.locator('html')).toHaveAttribute('lang', 'en')
  await expect(page).toHaveTitle('Demo — Integration Changelog Watch')
  await expect(page.locator('main')).toHaveCount(1)
  await expect(page.locator('img:not([alt])')).toHaveCount(0)
  const results = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa']).analyze()
  expect(results.violations.filter(({ impact }) => impact === 'serious' || impact === 'critical')).toEqual([])
  expect(consoleErrors).toEqual([])
})

test('privacy deep link has its own heading and title', async ({ page }) => {
  await page.goto('/privacy')
  await expect(page).toHaveTitle('Privacy — Integration Changelog Watch')
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Privacy for Integration Changelog Watch')
})

test('demo reflows without horizontal scrolling at 200% equivalent width', async ({ page }) => {
  await page.setViewportSize({ width: 195, height: 844 })
  await page.goto('/demo')
  const dimensions = await page.evaluate(() => ({ viewport: document.documentElement.clientWidth, content: document.documentElement.scrollWidth }))
  expect(dimensions.content).toBeLessThanOrEqual(dimensions.viewport)
})
