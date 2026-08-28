import { expect, test } from '@playwright/test'
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { execFile } from 'node:child_process'
import { promisify } from 'node:util'

const execFileAsync = promisify(execFile)

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

test('@claim:requested-scans runs a real workspace scan only after the owner requests it', async ({ page }) => {
  let scans = 0
  await page.route('**/api/workspaces', route => route.fulfill({ status: 201, contentType: 'application/json', body: JSON.stringify({ token: 'b'.repeat(64) }) }))
  await page.route('**/api/watches', route => route.fulfill({ contentType: 'application/json', body: JSON.stringify([{ id: 7, vendor: 'Vendor', url: 'https://vendor.example/feed', keywords: 'webhook', owner: 'Maya', version: '', command: 'npm test' }]) }))
  await page.route('**/api/actions', route => route.fulfill({ contentType: 'application/json', body: '[]' }))
  await page.route('**/api/scan', route => { scans += 1; return route.fulfill({ contentType: 'application/json', body: JSON.stringify({ message: 'Scan complete. 0 new action card(s).', failures: [] }) }) })
  await page.goto('/')
  expect(scans).toBe(0)
  await page.getByRole('button', { name: 'Scan watched feeds' }).click()
  await expect.poll(() => scans).toBe(1)
  await expect(page.locator('#notice')).toContainText('Scan complete. 0 new action card(s).')
})

test('scan errors remain visible after the real workspace refreshes', async ({ page }) => {
  await page.route('**/api/workspaces', route => route.fulfill({ status: 201, contentType: 'application/json', body: JSON.stringify({ token: 'c'.repeat(64) }) }))
  await page.route('**/api/watches', route => route.fulfill({ contentType: 'application/json', body: JSON.stringify([{ id: 7, vendor: 'Missing Feed', url: 'https://vendor.example/missing', keywords: 'webhook', owner: 'Maya', version: '', command: 'npm test' }]) }))
  await page.route('**/api/actions', route => route.fulfill({ contentType: 'application/json', body: '[]' }))
  await page.route('**/api/scan', route => route.fulfill({ contentType: 'application/json', body: JSON.stringify({ message: 'Scan finished with 1 feed error(s). Fix the listed address and scan again.', failures: ['Missing Feed: The feed returned an error response.'] }) }))
  await page.goto('/')
  await page.getByRole('button', { name: 'Scan watched feeds' }).click()
  await expect(page.locator('#notice')).toContainText('Missing Feed: The feed returned an error response.')
})

test('a full workspace exposes edit and remove recovery controls', async ({ page }) => {
  const watches = [1, 2, 3].map(id => ({ id, vendor: `Vendor ${id}`, url: `https://vendor.example/${id}`, keywords: 'webhook', owner: 'Maya', version: '', command: 'npm test' }))
  let removed = false
  await page.route('**/api/workspaces', route => route.fulfill({ status: 201, contentType: 'application/json', body: JSON.stringify({ token: 'd'.repeat(64) }) }))
  await page.route('**/api/watches', route => route.fulfill({ contentType: 'application/json', body: JSON.stringify(removed ? watches.slice(1) : watches) }))
  await page.route('**/api/actions', route => route.fulfill({ contentType: 'application/json', body: '[]' }))
  await page.route('**/api/watches/1', async route => {
    expect(route.request().method()).toBe('DELETE')
    removed = true
    await route.fulfill({ status: 204 })
  })
  await page.goto('/')
  await expect(page.getByRole('button', { name: 'Edit Vendor 1' })).toBeVisible()
  page.once('dialog', dialog => dialog.accept())
  await page.getByRole('button', { name: 'Remove Vendor 1' }).click()
  await expect.poll(() => removed).toBe(true)
  await expect(page.getByText('Removed Vendor 1. You can add another watch.')).toBeVisible()
})

test('a cold real dashboard creates one workspace even while it loads watches and actions together', async ({ page }) => {
  let creates = 0
  await page.route('**/api/workspaces', route => { creates += 1; return route.fulfill({ status: 201, contentType: 'application/json', body: JSON.stringify({ token: 'e'.repeat(64) }) }) })
  await page.route('**/api/watches', route => route.fulfill({ contentType: 'application/json', body: '[]' }))
  await page.route('**/api/actions', route => route.fulfill({ contentType: 'application/json', body: '[]' }))
  await page.goto('/')
  await expect.poll(() => creates).toBe(1)
})

test('@claim:cli-repository-workflow stores hashes, action cards, and acknowledgements beside a shipped mapping', async () => {
  const directory = await mkdtemp(join(tmpdir(), 'icw-cli-'))
  const feed = join(directory, 'feed.xml')
  const config = join(directory, 'watches.json')
  await writeFile(feed, '<rss><channel><item><title>Webhook update</title><description>Webhook migration notice</description><link>https://example.com/notice</link></item></channel></rss>')
  await writeFile(config, JSON.stringify({ watches: [{ vendor: 'Fixture', url: 'feed.xml', keywords: 'webhook', owner: 'Maya', version: '', command: 'npm test' }] }))
  try {
    await execFileAsync('cargo', ['run', '--quiet', '--', 'scan', '--config', config], { cwd: process.cwd() })
    const state = JSON.parse(await readFile(join(directory, '.integration-changelog-watch/state.json'), 'utf8')) as { actions: Array<{ id: string, acknowledged: boolean }> }
    expect(state.actions).toHaveLength(1)
    await expect(readFile(join(directory, `.integration-changelog-watch/actions/${state.actions[0].id}.md`), 'utf8')).resolves.toContain('Webhook update')
    await execFileAsync('cargo', ['run', '--quiet', '--', 'ack', '--config', config, '--id', state.actions[0].id], { cwd: process.cwd() })
    const acknowledged = JSON.parse(await readFile(join(directory, '.integration-changelog-watch/state.json'), 'utf8')) as { actions: Array<{ acknowledged: boolean }> }
    expect(acknowledged.actions[0].acknowledged).toBe(true)
  } finally {
    await rm(directory, { recursive: true, force: true })
  }
})
