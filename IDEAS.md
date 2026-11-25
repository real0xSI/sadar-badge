# MVP Concept Candidates — Tari November Build Contest

Theme: **MVP** — "something you could go to market with today and start
finding users for." Judged subjectively by the Council; innovate, don't reskin.
Reuse of previous rounds' open-source code is allowed this month, but minor
changes on existing projects score poorly. AI-agent-built code is allowed.

## Recommended: (1) Sadar Badge — verifiable literacy-achievement badges

- **What:** a facilitator issues a non-transferable achievement badge (NFT) on
  Tari Ootle to a learner who completes a literacy level; anyone can verify a
  badge from its public page; the facilitator can revoke a badge issued in
  error. Learner identity on chain is an opaque id only — no personal data.
- **Why it wins the "MVP" test:** real users exist today (Sadar TaRL pilot);
  a facilitator could use it in a real class tomorrow. No entry in the thread
  does education/credential badges (competitors: voting, memberships, tip
  jars, ROSCA, escrow, games, signal desks, grant ledgers — see RESEARCH.md).
- **Why it's buildable in days:** one template (issue / verify / revoke),
  one resource (badge NFT), one static web demo. Walletless simulation is
  the default judge path; live Esmeralda/Sapient path is a bonus.
- **Risks:** badge/credential concepts are a known Web3 trope — the write-up
  must show the real Sadar users and the real workflow, not just the tech.
- **Status:** scaffolded as the default in this repo (`template/`, `web/`).

## Alternative: (2) WarungPay — pre-order + escrow for warung sellers

- **What:** a buyer pre-orders from a small warung seller; funds sit in the
  template vault and release to the seller on delivery confirmation (or refund
  on cancellation). Fits the Principal's marketplace-selling work.
- **Why not first choice:** escrow is already represented in the thread
  (BountyVault — bug-bounty escrow), so differentiation is harder; the
  "clever riff" would have to be the warung/pre-order workflow itself.
- **Keep as fallback** if Sadar Badge is rejected.

## Alternative: (3) KelasKoin — learn-to-earn quiz + badge

- **What:** short literacy quizzes; passing mints a badge and releases a small
  TARI reward from a sponsor-funded pool. Extends concept (1) with incentives.
- **Why not first choice:** strictly bigger scope than (1) (adds a reward pool,
  sponsor funding, anti-cheat questions) — risky in 5 days. Could be the
  "v2" of Sadar Badge after the contest, or a December-theme entry.
- **Keep as follow-up**, not for this deadline.

## Decision needed from the Principal

Pick **(1) Sadar Badge** (recommended), (2), (3), or name a different MVP.
Everything in `template/` and `web/` is written for (1) and would need a
rewrite for (2)/(3) — the repo layout and submission mechanics stay the same.
