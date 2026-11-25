// Engine tests for Sadar Badge — run against the real Ootle engine
// in-process, no live network needed: `cargo test`.
//
// API verified 25 Nov 2026 against tari_template_test_tooling 0.55 +
// the OFFICIAL wasm-template test
// (github.com/tari-project/wasm-template, wasm_templates/empty/tests/test.rs):
// `TemplateTest::my_crate()`, `call_function` / `call_method`,
// `execute_expect_success` / `execute_expect_failure`, workspace buckets,
// `extract_component_value::<u64>(addr, "N")`.
//
// NOTE on state paths: the component state is CBOR-encoded as an ARRAY of
// struct fields (minicbor `#[n(N)]` indices), NOT a string-keyed map —
// so `extract_component_value` takes NUMERIC paths: field order in
// `SadarBadge` is badge_resource=0, issued_count=1, revoked_count=2,
// admin_badge_resource=3.
//
// What these tests prove: the template instantiates in the engine, a
// facilitator holding the admin badge can issue a real NFT badge and it
// verifies with the right data, revoking (burning) it makes verify fail,
// and calling issue WITHOUT the admin badge fails. They do NOT prove the
// deployer-only bootstrap gap is closed (see lib.rs: `create_admin_badge`
// is first-come-first-served).

use tari_template_test_tooling::crypto::RistrettoSecretKey;
use tari_template_test_tooling::template_lib_types::{
    ComponentAddress, NonFungibleAddress, NonFungibleId, ResourceAddress,
};
use tari_template_test_tooling::{TemplateTest, transaction::args};

struct Fixture {
    badge: ComponentAddress,
    admin_proof: NonFungibleAddress,
    facilitator: ComponentAddress,
    facilitator_secret: RistrettoSecretKey,
    learner: ComponentAddress,
    learner_secret: RistrettoSecretKey,
}

fn setup(test: &mut TemplateTest) -> Fixture {
    // `new` takes a ComponentAddressAllocation — the engine does NOT inject
    // it, the caller must pre-allocate one in the transaction and pass it
    // via the workspace (found 25 Nov 2026: first run failed with
    // "Function 'new' expected 1 arguments, but got 0").
    let template = test.get_template_address("SadarBadge");
    let result = test.execute_expect_success(
        test.transaction()
            .allocate_component_address("badge_addr")
            .call_function(template, "new", args![Workspace("badge_addr")])
            .build_and_seal(test.secret_key()),
        vec![test.owner_proof()],
    );
    let badge = result
        .finalize
        .accept()
        .expect("instantiate failed")
        .up_iter()
        .find_map(|(id, _)| id.as_component_address())
        .unwrap();
    let admin_resource: ResourceAddress = test.extract_component_value(badge, "3");

    let (facilitator, facilitator_proof, facilitator_secret) = test.create_funded_account();
    let (learner, _, learner_secret) = test.create_funded_account();

    // Mint the single admin badge (id 1) into the facilitator's vault.
    test.execute_expect_success(
        test.transaction()
            .call_function(template, "create_admin_badge", args![admin_resource])
            .put_last_instruction_output_on_workspace("admin_bucket")
            .call_method(facilitator, "deposit", args![Workspace("admin_bucket")])
            .build_and_seal(&facilitator_secret),
        vec![facilitator_proof],
    );
    let admin_proof = NonFungibleAddress::new(admin_resource, NonFungibleId::from_u64(1));

    Fixture {
        badge,
        admin_proof,
        facilitator,
        facilitator_secret,
        learner,
        learner_secret,
    }
}

fn issue_badge(test: &mut TemplateTest, fx: &Fixture) {
    test.execute_expect_success(
        test.transaction()
            .call_method(fx.badge, "issue", args!["sadar-2026-0042", "TaRL-2", "Sadar"])
            .put_last_instruction_output_on_workspace("badge_bucket")
            .call_method(fx.learner, "deposit", args![Workspace("badge_bucket")])
            .build_and_seal(&fx.facilitator_secret),
        vec![fx.admin_proof.clone()],
    );
}

#[test]
fn issue_then_verify() {
    let mut test = TemplateTest::my_crate();
    let fx = setup(&mut test);
    assert_eq!(test.extract_component_value::<u64>(fx.badge, "1"), 0);

    issue_badge(&mut test, &fx);
    assert_eq!(test.extract_component_value::<u64>(fx.badge, "1"), 1);

    // Badge id 0 = first issue (issued_count was 0 at mint time).
    let info: String = test.call_method(
        fx.badge,
        "verify",
        args![NonFungibleId::from_u64(0)],
        vec![],
    );
    assert!(info.contains("sadar-2026-0042"), "verify info: {info}");
    assert!(info.contains("TaRL-2"), "verify info: {info}");
    assert!(info.contains("VALID"), "verify info: {info}");
}

#[test]
fn revoke_invalidates_badge() {
    let mut test = TemplateTest::my_crate();
    let fx = setup(&mut test);

    // Issue and revoke in one transaction (bucket via workspace — no vault juggling).
    test.execute_expect_success(
        test.transaction()
            .call_method(fx.badge, "issue", args!["sadar-2026-0042", "TaRL-2", "Sadar"])
            .put_last_instruction_output_on_workspace("badge_bucket")
            .call_method(fx.badge, "revoke", args![Workspace("badge_bucket")])
            .build_and_seal(&fx.facilitator_secret),
        vec![fx.admin_proof.clone()],
    );
    assert_eq!(test.extract_component_value::<u64>(fx.badge, "1"), 1);
    assert_eq!(test.extract_component_value::<u64>(fx.badge, "2"), 1);

    // Burned badge must no longer verify.
    test.execute_expect_failure(
        test.transaction()
            .call_method(fx.badge, "verify", args![NonFungibleId::from_u64(0)])
            .build_and_seal(test.secret_key()),
        vec![],
    );
}

#[test]
fn non_facilitator_cannot_issue() {
    let mut test = TemplateTest::my_crate();
    let fx = setup(&mut test);

    // Learner (no admin badge) tries to issue, sealed by their own key.
    test.execute_expect_failure(
        test.transaction()
            .call_method(fx.badge, "issue", args!["sadar-2026-0042", "TaRL-2", "Sadar"])
            .build_and_seal(&fx.learner_secret),
        vec![],
    );
}
