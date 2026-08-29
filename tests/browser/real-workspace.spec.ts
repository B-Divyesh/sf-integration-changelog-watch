import { expect, test } from '@playwright/test'
import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { execFile } from 'node:child_process'
import { promisify } from 'node:util'
import { createServer } from 'node:net'

const execFileAsync = promisify(execFile)

test('real workspace renders the server action schema and acknowledges its numeric ID', async ({ page }) => {
  let acknowledged = false
  await page.route('**/api/workspaces', route => route.fulfill({ status: 201, contentType: 'application/json', body: JSON.stringify({ token: 'a'.repeat(64) }) }))
  await page.route('**/api/watches', route => route.fulfill({ contentType: 'application/json', body: JSON.stringify([{ id: 7, vendor: 'Vendor', url: 'https://vendor.example/feed', keywords: 'webhook', owner: 'Maya', version: '', command: 'npm test' }]) }))
  await page.route('**/api/actions', route => route.fulfill({ contentType: 'application/json', body: JSON.stringify([{ id: 42, watchId: 7, title: 'Webhook change', excerpt: 'A real feed notice', matched: 'webhook', url: 'https://vendor.example/notice', owner: 'Maya', version: 'vendor-sdk 4.2', command: 'npm test', acknowledged, seenAt: 'Today' }]) }))
  await page.route('**/api/actions/42', async route => {
    expect(route.request().method()).toBe('POST')
    expect(route.request().postDataJSON()).toEqual({ acknowledged: true })
    acknowledged = true
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify({}) })
  })
  await page.goto('/')
  await expect(page.getByRole('heading', { name: 'Webhook change' })).toBeVisible()
  await expect(page.getByText('vendor-sdk 4.2')).toBeVisible()
  await expect(page.getByRole('link', { name: /Open vendor notice/ })).toHaveAttribute('href', 'https://vendor.example/notice')
  await page.getByRole('button', { name: 'Acknowledge action' }).click()
  await expect.poll(() => acknowledged).toBe(true)
})

test('@claim:workspace-boundary workspace tokens isolate records, reject anonymous callers, and block private feeds', async ({ page }) => {
  await page.goto('/privacy')
  const result = await page.evaluate(async () => {
    const unauthenticated = await fetch('/api/watches')
    const first = await fetch('/api/workspaces', { method: 'POST' }).then(response => response.json()) as { token: string }
    const second = await fetch('/api/workspaces', { method: 'POST' }).then(response => response.json()) as { token: string }
    const firstWatch = await fetch('/api/watches', {
      method: 'POST',
      headers: { authorization: `Bearer ${first.token}`, 'content-type': 'application/json' },
      body: JSON.stringify({ vendor: 'Isolated', url: 'https://1.1.1.1/feed', keywords: 'webhook', owner: 'Maya', version: '', command: 'npm test' }),
    })
    const secondWatches = await fetch('/api/watches', { headers: { authorization: `Bearer ${second.token}` } })
    const privateFeed = await fetch('/api/watches', {
      method: 'POST',
      headers: { authorization: `Bearer ${first.token}`, 'content-type': 'application/json' },
      body: JSON.stringify({ vendor: 'Blocked', url: 'http://127.0.0.1/internal', keywords: 'webhook', owner: 'Maya', version: '', command: 'npm test' }),
    })
    return {
      unauthenticated: unauthenticated.status,
      firstWatch: firstWatch.status,
      privateFeed: privateFeed.status,
      secondWatches: secondWatches.status,
      secondWorkspaceWatchCount: (await secondWatches.json() as unknown[]).length,
    }
  })
  expect(result).toEqual({ unauthenticated: 401, firstWatch: 201, privateFeed: 400, secondWatches: 200, secondWorkspaceWatchCount: 0 })
})

