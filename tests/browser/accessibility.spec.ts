import { expect, test } from '@playwright/test'
import AxeBuilder from '@axe-core/playwright'

test('demo has no accessibility violations', async ({ page }) => {
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
  expect(results.violations).toEqual([])
  expect(consoleErrors).toEqual([])
})

test('every app route supplies route-specific titles, descriptions, previews, and canonicals', async ({ page }) => {
  const expected = [
    ['/', 'Integration Changelog Watch — Assign vendor changes', 'Turn vendor release notes into assigned action cards with an owner, version, and local check.', '/'],
    ['/demo', 'Demo — Integration Changelog Watch', 'Explore sample vendor notices and assigned action cards. Demo changes stay separate from your private workspace.', '/demo'],
    ['/privacy', 'Privacy — Integration Changelog Watch', 'Read how Integration Changelog Watch separates demo data, workspace records, and third-party requests.', '/privacy'],
    ['/terms', 'Terms — Integration Changelog Watch', 'Read the public-source and responsible-use terms for Integration Changelog Watch.', '/terms'],
  ] as const
  for (const [path, title, description, canonical] of expected) {
    await page.goto(path)
    await expect(page).toHaveTitle(title)
    await expect(page.locator('meta[name="description"]')).toHaveAttribute('content', description)
    await expect(page.locator('meta[property="og:title"]')).toHaveAttribute('content', title)
    await expect(page.locator('meta[property="og:description"]')).toHaveAttribute('content', description)
    await expect(page.locator('meta[name="twitter:title"]')).toHaveAttribute('content', title)
    await expect(page.locator('meta[name="twitter:description"]')).toHaveAttribute('content', description)
    await expect(page.locator('link[rel="canonical"]')).toHaveAttribute('href', `https://integration-changelog-watch.sociobot.in${canonical}`)
  }
})

test('privacy deep link has its own heading and title', async ({ page }) => {
  await page.goto('/privacy')
  await expect(page).toHaveTitle('Privacy — Integration Changelog Watch')
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Privacy for Integration Changelog Watch')
})

test('legal pages do not create a workspace or make dashboard API requests', async ({ page }) => {
  for (const path of ['/privacy', '/terms']) {
    const apiRequests: string[] = []
    page.on('request', request => {
      if (new URL(request.url()).pathname.startsWith('/api/')) apiRequests.push(request.url())
    })
    await page.goto(path)
    expect(apiRequests).toEqual([])
    expect(await page.evaluate(() => localStorage.getItem('icw:workspace-token'))).toBeNull()
  }
})

test('mobile navigation and footer links meet the 44px touch target minimum', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 })
  await page.goto('/privacy')
  for (const link of [page.getByRole('link', { name: 'Demo' }), page.getByRole('link', { name: 'Terms' })]) {
    const box = await link.boundingBox()
    expect(box).not.toBeNull()
    expect(box!.width).toBeGreaterThanOrEqual(44)
    expect(box!.height).toBeGreaterThanOrEqual(44)
  }
})

test('demo reflows without horizontal scrolling at 200% equivalent width', async ({ page }) => {
  await page.setViewportSize({ width: 195, height: 844 })
  await page.goto('/demo')
  const dimensions = await page.evaluate(() => ({ viewport: document.documentElement.clientWidth, content: document.documentElement.scrollWidth }))
  expect(dimensions.content).toBeLessThanOrEqual(dimensions.viewport)
})

test('missing routes return the product-styled 404 screen', async ({ page }) => {
  const response = await page.goto('/missing-action-board')
  expect(response?.status()).toBe(404)
  await expect(page).toHaveTitle('Page not found — Integration Changelog Watch')
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('That page is not here')
  await expect(page.getByRole('link', { name: 'Return home' })).toBeVisible()
  await expect(page.locator('link[href="/404.css"]')).toHaveCount(1)
  await expect(page.getByRole('link', { name: 'How it works' })).toBeVisible()
  for (const selector of ['meta[property="og:title"]', 'meta[property="og:description"]', 'meta[name="twitter:card"]', 'meta[name="twitter:title"]', 'meta[name="twitter:description"]', 'link[rel="apple-touch-icon"]']) await expect(page.locator(selector)).toHaveCount(1)
})

test('SPA and 404 footers use the same runtime build identity as health', async ({ page }) => {
  const build = await page.request.get('/health').then(response => response.json()).then((body: { build: string }) => body.build)
  await page.goto('/demo')
  await expect(page.locator('footer')).toContainText(`build ${build}`)
  await page.goto('/missing-build-identity')
  await expect(page.locator('footer')).toContainText(`build ${build}`)
})
