const shaPattern = /^[0-9a-f]{40}$/

function assertFullSha(value, label) {
  if (!shaPattern.test(value)) throw new Error(`${label} must be a full 40-character lowercase Git SHA.`)
}

export function assertPublishedCommit(expected, published) {
  assertFullSha(expected, 'Expected build SHA')
  assertFullSha(published, 'Published main SHA')
  if (expected !== published) {
    throw new Error(`Refusing to deploy ${expected}: origin/main is ${published}. Push the exact commit first.`)
  }
}

export function assertLiveIdentity(expected, healthBody, html) {
  assertFullSha(expected, 'Expected build SHA')
  let health
  try {
    health = JSON.parse(healthBody)
  } catch {
    throw new Error('Live /health did not return JSON.')
  }
  if (health?.ok !== true || health.build !== expected) {
    throw new Error(`Live /health build did not match ${expected}.`)
  }
  if (!html.includes(`data-build="${expected}"`)) {
    throw new Error(`Live HTML data-build marker did not match ${expected}.`)
  }
}

async function main() {
  const [mode, expected, published] = process.argv.slice(2)
  if (mode === 'published') {
    assertPublishedCommit(expected, published)
    console.log(`Published source identity confirmed: ${expected}`)
    return
  }
  if (mode === 'live') {
    const input = await new Promise(resolve => {
      let body = ''
      process.stdin.setEncoding('utf8')
      process.stdin.on('data', chunk => { body += chunk })
      process.stdin.on('end', () => resolve(body))
    })
    const boundary = input.indexOf('\n')
    if (boundary < 0) throw new Error('Pass /health JSON followed by a newline and the live HTML.')
    assertLiveIdentity(expected, input.slice(0, boundary), input.slice(boundary + 1))
    console.log(`Live deployment identity confirmed: ${expected}`)
    return
  }
  throw new Error('Usage: release-identity.mjs published <expected-sha> <origin-main-sha> | live <expected-sha>')
}

const invoked = process.argv[1] && import.meta.url === new URL(`file://${process.argv[1]}`).href
if (invoked) {
  main().catch(error => {
    console.error(error.message)
    process.exitCode = 1
  })
}
