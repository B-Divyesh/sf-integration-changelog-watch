import { expect, test } from '@playwright/test'
import AxeBuilder from '@axe-core/playwright'

test('demo has no serious accessibility violations', async ({ page }) => {
  await page.goto('/demo')
  const results = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa']).analyze()
  expect(results.violations.filter(({ impact }) => impact === 'serious' || impact === 'critical')).toEqual([])
})

test('privacy deep link has its own heading and title', async ({ page }) => {
  await page.goto('/privacy')
  await expect(page).toHaveTitle('Privacy — Integration Changelog Watch')
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Privacy for Integration Changelog Watch')
})
