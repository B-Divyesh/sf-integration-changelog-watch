import './style.css'
import { sampleActions, sampleWatches, type Action, type Watch } from './sample'
import { actionCsv } from './csv'

const app = document.querySelector<HTMLDivElement>('#app')!
const demoKey = 'demo:integration-changelog-watch'
const realKey = 'icw:workspace'
const workspaceKey = 'icw:workspace-token'
let demo = location.pathname === '/demo' || new URLSearchParams(location.search).get('demo') === '1'
let watches: Watch[] = []
let actions: Action[] = []
let active = 'home'
let statusMessage = ''
let workspacePromise: Promise<string> | undefined
const storageKey = () => (demo ? demoKey : realKey)
const escape = (value: string) => value.replace(/[&<>"']/g, char => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[char]!))

function load() {
  const saved = localStorage.getItem(storageKey())
  if (saved) {
    ;({ watches, actions } = JSON.parse(saved) as { watches: Watch[]; actions: Action[] })
  } else if (demo) {
    watches = structuredClone(sampleWatches)
    actions = structuredClone(sampleActions)
  } else {
    watches = []
    actions = []
  }
}

function save() {
  localStorage.setItem(storageKey(), JSON.stringify({ watches, actions }))
}

async function ensureWorkspace() {
  let token = localStorage.getItem(workspaceKey)
  if (token) return token
  workspacePromise ??= (async () => {
    const response = await fetch('/api/workspaces', { method: 'POST' })
    if (!response.ok) throw new Error('Could not create a private workspace.')
    const created = (await response.json() as { token: string }).token
    localStorage.setItem(workspaceKey, created)
    return created
  })()
  try {
    return await workspacePromise
  } catch (error) {
    workspacePromise = undefined
    throw error
  }
}

async function api(path: string, init: RequestInit = {}) {
  const token = await ensureWorkspace()
  const headers = new Headers(init.headers)
  headers.set('Authorization', `Bearer ${token}`)
  return fetch(path, { ...init, headers })
}

async function hydrateReal() {
  if (demo) return
  try {
    const [watchResponse, actionResponse] = await Promise.all([api('/api/watches'), api('/api/actions')])
    if (!watchResponse.ok || !actionResponse.ok) throw new Error('Could not load this workspace.')
    watches = await watchResponse.json() as Watch[]
    actions = await actionResponse.json() as Action[]
    save()
    render()
  } catch {
    statusMessage = 'Your private workspace could not load. Check your connection, then reload.'
    const notice = document.querySelector<HTMLElement>('#notice')
    if (notice) notice.textContent = statusMessage
  }
}

function actionCard(action: Action) {
  return `<article class="action ${action.acknowledged ? 'done' : ''}" data-action="${action.id}"><div class="action-top"><p class="eyebrow">Matched “${escape(action.matched)}” · ${escape(action.seenAt)}</p><span class="status">${action.acknowledged ? 'Acknowledged' : 'Needs owner'}</span></div><h3>${escape(action.title)}</h3><p>${escape(action.excerpt)}</p><dl><div><dt>Owner</dt><dd>${escape(action.owner)}</dd></div><div><dt>Check</dt><dd><code>${escape(action.command)}</code></dd></div></dl><div class="card-actions"><a href="${escape(action.url)}" target="_blank" rel="noreferrer">Open vendor notice <span class="sr-only">(opens in a new tab)</span></a>${action.acknowledged ? '' : `<button class="secondary ack" data-id="${action.id}">Acknowledge action</button>`}</div></article>`
}

function dashboard() {
  const pending = actions.filter(action => !action.acknowledged)
  return `<section class="dashboard" aria-labelledby="action-heading"><div class="dash-head"><div><p class="eyebrow">Your owned queue</p><h2 id="action-heading">${pending.length ? `${pending.length} action${pending.length === 1 ? ' needs' : 's need'} an owner` : 'No actions need an owner'}</h2></div><div class="dash-buttons"><button id="scan" class="primary" ${watches.length ? '' : 'disabled'}>Scan watched feeds</button><button id="add-watch" class="secondary">Add a watch</button></div></div><p id="notice" class="notice" role="status" aria-live="polite">${escape(statusMessage)}</p><div class="dash-grid"><section><h3>Action cards</h3><div class="action-list">${actions.length ? actions.map(actionCard).join('') : `<div class="empty"><h3>No action cards yet</h3><p>Add a feed, rule, owner, and check command. Matched release notes will appear here.</p><button class="primary" id="empty-add">Add your first watch</button></div>`}</div></section><aside class="watches" aria-label="Watched feeds"><div class="side-title"><h3>Watched feeds</h3><span>${watches.length}/3</span></div>${watches.length ? watches.map(watch => `<article class="watch"><strong>${escape(watch.vendor)}</strong><span>${escape(watch.owner)}</span><small>${escape(watch.keywords)}</small><span class="watch-controls"><button class="text-button edit-watch" data-watch="${watch.id}">Edit ${escape(watch.vendor)}</button><button class="text-button delete-watch" data-watch="${watch.id}">Remove ${escape(watch.vendor)}</button></span></article>`).join('') : '<p>Nothing is watched yet.</p>'}<button id="export" class="text-button" ${actions.length ? '' : 'disabled'}>Export action cards as CSV</button></aside></div></section>`
}

function home() {
  return `<section class="hero"><div class="hero-copy"><p class="kicker">INTEGRATION CHANGELOG WATCH</p><h1 tabindex="-1">Turn vendor changes into owned actions</h1><p class="lede">For engineers who maintain payment, auth, analytics, or messaging integrations.</p><div class="hero-actions"><button class="primary" id="try-demo">Try it with sample data</button><span>See matched notices, owners, and checks.</span></div><ul class="facts"><li>Rules are written by your team.</li><li>Scans run only when you request them.</li><li>Your workspace is separated from other visitors.</li></ul></div><figure><img src="/paper-cut-hero.webp" width="1536" height="1024" fetchpriority="high" decoding="async" alt="Paper release-note cards travel into a small action card."><figcaption>Original paper-cut illustration.</figcaption></figure></section>${dashboard()}<section class="how" id="how"><p class="eyebrow">How it works</p><h2>Give each vendor change a next step</h2><ol><li><strong>Watch a public feed.</strong><span>Paste a changelog or RSS address you are allowed to read.</span></li><li><strong>Match your words.</strong><span>Use rules like “webhook”, “deprecation”, or an API version.</span></li><li><strong>Run the right check.</strong><span>Each matching notice gets an owner and a check command.</span></li></ol></section><section class="limits"><h2>What this does not do</h2><p>It does not read private portals, alter code, or detect undocumented changes.</p><p>Private, loopback, and link-local addresses are blocked.</p></section>`
}

function legal(kind: 'privacy' | 'terms') {
  const privacy = kind === 'privacy'
  document.title = `${privacy ? 'Privacy' : 'Terms'} — Integration Changelog Watch`
  return `<article class="legal"><h1 tabindex="-1">${privacy ? 'Privacy for Integration Changelog Watch' : 'Terms for Integration Changelog Watch'}</h1><p>${privacy ? 'Demo data stays in separate browser storage. Real workspaces use a random browser-held token and are not visible to other workspace tokens.' : 'Use only public changelog and RSS addresses that you are allowed to read. You are responsible for your matching rules and follow-up work.'}</p><h2>${privacy ? 'Data handling' : 'Public sources'}</h2><p>${privacy ? 'No analytics, advertising scripts, or third-party fonts run here. The server stores watches and action cards only inside the workspace token you create.' : 'Private, loopback, link-local, and redirecting source addresses are blocked to protect the service.'}</p><p><a href="/">Return home</a></p></article>`
}

function render() {
  document.title = active === 'privacy' ? 'Privacy — Integration Changelog Watch' : active === 'terms' ? 'Terms — Integration Changelog Watch' : demo ? 'Demo — Integration Changelog Watch' : 'Integration Changelog Watch — Track vendor changes'
  document.querySelector<HTMLLinkElement>('link[rel="canonical"]')!.href = `https://integration-changelog-watch.sociobot.in${location.pathname}`
  const pageName = active === 'privacy' ? 'Privacy' : active === 'terms' ? 'Terms' : demo ? 'Demo' : 'Integration Changelog Watch'
  app.innerHTML = `<a class="skip" href="#main">Skip to content</a><header><a class="wordmark" href="/" data-route="home"><span aria-hidden="true">▰</span> Changelog Watch</a><nav aria-label="Main navigation"><a href="/demo" data-route="demo">Demo</a><a href="/#how">How it works</a><a href="/privacy" data-route="privacy">Privacy</a></nav></header>${demo ? `<aside class="demo-banner">Demo — sample data, nothing is saved <span><button id="reset-demo">Reset demo</button><button id="start-real">Start for real</button></span></aside>` : ''}<main id="main" tabindex="-1">${active === 'privacy' || active === 'terms' ? legal(active) : home()}</main><p class="sr-only" aria-live="polite" aria-atomic="true">${pageName}</p><footer><p>Vendor notices become owned integration actions.</p><p><a href="/privacy" data-route="privacy">Privacy</a> · <a href="/terms" data-route="terms">Terms</a> · Built by Param Factory · v2</p></footer>`
  bind()
}

function route(path: string, moveFocus = false) {
  active = path.includes('privacy') ? 'privacy' : path.includes('terms') ? 'terms' : 'home'
  demo = path === '/demo' || new URLSearchParams(location.search).get('demo') === '1'
  load()
  render()
  // Legal pages are informational routes. They must not create a workspace or
  // make dashboard API requests merely because someone reads their terms.
  if (!demo && active === 'home') void hydrateReal()
  if (moveFocus) document.querySelector<HTMLElement>('h1')?.focus({ preventScroll: true })
}

function navigate(path: string) {
  history.pushState({}, '', path)
  route(path, true)
}

async function addWatch() {
  await saveWatch()
}

async function saveWatch(existing?: Watch) {
  const vendor = prompt('Vendor name', existing?.vendor || '')
  if (!vendor) return
  const url = prompt('Public RSS or changelog URL', existing?.url || '')
  if (!url) return
  const keywords = prompt('Keywords, separated by commas', existing?.keywords || 'breaking,deprecation') || ''
  const owner = prompt('Owner', existing?.owner || 'Integration owner') || ''
  const command = prompt('Local check command', existing?.command || 'npm test') || ''
  const watch: Watch = { id: existing?.id || crypto.randomUUID(), vendor, url, keywords, owner, version: existing?.version || '', command }
  if (!demo) {
    try {
      const response = await api(existing ? `/api/watches/${existing.id}` : '/api/watches', { method: existing ? 'PUT' : 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify(watch) })
      if (!response.ok) throw new Error(await response.text())
      statusMessage = existing ? `Updated ${vendor}. Scan it when you are ready.` : `Saved ${vendor}. Scan it when you are ready.`
      await hydrateReal()
      return
    } catch (error) {
      statusMessage = `This watch was not saved. ${error instanceof Error ? error.message : 'Try again.'}`
      render()
      return
    }
  }
  if (existing) watches = watches.map(item => String(item.id) === String(existing.id) ? watch : item)
  else watches.push(watch)
  save()
  render()
}

async function removeWatch(watch: Watch) {
  if (!confirm(`Remove ${watch.vendor}? Its action cards will also be removed.`)) return
  if (demo) {
    watches = watches.filter(item => String(item.id) !== String(watch.id))
    actions = actions.filter(item => String(item.watchId) !== String(watch.id))
    statusMessage = `Removed ${watch.vendor} from this sample.`
    save()
    render()
    return
  }
  try {
    const response = await api(`/api/watches/${watch.id}`, { method: 'DELETE' })
    if (!response.ok) throw new Error(await response.text())
    statusMessage = `Removed ${watch.vendor}. You can add another watch.`
    await hydrateReal()
  } catch (error) {
    statusMessage = `This watch was not removed. ${error instanceof Error ? error.message : 'Try again.'}`
    render()
  }
}

function csv() {
  const anchor = document.createElement('a')
  anchor.href = URL.createObjectURL(new Blob([actionCsv(actions)], { type: 'text/csv' }))
  anchor.download = 'integration-actions.csv'
  anchor.click()
  URL.revokeObjectURL(anchor.href)
}

function bind() {
  document.querySelectorAll<HTMLElement>('[data-route]').forEach(link => link.addEventListener('click', event => {
    event.preventDefault()
    navigate((link as HTMLAnchorElement).getAttribute('href')!)
  }))
  document.querySelector('#try-demo')?.addEventListener('click', () => navigate('/demo'))
  document.querySelector('#add-watch, #empty-add')?.addEventListener('click', () => void addWatch())
  document.querySelectorAll<HTMLButtonElement>('.edit-watch').forEach(button => button.addEventListener('click', () => {
    const watch = watches.find(item => String(item.id) === button.dataset.watch)
    if (watch) void saveWatch(watch)
  }))
  document.querySelectorAll<HTMLButtonElement>('.delete-watch').forEach(button => button.addEventListener('click', () => {
    const watch = watches.find(item => String(item.id) === button.dataset.watch)
    if (watch) void removeWatch(watch)
  }))
  document.querySelector('#reset-demo')?.addEventListener('click', () => { localStorage.removeItem(demoKey); load(); render() })
  document.querySelector('#start-real')?.addEventListener('click', () => { localStorage.removeItem(demoKey); navigate('/') })
  document.querySelectorAll<HTMLButtonElement>('.ack').forEach(button => button.addEventListener('click', async () => {
    const action = actions.find(item => String(item.id) === button.dataset.id)
    if (!action) return
    if (!demo) {
      const response = await api(`/api/actions/${action.id}`, { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ acknowledged: true }) })
      if (!response.ok) { statusMessage = 'The action was not acknowledged. Reload and try again.'; render(); return }
      await hydrateReal()
    } else {
      action.acknowledged = true
      save()
      render()
    }
    const updatedCard = document.querySelector<HTMLElement>(`[data-action="${String(action.id)}"]`)
    if (updatedCard) { updatedCard.tabIndex = -1; updatedCard.focus() }
  }))
  document.querySelector('#export')?.addEventListener('click', csv)
  document.querySelector('#scan')?.addEventListener('click', async () => {
    const notice = document.querySelector<HTMLElement>('#notice')!
    statusMessage = navigator.onLine ? 'Scanning public feeds…' : 'You are offline. Connect, then scan again.'
    notice.textContent = statusMessage
    if (!navigator.onLine) return
    if (demo) { setTimeout(() => { statusMessage = 'No new matched notices were found. Your existing action cards remain.'; notice.textContent = statusMessage }, 450); return }
    try {
      const response = await api('/api/scan', { method: 'POST' })
      const result = await response.json() as { message?: string; failures?: string[] }
      statusMessage = result.failures?.length ? `${result.message} ${result.failures.join(' ')}` : result.message || 'The scan did not finish. Check the feed address, then try again.'
      notice.textContent = statusMessage
      await hydrateReal()
    } catch {
      statusMessage = 'The scan did not finish. Check the feed address, then try again.'
      notice.textContent = statusMessage
    }
  })
}

window.addEventListener('popstate', () => route(location.pathname, true))
route(location.pathname)
