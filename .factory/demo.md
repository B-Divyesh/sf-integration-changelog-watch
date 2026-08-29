# Demo sandbox

Open `/demo`, `/?demo=1`, or click **Try it with sample data**. It opens directly on two realistic sample action cards and three public-feed watches. The persistent banner can reset the seed or discard it and begin a private workspace.

The browser stores demo state only under `demo:integration-changelog-watch`. It never reads or writes `icw:workspace`, the real dashboard key. The demo makes no API call during its standard flow.

Use **Export action cards as CSV** to exercise the export flow. Use **Export watch file** and **Import watch file** to move the shared CLI JSON schema. The sample is shipped in `frontend/src/sample.ts`.
