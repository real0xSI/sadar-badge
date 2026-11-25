# Tari November Build Contest 2026 — MVP: Sadar Badge

> **Status: TEMPLATE FULLY IMPLEMENTED + TESTS GREEN (3 passed) — not submitted.**
> This repo is a starter prepared for the Tari November Build Contest (theme: **MVP**).
> The contest rules below are verified from official Tari sources. The project
> concept ("Sadar Badge") is a **proposal** — the Principal picks the final
> concept (see `IDEAS.md`). **CONCEPT LOCKED 25 Nov 2026 05:53 EST by
> Principal: SADAR BADGE.** The Rust template **compiles and `cargo test` is
> green (3 passed, 0 ignored — 25 Nov 2026, ~49s)**: real NFT mint/burn +
> facilitator access rules are IMPLEMENTED (drafted by Claude Opus 5 via
> FreeModel/freemodel.in on Principal's order, corrected + verified locally).
> Known gap: admin-badge bootstrap is first-come-first-served (see lib.rs).

## Contest — verified facts

| Item | Verified fact |
|---|---|
| Contest | Tari November Build Contest 2026, theme **MVP** (Minimum Viable Product) |
| Organiser | Tari Council |
| Prizes | 1st: **1,000,000 XTM** · 2nd: **500,000 XTM** · 3rd: **250,000 XTM** |
| Deadline | **00:00 UTC, 1 December 2026** = Mon 30 Nov 2026 19:00 EST = Tue 1 Dec 2026 07:00 WIB |
| Eligibility | Anyone **except** Tari Council members and Tari Labs employees. Worldwide. |
| What to build | An MVP on Tari (Ootle, Tari's L2) — something you could "go to market with today and start finding users for" |
| Submit how | **Reply to the official forum thread** with the 5 items below (no form, no separate portal) |
| Judging | Entries reviewed and voted on by the Council; winners announced the following month |
| Licence | Code must be under an **OSI-approved open source licence** (MIT recommended) — this repo uses MIT |
| AI-built code | **Explicitly allowed** (Fox, Tari Council, 2 Oct 2026); judges may consider code quality |

Sources (official, read directly 25 Nov 2026):
- Contest thread: https://community.tari.com/t/november-build-contest-thread-mvp/520
- Full rules: https://community.tari.com/t/ootle-launch-date-and-launch-contest-rules/323/1
- Ootle docs: https://ootle.tari.com/ (CLI guide, template overview)
- Aggregator listing (not official): https://devgrantsdaily.com/industry/opensource/

Full rule-by-rule notes, clarifications, and competitor scan: see [`RESEARCH.md`](RESEARCH.md).

## Submission = 1 forum reply containing 5 items

1. A publicly viewable link to the code (this repo, public on GitHub)
2. A link to the MVP app (live demo — `web/` deployed, e.g. GitHub Pages)
3. A description of the project
4. A Tari payment address to send winnings to, should you win — **NEEDS PRINCIPAL**
5. A link to a public post on social media announcing the entry — **Bluesky, Reddit, Medium, or YouTube only** (this month)

One post per submission. Update your post in place if the submission changes
(new forum users can't self-edit after ~24h — post it right, or ping the thread
owner for edit access). Full checklist: [`SUBMISSION-CHECKLIST.md`](SUBMISSION-CHECKLIST.md).

## Project concept (proposal): Sadar Badge

Verifiable literacy-achievement badges as NFTs on Tari Ootle, for the Sadar
literacy programme: a facilitator issues a badge (non-transferable NFT) to a
learner on completing a level; anyone can verify a badge publicly; badges can
be revoked if issued in error. Real users exist today (Sadar TaRL pilot), and
no entry in the contest thread does this (see `RESEARCH.md` competitor scan).

MVP scope (deliberately small — must be shippable by 30 Nov):
- **On-chain (Rust Ootle template, `template/`):** `issue` (facilitator-only mint
  of a badge NFT to a learner), `verify` (public read-only check), `revoke`
  (facilitator-only). Badge metadata: learner id (opaque, no personal data),
  level, programme, issued epoch.
- **Web app (`web/`):** walletless simulation first (judges can click through
  with no wallet/faucet — the pattern most November entries use), plus a
  clearly-labelled live-network path (Sapient + Esmeralda) if time allows.
- **Out of scope for the MVP:** payments, on-chain identity/KYC, real learner
  personal data on chain (never), mainnet publish (bonus "First to Launch"
  contest already expired 21 Nov — see `RESEARCH.md`).

If the Principal prefers a different concept, swap it in `IDEAS.md` — the
repo layout (template + web demo + docs) stays the same.

## Repo layout

```
tari-nov-build-contest/
├── README.md                  # this file — rules + concept + quickstart
├── RESEARCH.md                # verified rules, sources, competitor scan
├── SUBMISSION-CHECKLIST.md    # the 5 submission items + step-by-step
├── IDEAS.md                   # candidate MVP concepts (Sadar Badge recommended)
├── LICENSE                    # MIT (OSI-approved, as the rules require)
├── template/                  # Rust Ootle template (WASM) — DRAFT, untested
│   ├── Cargo.toml
│   ├── src/lib.rs
│   └── tests/test.rs
└── web/                       # static walletless demo (deploy to GitHub Pages)
    ├── index.html
    ├── app.js
    └── README.md
```

## Quickstart

Rust toolchain IS installed on this VM now (rustup stable, rustc 1.100.0,
wasm32-unknown-unknown target, 25 Nov 2026) — `cargo test` runs here.

```bash
# 1. Toolchain + targets (already installed on this VM)
rustup target add wasm32-unknown-unknown
cargo install tari-ootle-cli        # official Ootle CLI (ootle.tari.com/guides/cli)
#    (or: cargo install cargo-generate, then
#     cargo generate https://github.com/tari-project/wasm-template wasm_templates/empty)

# 2. Template — run the engine tests
cd template
cargo test

# 3. Build the WASM (release profile in template/Cargo.toml is size-tuned:
#    publish cost scales with binary size — ~72 KiB ≈ 0.61 TARI, ~1 MiB ≈ 89 TARI)
cargo build --target wasm32-unknown-unknown --release

# 4. Publish (needs a funded wallet; Esmeralda testnet first, mainnet only if decided)
#    From the wallet web UI ("Publish Template") or: tari publish <name>

# 5. Web demo — static files, any host (GitHub Pages is what other entries use)
#    cd web && python3 -m http.server 8000
```

Version pins in `template/Cargo.toml` (`tari_template_lib 0.42`,
`tari_template_test_tooling 0.55`) are **verified against the official
wasm-template** (github.com/tari-project/wasm-template, 25 Nov 2026).
`cargo test` result 25 Nov 2026: **2 passed, 1 ignored, 0 failed**
(ignored: `non_facilitator_cannot_issue` — access rules not implemented yet).

## What is NOT done (honest status)

- [x] Template code written — **compiles, `cargo test` green (3 passed, 0 ignored, 25 Nov 2026)**: real NFT mint/burn + facilitator access rules IMPLEMENTED (issue → verify VALID → revoke → verify fails → non-facilitator rejected, all proven by the real engine). Known gaps: admin-badge bootstrap is first-come-first-served; issued epoch not yet in badge data (TODO in lib.rs)
- [ ] Web demo written (simulation) — **not deployed, no live URL yet**
- [ ] Repo **not** pushed to GitHub (no public link exists yet)
- [ ] No Tari payment address (needs Principal's wallet address)
- [ ] No social announcement post (needs Principal's account + post)
- [ ] No forum account/reply (needs Principal — must be posted from his account)
- [ ] No mainnet/Esmeralda publish (deliberately deferred; testnet first)

See `SUBMISSION-CHECKLIST.md` for the exact handoff items.
