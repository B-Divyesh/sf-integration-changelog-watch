# Demo sandbox

Open `/demo` or click **Try it with sample data**. It seeds three public-feed watches and two realistic action cards. The persistent banner can reset the seed or discard it and begin a real workspace.

The browser stores demo state only under `demo:integration-changelog-watch`. It never reads or writes `icw:workspace`, the real dashboard key. The demo makes no API call during its standard flow.

Use **Export action cards as CSV** to exercise the export flow. The sample is shipped in `frontend/src/sample.ts`.
