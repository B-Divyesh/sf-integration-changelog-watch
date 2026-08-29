# Copy audit

All visitor-facing landing sentences are below 22 words. No banned marketing terms appear.

| Location | Words | Copy | Status |
| --- | ---: | --- | --- |
| Hero heading | 7 | Turn vendor changes into assigned action cards | Pass |
| Hero audience | 10 | For engineers who maintain payment, authentication, analytics, or messaging integrations. | Pass |
| Hero action result | 7 | See matched notices, owners, versions, and checks. | Pass |
| Hero fact | 5 | You choose the matching keywords. | Tested: `keyword-edit` |
| Hero fact | 7 | Scans run only when you request them. | Tested: `requested-scans` |
| Hero fact | 7 | Your workspace is separated from other visitors. | Tested: `workspace-boundary` |
| Empty action state | 9 | Matched release notes appear here after you scan a feed. | Tested: `hosted-scan-result` |
| How it works | 8 | Give each vendor change a next step | Pass |
| How it works | 11 | Paste a changelog or RSS address you are allowed to read. | Pass |
| How it works | 10 | Use keywords like “webhook”, “deprecation”, or an API version. | Pass |
| How it works | 11 | Each matching notice includes an owner, dependency version, and check command. | Pass |
| Scope | 8 | The hosted workspace holds up to three watches. | Tested: `hosted-watch-limit` |
| Scope | 10 | Use the local CLI for a four-watch mapping. | Tested: `cli-more-feeds` |
| Limitation | 5 | It does not scan automatically. | Tested: `requested-scans` |
| Limitation | 7 | Private, loopback, and link-local addresses are blocked. | Tested: `workspace-boundary` |
| Footer | 6 | Vendor notices become assigned action cards. | Pass |
| Demo banner | 6 | Demo — sample data, nothing is saved | Tested: `demo-local` |
| Demo banner note | 3 | Discards this demo. | Tested: `demo-isolation-transitions` |
| Watch-file preview | 8 | A rejected import leaves your current watches unchanged. | Tested: `watch-file-rejection-preserves-watches` |

## Terminology

| Concept | One term |
| --- | --- |
| Vendor source | feed |
| Saved monitor | watch |
| Match input | keywords |
| Output | action card |
| Assignee | owner |
| Isolated hosted data | workspace |
| Local tool | CLI |

Catalog description: “Turn vendor changes into assigned action cards with owners, versions, and checks.” (12 words)
