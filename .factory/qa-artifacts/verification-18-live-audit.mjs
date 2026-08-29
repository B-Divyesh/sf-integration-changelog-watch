import { chromium } from 'playwright'
import AxeBuilder from '@axe-core/playwright'

const base = 'https://integration-changelog-watch.sociobot.in'
const browser = await chromium.launch({ headless: true })

async function audit(name, viewport, path) {
  const context = await browser.newContext({ viewport })
  const page = await context.newPage()
  const requests = []
  const errors = []
  page.on('request', request => requests.push(`${request.method()} ${request.url()}`))
  page.on('console', message => {
    if (message.type() === 'error') errors.push(`console: ${message.text()}`)
  })
  page.on('pageerror', error => errors.push(`pageerror: ${error.message}`))
  const response = await page.goto(`${base}${path}`, { waitUntil: 'networkidle', timeout: 60_000 })
  const axe = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa']).analyze()
  await page.screenshot({ path: `.factory/qa-artifacts/verification-18-${name}.png`, fullPage: true })
  return { context, page, requests, errors, response, axe }
}

const desktop = await audit('live-home-desktop', { width: 1440, height: 900 }, '/')
await desktop.page.keyboard.press('Tab')
const focused = await desktop.page.evaluate(() => {
  const element = document.activeElement
  const style = element ? getComputedStyle(element) : null
  return {
    text: element?.textContent?.trim(),
    tag: element?.tagName,
    outline: style ? `${style.outlineColor} ${style.outlineStyle} ${style.outlineWidth}` : null,
  }
})
await desktop.page.keyboard.press('Enter')
const skippedToMain = await desktop.page.evaluate(() => document.activeElement?.id)
await desktop.page.getByRole('button', { name: 'Try it with sample data' }).click()
await desktop.page.waitForURL('**/demo')
const oneClickDemo = {
  url: desktop.page.url(),
  banner: await desktop.page.getByText('Demo — sample data, nothing is saved').isVisible(),
  h1: await desktop.page.locator('h1').innerText(),
  actionTitle: await desktop.page.getByRole('heading', { name: 'Stripe retires legacy webhook event format' }).isVisible(),
  owner: await desktop.page.getByText('Maya · Payments', { exact: true }).first().isVisible(),
  version: await desktop.page.getByText('stripe-node 16.2', { exact: true }).first().isVisible(),
  check: await desktop.page.getByText('pnpm test:stripe', { exact: true }).isVisible(),
}
await desktop.context.close()

const mobile = await audit('live-demo-mobile', { width: 390, height: 844 }, '/demo')
const initialDemoRequests = [...mobile.requests]
const firstCardParts = await Promise.all([
  mobile.page.getByRole('heading', { name: 'Stripe retires legacy webhook event format' }).boundingBox(),
  mobile.page.getByText('Maya · Payments', { exact: true }).first().boundingBox(),
  mobile.page.getByText('stripe-node 16.2', { exact: true }).first().boundingBox(),
  mobile.page.getByText('pnpm test:stripe', { exact: true }).boundingBox(),
])
const storageBefore = await mobile.page.evaluate(() => Object.keys(localStorage).sort())
await mobile.page.getByRole('button', { name: 'Acknowledge action' }).first().click()
const acknowledged = await mobile.page.getByText('No actions need acknowledgement').isVisible()
await mobile.page.getByRole('button', { name: 'Reset demo' }).click()
const reset = await mobile.page.getByRole('heading', { name: 'Stripe retires legacy webhook event format' }).isVisible()
const storageAfter = await mobile.page.evaluate(() => Object.keys(localStorage).sort())
const mobileMetrics = await mobile.page.evaluate(() => ({
  clientWidth: document.documentElement.clientWidth,
  scrollWidth: document.documentElement.scrollWidth,
  visibleTargets: [...document.querySelectorAll('a,button,input')]
    .filter(element => {
      const rect = element.getBoundingClientRect()
      const style = getComputedStyle(element)
      return rect.width > 0 && rect.height > 0 && style.visibility !== 'hidden' && style.display !== 'none'
    })
    .map(element => {
      const rect = element.getBoundingClientRect()
      return { label: element.textContent?.trim() || element.getAttribute('aria-label') || element.id, width: rect.width, height: rect.height }
    }),
}))
await mobile.page.emulateMedia({ reducedMotion: 'reduce' })
const movingWithReducedMotion = await mobile.page.evaluate(() => [...document.querySelectorAll('*')].map(element => {
  const style = getComputedStyle(element)
  return { tag: element.tagName, transition: style.transitionDuration, animation: style.animationDuration, animationName: style.animationName }
}).filter(item => item.transition !== '0s' || (item.animation !== '0s' && item.animationName !== 'none')).slice(0, 20))
await mobile.context.close()

const zoom = await audit('live-demo-200pct', { width: 195, height: 844 }, '/demo')
const zoomMetrics = await zoom.page.evaluate(() => ({ clientWidth: document.documentElement.clientWidth, scrollWidth: document.documentElement.scrollWidth }))
await zoom.context.close()

const legal = await audit('live-privacy-mobile', { width: 390, height: 844 }, '/privacy')
const legalApiRequests = legal.requests.filter(request => new URL(request.split(' ')[1]).pathname.startsWith('/api/'))
await legal.context.close()

console.log(JSON.stringify({
  desktop: {
    status: desktop.response?.status(),
    axeViolations: desktop.axe.violations.map(item => ({ id: item.id, impact: item.impact, nodes: item.nodes.length })),
    errors: desktop.errors,
    focused,
    skippedToMain,
    oneClickDemo,
  },
  mobileDemo: {
    status: mobile.response?.status(),
    title: await (await browser.newPage()).title().catch(() => null),
    axeViolations: mobile.axe.violations.map(item => ({ id: item.id, impact: item.impact, nodes: item.nodes.length })),
    errors: mobile.errors,
    requests: initialDemoRequests,
    apiRequests: initialDemoRequests.filter(request => new URL(request.split(' ')[1]).pathname.startsWith('/api/')),
    allSameOrigin: initialDemoRequests.every(request => new URL(request.split(' ')[1]).origin === base),
    firstCardInFirstViewport: firstCardParts.map(box => Boolean(box && box.y >= 0 && box.y + box.height <= 844)),
    storageBefore,
    acknowledged,
    reset,
    storageAfter,
    mobileMetrics,
    movingWithReducedMotion,
  },
  zoom200: zoomMetrics,
  privacy: {
    axeViolations: legal.axe.violations.map(item => ({ id: item.id, impact: item.impact, nodes: item.nodes.length })),
    errors: legal.errors,
    apiRequests: legalApiRequests,
  },
}, null, 2))

await browser.close()
