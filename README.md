# Sadar Badge — web demo

Static, walletless **simulation** of the Sadar Badge template. No wallet, no
faucet, no network calls: the page mirrors the template's rules in JavaScript
so judges can click through issue → verify → revoke with zero setup — the
same pattern most November entries use for their judge path.

## Run locally

```bash
python3 -m http.server 8000   # then open http://localhost:8000
```

## Deploy (for the submission's "MVP app" link)

Any static host. GitHub Pages is the usual choice for contest entries:

1. Push this repo to public GitHub.
2. Repo → Settings → Pages → deploy from `/web` (or publish `web/` to a
   `gh-pages` branch).
3. Use the resulting `https://<user>.github.io/<repo>/` URL as the app link
   in the forum submission post.

## What is simulated vs live

- **Simulated (this page):** issuing, verifying and revoking badges; the
  rules enforced are the same as the template's (facilitator-only issue /
  revoke, public verify, opaque learner ids, no personal data).
- **Not implemented:** a live path against Esmeralda testnet via the Sapient
  wallet (`window.tari`). That is a bonus if time allows before the cutoff —
  it is NOT required for the contest, and this page is honestly labelled as
  a simulation (do not claim a live deployment you don't have).

## Files

- `index.html` — the demo page (labelled SIMULATION throughout).
- `app.js` — the in-browser simulation of the template's state machine.
