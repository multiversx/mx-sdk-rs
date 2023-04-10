use std::time::Duration;

use multiversx_sc_snippets::multiversx_sc::codec::test_util::top_encode_to_vec_u8_or_panic;

use super::*;

const ISSUE_COST: u64 = 50000000000000000; // 0.05 EGLD

const COLLECTION_NAME: &str = "TestCollection1";
const COLLECTION_TICKER: &str = "TESTCOLL1";
pub const COLLECTION_TOKEN_IDENTIFIER: &str = "TESTCOLL1-4096bf";
const NUM_ITEMS: usize = 3;
const ROYALTIES: usize = 3000;
const METADATA: &str = "tags:test,rust-interactor";

impl MultisigInteract {
    pub async fn issue_multisig_and_collection_full(&mut self) {
        self.deploy().await;
        self.feed_contract_egld().await;
        self.issue_collection().await;
        self.set_special_role().await;
        self.interactor.sleep(Duration::from_secs(15)).await;
        self.create_items().await;
    }

    pub async fn propose_issue_collection(&mut self) -> Option<usize> {
        let system_sc_address = bech32::decode(SYSTEM_SC_BECH32);
        let mut typed_sc_call = self
            .state
            .multisig()
            .propose_async_call(
                system_sc_address,
                ISSUE_COST,
                "issueNonFungible".to_string(),
                MultiValueVec::from([COLLECTION_NAME.to_string(), COLLECTION_TICKER.to_string()]),
            )
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit("10,000,000")
            .expect(TxExpect::ok());

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!(
                "propose issue collection failed with: {}",
                result.err().unwrap()
            );
            return None;
        }

        let action_id = result.unwrap();
        println!("successfully proposed issue colllection action `{action_id}`");
        Some(action_id)
    }

    pub async fn issue_collection(&mut self) {
        println!("proposing issue collection...");
        let action_id = self.propose_issue_collection().await;
        if action_id.is_none() {
            return;
        }

        let action_id = action_id.unwrap();
        println!("perfoming issue collection action `{action_id}`...");

        if !self.quorum_reached(action_id).await && !self.sign(action_id).await {
            return;
        }
        println!("quorum reached for action `{action_id}`");

        let mut typed_sc_call = self
            .state
            .multisig()
            .perform_action_endpoint(action_id)
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit("80,000,000");

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call
            .response()
            .issue_non_fungible_new_token_identifier();
        if result.is_err() {
            println!(
                "perform issue collection failed with: {}",
                result.err().unwrap()
            );
            return;
        }

        self.collection_token_identifier = result.unwrap();
        println!(
            "collection token identifier: {}",
            self.collection_token_identifier
        );
    }

    pub async fn propose_set_special_role(&mut self) -> Option<usize> {
        let multisig_address = self.state.multisig().to_address();
        let mut typed_sc_call = self
            .state
            .multisig()
            .propose_async_call(
                &self.system_sc_address,
                0u64,
                "setSpecialRole".to_string(),
                MultiValueVec::from([
                    self.collection_token_identifier.as_bytes(),
                    multisig_address.as_bytes(),
                    "ESDTRoleNFTCreate".as_bytes(),
                ]),
            )
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit("10,000,000");

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.result();
        if result.is_err() {
            println!(
                "propose set special role failed with: {}",
                result.err().unwrap()
            );
            return None;
        }

        let action_id = result.unwrap();
        println!("successfully proposed set special role with action `{action_id}`");
        Some(action_id)
    }

    pub async fn set_special_role(&mut self) {
        println!("proposing set special role...");
        let action_id = self.propose_set_special_role().await;
        if action_id.is_none() {
            return;
        }

        let action_id = action_id.unwrap();
        println!("performing set special role action `{action_id}`...");
        self.perform_action(action_id, "80,000,000").await;
    }

    pub async fn create_items(&mut self) {
        println!("creating items...");

        let multisig_address = self.state.multisig().to_address();
        let mut steps = Vec::new();

        for item_index in 0..NUM_ITEMS {
            let item_name = format!("Test collection item #{item_index}");
            let image_cid = format!(
                "https://ipfs.io/ipfs/QmYyAaEf1phJS5mN6wfou5de5GbpUddBxTY1VekKcjd5PC/nft{item_index:02}.jpeg"
            );

            let typed_sc_call = self
                .state
                .multisig()
                .propose_async_call(
                    &multisig_address,
                    0u64,
                    "ESDTNFTCreate".to_string(),
                    MultiValueVec::from([
                        self.collection_token_identifier.as_bytes(),
                        top_encode_to_vec_u8_or_panic(&1u32).as_slice(),
                        item_name.as_bytes(),
                        top_encode_to_vec_u8_or_panic(&ROYALTIES).as_slice(),
                        &[][..],
                        METADATA.as_bytes(),
                        image_cid.as_bytes(),
                    ]),
                )
                .into_blockchain_call()
                .from(&self.wallet_address)
                .gas_limit("10,000,000");

            steps.push(typed_sc_call);
        }

        self.interactor
            .multi_sc_exec(StepBuffer::from_sc_call_vec(&mut steps))
            .await;

        let mut actions = Vec::new();
        for step in steps.iter() {
            let result = step.result();
            if result.is_err() {
                println!(
                    "propose ESDTNFTCreate failed with: {}",
                    result.err().unwrap()
                );
                return;
            }

            let action_id = result.unwrap();
            println!("successfully proposed ESDTNFTCreate action `{action_id}`");
            actions.push(action_id);
        }

        self.perform_actions(actions, "30,000,000").await;
    }
}
