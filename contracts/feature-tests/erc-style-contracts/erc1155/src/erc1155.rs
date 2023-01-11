#![no_std]
#![allow(clippy::type_complexity)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod erc1155_user_proxy;

#[multiversx_sc::contract]
pub trait Erc1155 {
    #[init]
    fn init(&self) {}

    // endpoints

    /// `value` is amount for fungible, nft_id for non-fungible
    #[endpoint(safeTransferFrom)]
    fn safe_transfer_from(
        &self,
        from: ManagedAddress,
        to: ManagedAddress,
        type_id: BigUint,
        value: BigUint,
        data: &ManagedBuffer,
    ) {
        let caller = self.blockchain().get_caller();

        require!(!to.is_zero(), "Can't transfer to address zero");
        require!(self.is_valid_type_id(&type_id), "Token id is invalid");
        require!(
            caller == from || self.is_approved(&caller, &from).get(),
            "Caller is not approved to transfer tokens from address"
        );

        if self.is_fungible(&type_id).get() {
            self.safe_transfer_from_fungible(from, to, type_id, value, data)
        } else {
            self.safe_transfer_from_non_fungible(from, to, type_id, value, data)
        }

        // self.transfer_single_event(&caller, &from, &to, &id, &amount);
    }

    fn safe_transfer_from_fungible(
        &self,
        from: ManagedAddress,
        to: ManagedAddress,
        type_id: BigUint,
        amount: BigUint,
        data: &ManagedBuffer,
    ) {
        self.try_reserve_fungible(&from, &type_id, &amount);

        if self.blockchain().is_smart_contract(&to) {
            self.peform_async_call_single_transfer(from, to, type_id, amount, data);
        } else {
            self.increase_balance(&to, &type_id, &amount);
        }
    }

    fn safe_transfer_from_non_fungible(
        &self,
        from: ManagedAddress,
        to: ManagedAddress,
        type_id: BigUint,
        nft_id: BigUint,
        data: &ManagedBuffer,
    ) {
        self.try_reserve_non_fungible(&from, &type_id, &nft_id);

        if self.blockchain().is_smart_contract(&to) {
            self.peform_async_call_single_transfer(from, to, type_id, nft_id, data);
        } else {
            let amount = BigUint::from(1u32);
            self.increase_balance(&to, &type_id, &amount);
            self.token_owner(&type_id, &nft_id).set(&to);
        }
    }

    /// `value` is amount for fungible, nft_id for non-fungible
    #[endpoint(safeBatchTransferFrom)]
    fn safe_batch_transfer_from(
        &self,
        from: ManagedAddress,
        to: ManagedAddress,
        type_ids: &[BigUint],
        values: &[BigUint],
        data: ManagedBuffer,
    ) {
        let caller = self.blockchain().get_caller();
        let is_receiver_smart_contract = self.blockchain().is_smart_contract(&to);

        require!(
            caller == from || self.is_approved(&caller, &from).get(),
            "Caller is not approved to transfer tokens from address"
        );
        require!(!to.is_zero(), "Can't transfer to address zero");
        require!(
            !type_ids.is_empty() && !values.is_empty(),
            "No type_ids and/or values provided"
        );
        require!(
            type_ids.len() == values.len(),
            "Id and value lenghts do not match"
        );

        // storage edits are rolled back in case of SCError,
        // so the reverting is handled automatically if one of the transfers fails
        for (type_id, value) in type_ids.iter().zip(values.iter()) {
            if self.is_fungible(type_id).get() {
                self.safe_batch_item_transfer_from_fungible(
                    is_receiver_smart_contract,
                    &from,
                    &to,
                    type_id,
                    value,
                );
            } else {
                self.safe_batch_item_transfer_from_non_fungible(
                    is_receiver_smart_contract,
                    &from,
                    &to,
                    type_id,
                    value,
                );
            }
        }

        if is_receiver_smart_contract {
            self.peform_async_call_batch_transfer(from, to, type_ids, values, &data);
        }
    }

    fn safe_batch_item_transfer_from_fungible(
        &self,
        is_receiver_smart_contract: bool,
        from: &ManagedAddress,
        to: &ManagedAddress,
        type_id: &BigUint,
        amount: &BigUint,
    ) {
        self.try_reserve_fungible(from, type_id, amount);
        if !is_receiver_smart_contract {
            self.increase_balance(to, type_id, amount);
        }
    }

    fn safe_batch_item_transfer_from_non_fungible(
        &self,
        is_receiver_smart_contract: bool,
        from: &ManagedAddress,
        to: &ManagedAddress,
        type_id: &BigUint,
        nft_id: &BigUint,
    ) {
        self.try_reserve_non_fungible(from, type_id, nft_id);
        if !is_receiver_smart_contract {
            let amount = BigUint::from(1u32);
            self.increase_balance(to, type_id, &amount);
            self.token_owner(type_id, nft_id).set(to);
        } else {
            self.token_owner(type_id, nft_id)
                .set(&ManagedAddress::zero());
        }
    }

