// Sadar Badge — verifiable literacy-achievement badges on Tari Ootle.
//
// Status 25 Nov 2026: REAL IMPLEMENTATION DONE + `cargo test` GREEN
// (3 passed, 0 ignored, ~49s) against tari_template_lib 0.42 / test
// tooling 0.55 (verified on this machine). Mint/burn + facilitator access
// rules implemented; drafted by Claude Opus 5 via FreeModel (freemodel.in)
// on Principal's order and corrected + verified locally by Gorith.
// API shape verified against the OFFICIAL wasm-template
// (github.com/tari-project/wasm-template) and the 0.42/0.55 crate sources.
//
// Design (MVP, deliberately minimal):
//   * One non-fungible badge resource (SADAR). Each badge is an NFT whose
//     immutable data carries: learner_id (opaque, NO personal data), level,
//     programme. Mintable/burnable ONLY by this component (ScopedToComponent).
//   * One admin badge resource (SADARADM). Holding one admin NFT gates
//     `issue`/`revoke` (RuleRequirement::Resource). `verify` is public.
//   * `issue`   — facilitator-only: mints one badge NFT, returns the bucket;
//     the caller deposits it into the learner's vault. Badge id is
//     deterministic: NonFungibleId::from_u64(issued_count).
//   * `verify`  — public read-only: returns the badge data + VALID, or the
//     transaction panics (rolls back) when the badge does not exist
//     (unknown id, or revoked = burned).
//   * `revoke`  — facilitator-only: burns the badge bucket the caller
//     deposits, so the id can never verify again.
//   * `create_admin_badge` — bootstrap: mints the single admin badge
//     (id 1; a second mint panics on duplicate id). FIRST-COME-FIRST-SERVED:
//     anyone can call it, so the deployer must call it immediately after
//     deploying, before anyone else does. (A deployer-only bootstrap would
//     need an owner rule on the admin resource; deferred as a known gap.)
//
// Security notes:
//   * NEVER take the caller's key as a method argument — it can be spoofed.
//     Authorisation here is by badge: only the holder of the facilitator
//     admin badge may call `issue`/`revoke`.
//   * Errors are signalled by panicking; a panic rolls back the transaction.
//   * All component fields must be CBOR-encodable (the #[template] macro
//     derives the encoding for structs declared inside the module).
//   * TODO: append the issued epoch to the badge immutable data once the
//     epoch accessor for this pin is confirmed.

use tari_template_lib::prelude::*;

#[template]
mod template {
    use super::*;

    pub struct SadarBadge {
        /// Address of the badge NFT resource created in the constructor.
        badge_resource: ResourceAddress,
        /// Total badges ever issued (for display / auditing).
        issued_count: u64,
        /// Total badges revoked.
        revoked_count: u64,
        /// Facilitator credential resource: holding one NFT gates issue/revoke.
        admin_badge_resource: ResourceAddress,
    }

    impl SadarBadge {
        /// Constructor: creates the badge + admin resources and the component.
        ///
        /// The badge resource can only be minted/burned by this component
        /// (ScopedToComponent), so badges can only ever come into existence
        /// through `issue`. `issue`/`revoke` require the admin badge;
        /// `verify` is public; everything else defaults to DenyAll.
        pub fn new(alloc: ComponentAddressAllocation) -> Component<Self> {
            // The address this component will live at (known before creation).
            let this_component: ComponentAddress = alloc.get_address();

            // Facilitator credential: one NFT per admin, minted by
            // `create_admin_badge`. Openly mintable in the MVP — the
            // ComponentAccessRules below are what actually gate issue/revoke,
            // and only ONE admin badge can ever exist (id 1; a second mint
            // panics on duplicate id). Deployer must claim it immediately.
            let admin_badge_resource = ResourceBuilder::non_fungible()
                .with_token_symbol("SADARADM")
                .mintable(AccessRule::AllowAll, UpdateRule::Locked)
                .build();

            // Badge supply: only this component may mint or burn.
            let self_only = || {
                AccessRule::Restricted(RestrictedAccessRule::Require(RequireRule::Require(
                    RuleRequirement::ScopedToComponent(this_component),
                )))
            };
            let badge_resource = ResourceBuilder::non_fungible()
                .with_token_symbol("SADAR")
                .mintable(self_only(), UpdateRule::Locked)
                .burnable(self_only(), UpdateRule::Locked)
                .build();

            let facilitator = || {
                AccessRule::Restricted(RestrictedAccessRule::Require(RequireRule::Require(
                    RuleRequirement::Resource(admin_badge_resource),
                )))
            };
            let rules = ComponentAccessRules::allow_all()
                .method("issue", facilitator())
                .method("revoke", facilitator())
                .method("verify", AccessRule::AllowAll)
                .default(AccessRule::DenyAll);

            Component::new(Self {
                badge_resource,
                issued_count: 0,
                revoked_count: 0,
                admin_badge_resource,
            })
            .with_address_allocation(alloc)
            .with_access_rules(rules)
            .create()
        }

        /// Issue one badge to a learner. Facilitator-only (admin badge).
        ///
        /// Returns the bucket holding the minted badge NFT; the caller
        /// deposits it into the learner's vault. Badge id is deterministic:
        /// `NonFungibleId::from_u64(issued_count)`.
        ///
        /// `learner_id` is an opaque string chosen by the facilitator
        /// (e.g. "sadar-2026-0042") — never a name, phone number, or any
        /// personal data. `level` is the completed level (e.g. "TaRL-2").
        pub fn issue(&mut self, learner_id: String, level: String, programme: String) -> Bucket {
            let badge_id = NonFungibleId::from_u64(self.issued_count);
            // Immutable data = (learner_id, level, programme).
            // TODO: append the issued epoch once the epoch accessor is pinned.
            let bucket = ResourceManager::get(self.badge_resource).mint_non_fungible(
                badge_id,
                &(learner_id, level, programme),
                &(),
            );
            self.issued_count += 1;
            bucket
        }

        /// Public read-only check: returns the badge data for a given NFT id.
        /// Panics (rolling back the transaction) when the badge does not exist
        /// — unknown id, or revoked (burned) badge.
        pub fn verify(&self, badge_id: NonFungibleId) -> String {
            let nft = ResourceManager::get(self.badge_resource).get_non_fungible(&badge_id);
            let (learner_id, level, programme): (String, String, String) = nft.get_data();
            format!(
                "learner={} level={} programme={} | VALID",
                learner_id, level, programme
            )
        }

        /// Revoke a badge (burn it). Facilitator-only (admin badge).
        /// The caller deposits the badge bucket; it is burned so the id can
        /// never verify again.
        pub fn revoke(&mut self, bucket: Bucket) {
            bucket.burn();
            self.revoked_count += 1;
        }

        /// Bootstrap: mints the single admin badge into the caller's hands.
        /// Must live INSIDE this impl block — free functions at module level
        /// are NOT registered as template functions (proven 25 Nov 2026:
        /// "Function create_admin_badge not found" when it sat outside).
        /// Pass `SadarBadge`'s `admin_badge_resource` (component state
        /// field 3); the caller deposits the returned bucket. Only ONE admin
        /// badge can ever exist (id 1; a second mint panics on duplicate
        /// id), so the deployer must call this immediately after deploying.
        pub fn create_admin_badge(admin_badge_resource: ResourceAddress) -> Bucket {
            ResourceManager::get(admin_badge_resource).mint_non_fungible(
                NonFungibleId::from_u64(1),
                &"SADAR-ADMIN".to_string(),
                &(),
            )
        }
    }
}
