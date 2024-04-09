use std::time::Duration;

use multiversx_sc_snippets::imports::*;

use super::*;

const ISSUE_COST: u64 = 50000000000000000; // 0.05 EGLD

const COLLECTION_NAME: &str = "TestCollection1";
const COLLECTION_TICKER: &str = "TESTCOLL1";
const TOKEN_TYPE: &str = "NFT";

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

    pub async fn issue_multisig_and_collection_with_all_roles_full(&mut self) {
        self.deploy().await;
        self.feed_contract_egld().await;
        self.issue_collection_with_all_roles().await;
        self.interactor.sleep(Duration::from_secs(15)).await;
        self.create_items().await;
    }

    pub async fn propose_issue_collection_with_all_roles(&mut self) -> usize {
        let action_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_multisig_address())
            .gas(NumExpr("10,000,000"))
            .typed(multisig_proxy::MultisigProxy)
            .propose_async_call(
                ESDTSystemSCAddress,
                ISSUE_COST,
                FunctionCall::new("registerAndSetAllRoles")
                    .argument(&COLLECTION_NAME)
                    .argument(&COLLECTION_TICKER)
                    .argument(&TOKEN_TYPE)
                    .argument(&0u32),
            )
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await;

        println!("successfully proposed issue colllection with roles all action `{action_id}`");
        action_id
    }

    pub async fn issue_collection_with_all_roles(&mut self) {
        println!("proposing issue collection with all roles...");
        let action_id = self.propose_issue_collection_with_all_roles().await;

        println!("perfoming issue collection with all roles action `{action_id}`...");

        self.sign_if_quorum_not_reached(action_id).await;

        let new_token_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_multisig_address())
            .gas(NumExpr("80,000,000"))
            .typed(multisig_proxy::MultisigProxy)
            .perform_action_endpoint(action_id)
            .returns(ReturnsNewTokenIdentifier)
            .prepare_async()
            .run()
            .await;
        self.collection_token_identifier = new_token_id.to_string();

        println!(
            "collection token identifier: {}",
            self.collection_token_identifier
        );
    }

    pub async fn propose_issue_collection(&mut self) -> usize {
        let action_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_multisig_address())
            .gas(NumExpr("10,000,000"))
            .typed(multisig_proxy::MultisigProxy)
            .propose_async_call(
                ESDTSystemSCAddress,
                ISSUE_COST,
                FunctionCall::new("issueNonFungible")
                    .argument(&COLLECTION_NAME)
                    .argument(&COLLECTION_TICKER),
            )
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await;

        println!("successfully proposed issue colllection action `{action_id}`");
        action_id
    }

    pub async fn issue_collection(&mut self) {
        println!("proposing issue collection...");
        let action_id = self.propose_issue_collection().await;

        println!("perfoming issue collection action `{action_id}`...");

        self.sign_if_quorum_not_reached(action_id).await;

        let new_token_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_multisig_address())
            .gas(NumExpr("80,000,000"))
            .typed(multisig_proxy::MultisigProxy)
            .perform_action_endpoint(action_id)
            .returns(ReturnsNewTokenIdentifier)
            .prepare_async()
            .run()
            .await;
        self.collection_token_identifier = new_token_id;

        println!(
            "collection token identifier: {}",
            self.collection_token_identifier
        );
    }

    pub async fn propose_set_special_role(&mut self) -> usize {
        let multisig_address = self.state.current_multisig_address();
        let action_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_multisig_address())
            .gas(NumExpr("10,000,000"))
            .typed(multisig_proxy::MultisigProxy)
            .propose_async_call(
                ESDTSystemSCAddress,
                0u64,
                FunctionCall::new("setSpecialRole")
                    .argument(&self.collection_token_identifier)
                    .argument(multisig_address)
                    .argument(&"ESDTRoleNFTCreate"),
            )
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await;

        println!("successfully proposed set special role with action `{action_id}`");
        action_id
    }

    pub async fn set_special_role(&mut self) {
        println!("proposing set special role...");
        let action_id = self.propose_set_special_role().await;

        println!("performing set special role action `{action_id}`...");
        self.perform_action(action_id, 80_000_000u64).await;
    }

    pub async fn create_items(&mut self) {
        println!("creating items...");

        let mut buffer = self.interactor.homogenous_call_buffer();
        let multisig_address = self.state.current_multisig_address();
        for item_index in 0..NUM_ITEMS {
            let item_name = format!("Test collection item #{item_index}");
            let image_cid = format!(
                "https://ipfs.io/ipfs/QmYyAaEf1phJS5mN6wfou5de5GbpUddBxTY1VekKcjd5PC/nft{item_index:02}.png"
            );

            buffer.push_tx(|tx| {
                tx.from(&self.wallet_address)
                    .to(multisig_address)
                    .gas(10_000_000u64)
                    .typed(multisig_proxy::MultisigProxy)
                    .propose_async_call(
                        multisig_address,
                        0u64,
                        FunctionCall::new("ESDTNFTCreate")
                            .argument(&self.collection_token_identifier)
                            .argument(&1u32)
                            .argument(&item_name)
                            .argument(&ROYALTIES)
                            .argument(&Empty)
                            .argument(&METADATA)
                            .argument(&image_cid),
                    )
                    .returns(ReturnsResult)
            });
        }

        let action_ids = buffer.run().await;
        for action_id in action_ids.iter() {
            println!("successfully proposed ESDTNFTCreate action `{action_id}`");
        }

        self.perform_actions(action_ids, 30_000_000u64).await;
    }
}
