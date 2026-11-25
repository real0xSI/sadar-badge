# Tari November Build Contest 2026 — Research Notes

Researched 25 Nov 2026 (5 days before the deadline) by direct reads of the
official Tari Community forum and Ootle docs. Aggregator pages (DevGrants
Daily, AlphaGrowth, teapawt.net) were used only as pointers; every rule below
is from an official Tari source.

## 1. Official sources (read directly)

| # | Source | URL |
|---|---|---|
| 1 | Contest thread (OP, 1 Nov 2026 13:23) — "November Build Contest Thread: MVP!" | https://community.tari.com/t/november-build-contest-thread-mvp/520 |
| 2 | Full contest rules (OP, 1 Sep 2026 17:41) — "Ootle Launch Date and Launch Contest Rules" | https://community.tari.com/t/ootle-launch-date-and-launch-contest-rules/323/1 |
| 3 | Ootle docs — CLI guide | https://ootle.tari.com/guides/cli/ |
| 4 | Ootle docs — Templates overview | https://ootle.tari.com/guides/template-overview/ |
| 5 | `tari_template_lib` crate docs | https://docs.rs/tari_template_lib/latest/tari_template_lib/ |
| 6 | Bluesky guide (linked from the contest thread) | https://community.tari.com/t/bluesky-guide-getting-started/329 |

## 2. Rules — full text summary (rules page, 1 Sep 2026)

The Ootle launched 11 Nov 2026. The Council runs one contest per month, each
with its own theme. Entries are judged on adherence to the theme and subject
to these rules:

1. **New code:** all entries must be new code as of the announcement of the
   theme (September was the only exception, allowing existing projects).
2. **Authorship:** all code must be written by you (or your team); may not
   borrow code from other submissions.
3. **Licence:** all code under an **OSI-approved open source licence**
   (MIT, Apache, AGPL…; "if you're not sure which to use, go with MIT").
4. **Ineligible:** Tari Council members and Tari Labs employees.
5. **Submission format:** posted to the forum thread created by the Council,
   and must include:
   1. a publicly viewable link to the code and/or app,
   2. a description of the project,
   3. a Tari payment address to send winnings to, should you win,
   4. a link to a public post on social media announcing the contest entry.
6. **Judging/prizes:** entries reviewed and voted on by the Council; winners
   announced the following month. 1st: **1,000,000 XTM** · 2nd: **500,000 XTM** ·
   3rd: **250,000 XTM**.
7. **Cutoff:** the start of the first day of the following month, UTC. For
   November: **00:00 UTC, 1 December 2026**.

## 3. November theme — MVP (contest thread, 1 Nov 2026)

- Theme: **MVP** — "a Minimum Viable Product is something you could go to
  market with today and start finding users for."
- **Reuse allowed this month:** you may build on any code submitted by any
  contestant in previous rounds (including your own) — it's open source, so
  leverage it. BUT the submission must **innovate** on whatever you borrow;
  "minor changes atop existing projects will not get the same treatment as
  unique and clever riffs and savvy combinations. The judgement here is likely
  to get more subjective, so fair warning!"
- Submission reply must have **5 items** (the thread version is stricter than
  the rules page — it splits code and app into separate links, and restricts
  the social platform):
  1. publicly viewable link to the code,
  2. link to the MVP app,
  3. description of the project,
  4. Tari payment address,
  5. link to a public announcement post on **Bluesky** (guide: source 6),
     **Reddit** (r/tari), **Medium**, or **YouTube** — this month only these four.
- **One post per submission.** If you update the submission, update your post
  and add a section about what's updated. Note: newer submitters often don't
  have privileges to self-edit after ~24h — "ping me for access" (thread
  owner) if you can't update.
- Don't discuss submissions in the thread — start a new thread in the
  **projects** category of the forum and link it in your post.
- Help while developing: new thread in **development**, or the development
  channel in the Tari Discord (invite link in the thread).

## 4. Official clarifications (Fox, Tari Council, 2 Oct 2026, rules thread)

- September was the only month for pre-existing projects. For later months,
  only projects built as of the announcement post count. **But:** "if you
  submitted a newly created smart contract used by your project, or a library
  consumed by your project that others can use, then those would count. The
  key is it should be a new repo with new code."
- **Source code must be available under an OSI-approved licence.** A working
  public app alone, without public code, is not enough.
- **Development using coding agents is fine.** "Very few in this community
  are hand-rolling code, and we don't expect that of them. However, do note
  that judges may consider code quality in their evaluations."
- Reddit: "It is in the tradition of Reddit to make Throwaways. Your
  submission will be accepted to /r/tari."

## 5. Deadline — verified by date arithmetic

