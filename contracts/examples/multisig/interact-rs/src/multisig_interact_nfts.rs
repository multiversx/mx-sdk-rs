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
        let result = self
            .interactor
            .sc_call_get_result(
                self.state
                    .multisig()
                    .propose_async_call(
                        system_sc_address,
                        ISSUE_COST,
                        "issueNonFungible".to_string(),
                        MultiValueVec::from([
                            COLLECTION_NAME.to_string(),
                            COLLECTION_TICKER.to_string(),
                        ]),
                    )
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .gas_limit("10,000,000")
                    .expect(TxExpect::ok()),
            )
            .await;

        let result = result.value();
        if result.is_err() {
            println!("propose issue collection failed: {}", result.err().unwrap());
            return None;
        }
        Some(result.unwrap())
    }

    pub async fn issue_collection(&mut self) {
        let action_id = self.propose_issue_collection().await;
        if action_id.is_none() {
            return;
        }

        let action_id = action_id.unwrap();
        println!("propose issue collection: {action_id}");

        let step = self.perform_action_step(action_id, "80,000,000");
        let raw_result = self.interactor.sc_call_get_raw_result(step).await;
        let result = raw_result.issue_non_fungible_new_token_identifier();
        if result.is_err() {
            println!("perform issue collection failed: {}", result.err().unwrap());
            return;
        }

        self.collection_token_identifier = result.unwrap();
        println!(
            "perform issue collection: {}; collection token identifier: {}",
            action_id, self.collection_token_identifier
        );
    }

    pub async fn propose_set_special_role(&mut self) -> Option<usize> {
        let multisig_address = self.state.multisig().to_address();
        let result = self
            .interactor
            .sc_call_get_result(
                self.state
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
                    .gas_limit("10,000,000"),
            )
            .await;

        let result = result.value();
        if result.is_err() {
            println!("propose set special role failed: {}", result.err().unwrap());
            return None;
        }
        Some(result.unwrap())
    }

    pub async fn set_special_role(&mut self) {
        let action_id = self.propose_set_special_role().await;
        if action_id.is_none() {
            return;
        }

        let action_id = action_id.unwrap();
        println!("propose set special role: {action_id}");

        self.perform_action(action_id, "80,000,000").await;
        println!("perform set special role: {action_id}");
    }

    pub async fn create_items(&mut self) {
        let mut last_index = self.get_action_last_index().await;
        let multisig_address = self.state.multisig().to_address();

        let mut steps = Vec::<ScCallStep>::new();
        for item_index in 0..NUM_ITEMS {
            let item_name = format!("Test collection item #{item_index}");
            let image_cid = format!(
                "https://ipfs.io/ipfs/QmYyAaEf1phJS5mN6wfou5de5GbpUddBxTY1VekKcjd5PC/nft{item_index:02}.jpeg"
            );

            steps.push(
                self.state
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
                    .gas_limit("10,000,000")
                    .into(),
            );
        }

        for _ in 0..NUM_ITEMS {
            last_index += 1;
            steps.push(self.perform_action_step(last_index, "30,000,000"));
        }

        self.interactor.multiple_sc_calls(steps.as_slice()).await;
    }
}