    #[endpoint(setApprovalForAll)]
    fn set_approved_for_all(&self, operator: ManagedAddress, approved: bool) {
        let caller = self.blockchain().get_caller();

        self.is_approved(&operator, &caller).set(approved);
    }

    // returns assigned id
    #[endpoint(createToken)]
    fn create_token(
        &self,
        uri: &BoxedBytes,
        initial_supply: BigUint,
        is_fungible: bool,
    ) -> BigUint {
        let big_uint_one = BigUint::from(1u32);

        let creator = self.blockchain().get_caller();
        let type_id = &self.last_valid_type_id().get() + &big_uint_one;

        self.set_balance(&creator, &type_id, &initial_supply);
        self.token_type_creator(&type_id).set(&creator);
        self.is_fungible(&type_id).set(is_fungible);

        if !is_fungible {
            self.set_owner_for_range(&type_id, &big_uint_one, &initial_supply, &creator);
            self.last_valid_nft_id_for_type(&type_id)
                .set(&initial_supply);
        }

        self.last_valid_type_id().set(&type_id);
        self.token_type_uri(&type_id).set(uri);

        type_id
    }

    #[endpoint]
    fn mint(&self, type_id: BigUint, amount: BigUint) {
        let creator = self.token_type_creator(&type_id).get();

        require!(
            self.blockchain().get_caller() == creator,
            "Only the token creator may mint more tokens"
        );

        self.increase_balance(&creator, &type_id, &amount);

        if !self.is_fungible(&type_id).get() {
            let last_valid_id = self.last_valid_nft_id_for_type(&type_id).get();
            let id_first = &last_valid_id + 1u32;
            let id_last = last_valid_id + amount;

            self.set_owner_for_range(&type_id, &id_first, &id_last, &creator);

            self.last_valid_nft_id_for_type(&type_id).set(&id_last);
        }

        // self.transfer_single_event(&caller, &from, &to, &id, &amount);
    }

    #[endpoint]
    fn burn(&self, type_id: BigUint, amount: BigUint) {
        require!(
            self.is_fungible(&type_id).get(),
            "Only fungible tokens can be burned"
        );

        let caller = self.blockchain().get_caller();
        let balance = self.balance_of(&caller, &type_id);

        require!(balance >= amount, "Not enough tokens to burn");

        self.decrease_balance(&caller, &type_id, &amount);
    }

    // views

    #[view(balanceOf)]
    fn balance_of(&self, owner: &ManagedAddress, type_id: &BigUint) -> BigUint {
        self.get_balance_mapper(owner)
            .get(type_id)
            .unwrap_or_default()
    }

    // returns balance for each (owner, id) pair
    #[view(balanceOfBatch)]
    fn balance_of_batch(
        &self,
        owner_type_id_pairs: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
    ) -> MultiValueEncoded<BigUint> {
        let mut batch_balance = MultiValueEncoded::new();
        for multi_arg in owner_type_id_pairs.into_iter() {
            let (owner, type_id) = multi_arg.into_tuple();

            batch_balance.push(self.balance_of(&owner, &type_id));
        }

        batch_balance
    }

    // private

    fn is_valid_type_id(&self, type_id: &BigUint) -> bool {
        type_id > &0 && type_id <= &self.last_valid_type_id().get()
    }

    fn is_valid_nft_id(&self, type_id: &BigUint, nft_id: &BigUint) -> bool {
        self.is_valid_type_id(type_id)
            && nft_id > &0
            && nft_id <= &self.last_valid_nft_id_for_type(type_id).get()
    }

    fn increase_balance(&self, owner: &ManagedAddress, type_id: &BigUint, amount: &BigUint) {
        let mut balance = self.balance_of(owner, type_id);
        balance += amount;
        self.set_balance(owner, type_id, &balance);
    }

    fn decrease_balance(&self, owner: &ManagedAddress, type_id: &BigUint, amount: &BigUint) {
        let mut balance = self.balance_of(owner, type_id);
        balance -= amount;
        self.set_balance(owner, type_id, &balance);
    }

    fn set_balance(&self, owner: &ManagedAddress, type_id: &BigUint, amount: &BigUint) {
        let mut balance_mapper = self.get_balance_mapper(owner);
        balance_mapper.insert(type_id.clone(), amount.clone());
    }

    fn try_reserve_fungible(&self, owner: &ManagedAddress, type_id: &BigUint, amount: &BigUint) {
        let balance = self.balance_of(owner, type_id);

        require!(amount > &0u32, "Must transfer more than 0");
        require!(amount <= &balance, "Not enough balance for id");

        self.decrease_balance(owner, type_id, amount);
    }