- Cutoff: **00:00 UTC, Tuesday 1 December 2026**
- = **Monday 30 November 2026, 19:00 EST** (America/Toronto)
- = **Tuesday 1 December 2026, 07:00 WIB** (Asia/Jakarta)
- Note the board's "30 Nov 2026" label is the same cutoff expressed as a date;
  the authoritative wording is "00:00 UTC on 1 December 2026"
  (DevGrants Daily's entry text agrees: "Entries close at 00:00 UTC on
  1 December 2026").

## 6. Eligibility — verified

- Worldwide; anyone except Tari Council members and Tari Labs employees.
- No age, team-size, or prior-participation restriction is stated anywhere in
  the official rules. (Do **not** assume an 18+ rule — it isn't in the rules;
  other aggregators' generic wording does not apply here.)
- **No registration form, no fee, no KYC mentioned.** The only "registration"
  is a forum account on community.tari.com so you can reply to the thread.
  The submission itself is the forum reply.

## 7. Mapping: the separate "registrasi forum Tari" checklist item

- The contest **requires submitting via the Tari forum** (reply to thread
  /t/november-build-contest-thread-mvp/520). So yes — a community.tari.com
  account is a hard prerequisite, and it must exist before the cutoff.
- Creating the forum account itself does **not** need an XTM address (it's a
  standard Discourse signup). The **XTM address is needed inside the
  submission post** (item 4 of 5) — a Tari L1 payment address (entries show
  the `12…` address format). So the Principal must supply his Tari payment
  address before the post is written; the forum account can be created
  independently of it.
- Practical gotcha from the thread OP: new accounts lose self-edit rights
  after ~24h, so the submission post should be complete and correct on the
  first post, or the thread owner must be pinged for edit access.

## 8. Related contest — November Bonus "First to Launch" (EXPIRED, ignore)

- Separate bonus thread: https://community.tari.com/t/november-bonus-contest-first-to-launch/521
- +100k XTM each for the first 20 people to publish a contract on Ootle
  mainnet; cutoff was **11:11:11 UTC, 21 November 2026** — already passed.
- Do not chase it. Mainnet publish is NOT required for the MVP contest
  itself (most entries ship a walletless simulation + optional Esmeralda
  testnet evidence).

## 9. What other entries look like (competitor scan, thread pp. 1–2)

By 17 Nov 2026 the thread already had **25+ replies**. Observed patterns:

| Archetype | Examples seen |
|---|---|
| Silent voting / sealed ballots | QuietTicket |
| Paid memberships / passes | Ootle Pass |
| Tip jars for OSS repos | Ootip |
| Rotating savings circles (ROSCA) | Ajo on Ootle, Arisan Circle |
| Bug-bounty escrow | BountyVault |
| Sealed trading calls / prediction | Signal Desk (builds on Sept+Oct entries) |
| Collectible crafting game | Ootle Forge |
| Public-grant ledger | Public Grant Ledger |

Common winning shape: **Rust Ootle `#[template]`** (`tari_template_lib`
0.37–0.40, `tari_template_test_tooling` 0.49, engine tests green) +
**walletless browser simulation** as the default judge path (GitHub Pages /
Vercel, no wallet/faucet needed) + optional live Esmeralda/Sapient evidence +
MIT licence + Medium/Bluesky announcement. Several entries were explicitly
built by AI agents, which the rules allow.

**Gap used for our concept:** no entry does verifiable achievement/credential
badges for education or literacy programmes — that is the Sadar Badge angle
(see `IDEAS.md`). It is also the Principal's own domain (Sadar), so the
"could go to market today and find users" test is genuinely met.

## 10. Technical grounding (Ootle docs + entries)

- Ootle = Tari L2; templates are Rust crates compiled to WASM
  (`tari_template_lib`, `#[template]` macro; first `pub struct` = component,
  fields = persisted CBOR state; errors by panicking; never take the caller's
  key as a method argument).
- Scaffold: `cargo install tari-ootle-cli`, then `tari create <name>`,
  `tari add <template>`, `tari publish <name>`; or `cargo-generate` from
  https://github.com/tari-project/wasm-template. Examples:
  https://github.com/tari-project/wasm-examples.
- Tests: `tari_template_test_tooling` runs transactions through the real
  engine in-process (`cargo test`, no live network needed).
- Publish cost scales with WASM size (~72 KiB ≈ 0.61 TARI; ~1 MiB ≈ 89 TARI);
  release profile `opt-level='s', lto=true, codegen-units=1, panic='abort',
  strip=true`. Publish via wallet web UI or `tari publish` (wallet-daemon
  JSON-RPC); wallet daemon runs `wasm-opt`.
- Explorer: Veil (explorer.tari.mw); testnet: Esmeralda; wallet: Sapient
  (browser extension, `window.tari`).
- **Unverified / to confirm at build time:** exact current crate versions
  (`tari_template_lib`, `tari_template_test_tooling`) on docs.rs; exact
  `ResourceBuilder::non_fungible` / access-rule API used in `template/` —
  the template code in this repo is a draft and has **not** been compiled
  (no Rust toolchain on this machine).
