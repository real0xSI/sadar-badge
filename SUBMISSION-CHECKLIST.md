# Submission Checklist — Tari November Build Contest (MVP)

Deadline: **00:00 UTC, 1 Dec 2026** (= 30 Nov 19:00 EST = 1 Dec 07:00 WIB).
Submit by replying to https://community.tari.com/t/november-build-contest-thread-mvp/520
— one post per submission, containing the 5 items below.

## The 5 items the forum reply must contain

- [ ] **1. Public code link** — repo public on GitHub, OSI-approved licence (MIT).
      Status: scaffold exists locally at `~/workspace/tari-nov-build-contest/`;
      **not yet pushed to GitHub — no public link exists.**
- [ ] **2. MVP app link** — live demo URL (GitHub Pages / Vercel).
      Status: `web/` simulation written; **not deployed — no live URL yet.**
- [ ] **3. Project description** — what it is, why it's an MVP, what it does
      today. Status: draft in `README.md` (Sadar Badge concept); finalise once
      the concept is locked.
- [ ] **4. Tari payment address** — **NEEDS PRINCIPAL**: his Tari L1 XTM payout
      address (format like the `12…` addresses in other entries). Without this
      the post is incomplete and winnings can't be paid.
- [ ] **5. Social announcement link** — **NEEDS PRINCIPAL**: a public post
      announcing the entry on **Bluesky, Reddit (r/tari), Medium, or YouTube**
      (this month's four allowed platforms). Needs his account on one of them
      + the post itself. Bluesky guide:
      https://community.tari.com/t/bluesky-guide-getting-started/329

## Build steps (before the post)

- [ ] Lock the concept (Sadar Badge recommended — see `IDEAS.md`; Principal decides)
- [ ] `template/`: finish the Rust template, `cargo test` green (needs a machine
      with Rust + `wasm32-unknown-unknown` + `tari-ootle-cli`; this VM has none)
- [ ] `web/`: finish the demo, deploy to GitHub Pages, verify the live URL loads
- [ ] Push the repo to public GitHub under an OSI licence (MIT file included)
- [ ] (Optional, stronger entry) publish the template to Esmeralda testnet and
      link explorer proof; mainnet publish is NOT required and the "First to
      Launch" bonus expired 21 Nov — don't chase it
- [ ] Sanity-check the entry against the rules: new code (Nov), own code,
      OSI licence, not Council/Labs, theme = MVP

## Posting steps (Principal — cannot be done by the agent)

- [ ] **Forum account**: log in / create account on community.tari.com
      (standard Discourse signup; no XTM address needed for the account itself)
- [ ] **Post the reply** in thread 520 with items 1–5 above, in one post
- [ ] Post it **complete on the first try**: new accounts can't self-edit after
      ~24h; if an edit is needed later, ping the thread owner for edit access
- [ ] If discussing the project in detail, open a separate thread in the
      forum's **projects** category and link it from the submission post
- [ ] Keep the post updated if the submission changes (add an "updates" section)

## What the agent must NOT do

- Do not create the forum account, do not post the reply, do not use the
  Principal's email/phone for any signup, do not publish anything to mainnet,
  do not spend any funds. All of those need the Principal himself.