    fn try_reserve_non_fungible(
        &self,
        owner: &ManagedAddress,
        type_id: &BigUint,
        nft_id: &BigUint,
    ) {
        require!(
            self.is_valid_nft_id(type_id, nft_id),
            "Token type-id pair is not valid"
        );
        require!(
            &self.token_owner(type_id, nft_id).get() == owner,
            "_from_ is not the owner of the token"
        );

        let amount = BigUint::from(1u32);
        self.decrease_balance(owner, type_id, &amount);
        self.token_owner(type_id, nft_id)
            .set(&ManagedAddress::zero());
    }

    /// Range is inclusive for both `start` and `end`
    fn set_owner_for_range(
        &self,
        type_id: &BigUint,
        start: &BigUint,
        end: &BigUint,
        owner: &ManagedAddress,
    ) {
        let big_uint_one = BigUint::from(1u32);
        let mut nft_id = start.clone();

        while &nft_id <= end {
            self.token_owner(type_id, &nft_id).set(owner);
            nft_id += &big_uint_one;
        }
    }

    fn peform_async_call_single_transfer(
        &self,
        from: ManagedAddress,
        to: ManagedAddress,
        type_id: BigUint,
        value: BigUint,
        data: &ManagedBuffer,
    ) {
        let caller = self.blockchain().get_caller();

        self.erc1155_user_proxy(to.clone())
            .on_erc1155_received(caller, from.clone(), type_id.clone(), value.clone(), data)
            .async_call()
            .with_callback(self.callbacks().transfer_callback(
                from,
                to,
                [type_id].to_vec(),
                [value].to_vec(),
            ))
            .call_and_exit()
    }

    fn peform_async_call_batch_transfer(
        &self,
        from: ManagedAddress,
        to: ManagedAddress,
        type_ids: &[BigUint],
        values: &[BigUint],
        data: &ManagedBuffer,
    ) {
        let caller = self.blockchain().get_caller();

        self.erc1155_user_proxy(to.clone())
            .on_erc1155_batch_received(
                caller,
                from.clone(),
                type_ids.to_vec(),
                values.to_vec(),
                data,
            )
            .async_call()
            .with_callback(self.callbacks().transfer_callback(
                from,
                to,
                type_ids.to_vec(),
                values.to_vec(),
            ))
            .call_and_exit()
    }

    // callbacks

    #[callback]
    fn transfer_callback(
        &self,
        from: ManagedAddress,
        to: ManagedAddress,
        type_ids: Vec<BigUint>,
        values: Vec<BigUint>,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        // in case of success, transfer to the intended address, otherwise, return tokens to original owner
        let dest_address = match result {
            ManagedAsyncCallResult::Ok(()) => to,
            ManagedAsyncCallResult::Err(_) => from,
        };
        let biguint_one = BigUint::from(1u32);

        for (type_id, value) in type_ids.iter().zip(values.iter()) {
            if self.is_fungible(type_id).get() {
                self.increase_balance(&dest_address, type_id, value);
            } else {
                self.increase_balance(&dest_address, type_id, &biguint_one);
                self.token_owner(type_id, value).set(&dest_address);
            }
        }
    }

    // proxy

    #[proxy]
    fn erc1155_user_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> erc1155_user_proxy::Proxy<Self::Api>;

    // storage

    // map for address -> type_id -> amount

    #[storage_mapper("balanceOf")]
    fn get_balance_mapper(&self, owner: &ManagedAddress) -> MapMapper<BigUint, BigUint>;

    // token owner
    // for non-fungible

    #[view(getTokenOwner)]
    #[storage_mapper("tokenOwner")]
    fn token_owner(&self, type_id: &BigUint, nft_id: &BigUint)
        -> SingleValueMapper<ManagedAddress>;

    // token creator

    #[view(getTokenTypeCreator)]
    #[storage_mapper("tokenTypeCreator")]
    fn token_type_creator(&self, type_id: &BigUint) -> SingleValueMapper<ManagedAddress>;

    // token type uri

    #[view(getTokenTypeUri)]
    #[storage_mapper("tokenTypeUri")]
    fn token_type_uri(&self, type_id: &BigUint) -> SingleValueMapper<BoxedBytes>;

    // check if a token is fungible

    #[view(isFungible)]
    #[storage_mapper("isFungible")]
    fn is_fungible(&self, type_id: &BigUint) -> SingleValueMapper<bool>;

    // last valid id

    #[storage_mapper("lastValidTypeId")]
    fn last_valid_type_id(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("lastValidTokenIdForType")]
    fn last_valid_nft_id_for_type(&self, type_id: &BigUint) -> SingleValueMapper<BigUint>;

    // check if an operator is approved. Default is false.

    #[view(isApprovedForAll)]
    #[storage_mapper("isApproved")]
    fn is_approved(
        &self,
        operator: &ManagedAddress,
        owner: &ManagedAddress,
    ) -> SingleValueMapper<bool>;
}
