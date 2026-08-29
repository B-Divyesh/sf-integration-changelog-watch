import { chromium } from 'playwright'

const browser = await chromium.launch({ headless: true })
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } })
const page = await context.newPage()
const api = []
const errors = []
page.on('response', response => {
  if (new URL(response.url()).pathname.startsWith('/api/')) api.push(`${response.request().method()} ${new URL(response.url()).pathname} ${response.status()}`)
})
page.on('console', message => { if (message.type() === 'error') errors.push(`console: ${message.text()}`) })
page.on('pageerror', error => errors.push(`pageerror: ${error.message}`))

let answers = []
page.on('dialog', async dialog => {
  if (dialog.type() === 'confirm') return dialog.accept()
  await dialog.accept(answers.shift() ?? '')
})

await page.goto('https://integration-changelog-watch.sociobot.in/', { waitUntil: 'networkidle' })

answers = ['Blocked QA watch', 'http://127.0.0.1/private', 'webhook', 'Verifier', 'sdk 1.0', 'npm test']
await page.getByRole('button', { name: 'Add a watch' }).click()
await page.waitForFunction(() => document.querySelector('#notice')?.textContent?.includes('not saved'))
const invalidMessage = await page.locator('#notice').innerText()

answers = ['QA recovery watch', 'https://1.1.1.1/feed', 'webhook', 'Verifier', 'sdk 1.0', 'npm test']
await page.getByRole('button', { name: 'Add a watch' }).click()
await page.getByRole('button', { name: 'Edit QA recovery watch' }).waitFor()
const savedMessage = await page.locator('#notice').innerText()

answers = ['QA recovery watch', 'https://1.1.1.1/feed', 'deprecation,webhook', 'Verifier', 'sdk 1.1', 'npm test:integration']
await page.getByRole('button', { name: 'Edit QA recovery watch' }).click()
await page.getByText('Keywords: deprecation,webhook').waitFor()
const editedMessage = await page.locator('#notice').innerText()

await page.getByRole('button', { name: 'Scan watched feeds' }).click()
await page.waitForFunction(() => !document.querySelector('#notice')?.textContent?.includes('Scanning public feeds'))
const scanMessage = await page.locator('#notice').innerText()
await page.screenshot({ path: '.factory/qa-artifacts/verification-18-live-workspace-recovery.png', fullPage: true })

await page.getByRole('button', { name: 'Remove QA recovery watch' }).click()
await page.getByText('Nothing is watched yet.').waitFor()
const removedMessage = await page.locator('#notice').innerText()

console.log(JSON.stringify({ invalidMessage, savedMessage, editedMessage, scanMessage, removedMessage, api, errors }, null, 2))
await browser.close()
