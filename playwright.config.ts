import { defineConfig, devices } from '@playwright/test'

export default defineConfig({
  testDir: './tests/browser',
  timeout: 30_000,
  forbidOnly: !!process.env.CI,
  use: {
    baseURL: process.env.PLAYWRIGHT_BASE_URL || 'http://127.0.0.1:8080',
    trace: 'retain-on-failure',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'mobile', use: { ...devices['iPhone 13'], browserName: 'chromium' } },
  ],
  webServer: {
    // The production default is the mounted /data volume. Browser tests use
    // an explicit local durable file so they never exercise an image-local
    // fallback that production must not have.
    // Build first, then replace the shell with the server process. Playwright
    // can now signal and await the process that owns port 8080 instead of
    // terminating Cargo while its spawned binary is still shutting down.
    command: 'cargo build --quiet && exec target/debug/integration-changelog-watch',
    env: { DATABASE_URL: 'sqlite:changelog-watch.db?mode=rwc' },
    url: 'http://127.0.0.1:8080/health',
    reuseExistingServer: false,
    timeout: 180_000,
    gracefulShutdown: { signal: 'SIGTERM', timeout: 10_000 },
  },
})
