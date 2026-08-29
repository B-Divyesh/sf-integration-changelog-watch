import { spawn } from 'node:child_process'
import { readFile } from 'node:fs/promises'
import { createServer } from 'node:net'

const claims = JSON.parse(await readFile(new URL('../.factory/claims.json', import.meta.url), 'utf8'))

function run(command) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, { cwd: process.cwd(), shell: true, stdio: 'inherit' })
    child.once('error', reject)
    child.once('exit', (code, signal) => {
      if (code === 0) resolve()
      else reject(new Error(`Claim command exited with ${code ?? signal}: ${command}`))
    })
  })
}

function assertClaimPortReleased() {
  return new Promise((resolve, reject) => {
    const server = createServer()
    server.unref()
    server.once('error', error => reject(new Error(`Claim command leaked port 8080: ${error.message}`)))
    server.listen(8080, '127.0.0.1', () => server.close(error => error ? reject(error) : resolve()))
  })
}

for (const claim of claims) {
  console.log(`\n[claim:${claim.id}] ${claim.test}`)
  await assertClaimPortReleased()
  await run(claim.test)
  await assertClaimPortReleased()
}

console.log(`\nAll ${claims.length} literal claim commands passed without leaking port 8080.`)