test('a fresh workspace token stays valid for parallel authenticated reads', async ({ page }, testInfo) => {
  // This is a backend consistency probe. Running the same 24-read burst in
  // both browser projects would intentionally exceed the per-IP rate limit.
  test.skip(testInfo.project.name === 'mobile', 'The desktop project covers this one shared API burst.')
  await page.goto('/')
  await page.waitForFunction(() => Boolean(localStorage.getItem('icw:workspace-token')))
  const statuses = await page.evaluate(async () => {
    const token = localStorage.getItem('icw:workspace-token')!
    return Promise.all(Array.from({ length: 24 }, (_, index) => fetch(index % 2 ? '/api/watches' : '/api/actions', {
      headers: { authorization: `Bearer ${token}` },
    }).then(response => response.status)))
  })
  expect(statuses).toEqual(Array(24).fill(200))
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
    const cardPath = join(directory, `.integration-changelog-watch/actions/${state.actions[0].id}.md`)
    await expect(readFile(cardPath, 'utf8')).resolves.toContain('Webhook update')
    await execFileAsync('cargo', ['run', '--quiet', '--', 'ack', '--config', config, '--id', state.actions[0].id], { cwd: process.cwd() })
    const acknowledged = JSON.parse(await readFile(join(directory, '.integration-changelog-watch/state.json'), 'utf8')) as { actions: Array<{ acknowledged: boolean }> }
    expect(acknowledged.actions[0].acknowledged).toBe(true)
    const acknowledgedCard = await readFile(cardPath, 'utf8')
    expect(acknowledgedCard).toContain('**Status:** Acknowledged')
    expect(acknowledgedCard).not.toContain('**Status:** Needs acknowledgement')
  } finally {
    await rm(directory, { recursive: true, force: true })
  }
})

test('@claim:cli-demo-local CLI demo prints shipped cards without using a configured network proxy', async () => {
  let proxyConnections = 0
  const proxy = createServer(socket => {
    proxyConnections += 1
    socket.destroy()
  })
  await new Promise<void>((resolve, reject) => {
    proxy.once('error', reject)
    proxy.listen(0, '127.0.0.1', () => resolve())
  })
  const address = proxy.address()
  if (!address || typeof address === 'string') throw new Error('Test proxy did not bind a TCP port.')
  try {
    const proxyUrl = `http://127.0.0.1:${address.port}`
    const { stdout } = await execFileAsync('cargo', ['run', '--quiet', '--', 'demo'], {
      cwd: process.cwd(),
      env: { ...process.env, HTTP_PROXY: proxyUrl, HTTPS_PROXY: proxyUrl, ALL_PROXY: proxyUrl, NO_PROXY: '' },
    })
    expect(stdout).toContain('Stripe retires legacy webhook event format')
    expect(stdout).toContain('Auth0 changes refresh token rotation defaults')
    expect(proxyConnections).toBe(0)
  } finally {
    await new Promise<void>(resolve => proxy.close(() => resolve()))
  }
})

test('@claim:cli-shipped-mapping-local the shipped CLI scan mapping reads its bundled feed without network access', async () => {
  let proxyConnections = 0
  const proxy = createServer(socket => {
    proxyConnections += 1
    socket.destroy()
  })
  await new Promise<void>((resolve, reject) => {
    proxy.once('error', reject)
    proxy.listen(0, '127.0.0.1', () => resolve())
  })
  const address = proxy.address()
  if (!address || typeof address === 'string') throw new Error('Test proxy did not bind a TCP port.')
  const directory = await mkdtemp(join(tmpdir(), 'icw-shipped-cli-'))
  try {
    const examples = join(directory, 'examples')
    await mkdir(examples)
    await writeFile(join(examples, 'watches.json'), await readFile(join(process.cwd(), 'examples/watches.json')))
    await writeFile(join(examples, 'sample-feed.xml'), await readFile(join(process.cwd(), 'examples/sample-feed.xml')))
    const proxyUrl = `http://127.0.0.1:${address.port}`
    const { stdout } = await execFileAsync('cargo', ['run', '--quiet', '--', 'scan', '--config', join(examples, 'watches.json')], {
      cwd: process.cwd(),
      env: { ...process.env, HTTP_PROXY: proxyUrl, HTTPS_PROXY: proxyUrl, ALL_PROXY: proxyUrl, NO_PROXY: '' },
    })
    expect(stdout).toContain('Created')
    expect(proxyConnections).toBe(0)
    await expect(readFile(join(examples, '.integration-changelog-watch/state.json'), 'utf8')).resolves.toContain('acknowledged')
  } finally {
    await new Promise<void>(resolve => proxy.close(() => resolve()))
    await rm(directory, { recursive: true, force: true })
  }
})
