# Copy audit

All visitor-facing landing sentences are below 22 words. No banned marketing terms appear.

| Location | Words | Copy | Status |
| --- | ---: | --- | --- |
| Hero heading | 7 | Turn vendor changes into assigned action cards | Pass |
| Hero audience | 10 | For engineers who maintain payment, authentication, analytics, or messaging integrations. | Pass |
| Hero action result | 7 | See matched notices, owners, versions, and checks. | Tested: `sample-action-cards` |
| Hero fact | 5 | Scanning public feeds requires internet. | Tested: `online-feed-scans` |
| Hero fact | 6 | No account or payment is required. | Tested: `no-account-or-payment` |
| Hero fact | 7 | Your workspace is separated from other visitors. | Tested: `workspace-boundary` |
| Empty action state | 10 | Matched release notes appear here after you scan a feed. | Tested: `hosted-scan-result` |
| How it works | 8 | Give each vendor change a next step | Pass |
| How it works | 11 | Paste a changelog or RSS address you are allowed to read. | Pass |
| How it works | 10 | Use keywords like “webhook”, “deprecation”, or an API version. | Pass |
| How it works | 11 | Each matching notice includes an owner, dependency version, and check command. | Tested: `hosted-scan-result` |
| Hosted workspace limits | 8 | The hosted workspace holds up to three watches. | Tested: `hosted-watch-limit` |
| Hosted workspace limits | 8 | Use the local CLI for a four-watch mapping. | Tested: `cli-more-feeds` |
| Scheduled scans | 12 | Turn on a schedule for any watch when you want automatic scans. | Tested: `scheduled-scan-consent` |
| Scheduled scans | 10 | Scheduled watches show the last run, next run, and errors. | Tested: `scheduled-run-status` |
| Scheduled scans | 10 | Add an optional public webhook destination for run summaries. | Tested: `scheduled-notification-destination` |
| Source safeguards | 7 | Private, loopback, and link-local addresses are blocked. | Tested: `workspace-boundary` |
| Footer | 6 | Vendor notices become assigned action cards. | Tested: `hosted-scan-result` |
| Demo banner | 6 | Demo — sample data, nothing is saved | Tested: `demo-local` |
| Demo banner note | 3 | Discards this demo. | Tested: `demo-isolation-transitions` |
| Schedule state | 4 | Scheduled scans are off. | Tested: `scheduled-scan-consent` |
| Schedule state | 4 | Scheduled every 60 minutes. | Tested: `scheduled-scan-consent` |
| Schedule failure | 9 | Last run error: Could not reach this public feed. | Tested: `scheduled-run-status` |
| Watch-file preview | 8 | A rejected import leaves your current watches unchanged. | Tested: `watch-file-rejection-preserves-watches` |

## README check

The README uses short sentences for setup, API, scheduling, and deployment. The longest deployment sentence has 20 words. Every product claim in the README is mapped in `.factory/claims.json`.

## Terminology

| Concept | One term |
| --- | --- |
| Vendor source | feed |
| Saved monitor | watch |
| Match input | keywords |
| Output | action card |
| Assignee | owner |
| Isolated hosted data | workspace |
| Repeating opt-in work | schedule |
| Local tool | CLI |
