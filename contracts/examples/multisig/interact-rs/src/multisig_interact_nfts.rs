use super::*;

const ISSUE_COST: u64 = 50000000000000000; // 0.05 EGLD

impl State {
    pub(crate) async fn propose_issue_collection(&mut self) -> usize {
        let system_sc_address = bech32::decode(SYSTEM_SC_BECH32);
        self.interactor
            .sc_call_get_result(
                self.multisig
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
            .await
    }

    pub(crate) async fn issue_collection(&mut self) {
        let action_id = self.propose_issue_collection().await;
        let tx_hash = self.perform_action(action_id, "80,000,000").await;
        println!("perform issue collection tx hash: {}", tx_hash);
    }

    pub(crate) async fn propose_set_special_role(&mut self) -> usize {
        let multisig_address = self.multisig.to_address();
        self.interactor
            .sc_call_get_result(
                self.multisig
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
                    .gas_limit("10,000,000")
                    .expect(TxExpect::ok()),
            )
            .await
    }

    pub(crate) async fn set_special_role(&mut self) {
        let action_id = self.propose_set_special_role().await;
        let tx_hash = self.perform_action(action_id, "80,000,000").await;
        println!("perform issue collection tx hash: {}", tx_hash);
    }
}
