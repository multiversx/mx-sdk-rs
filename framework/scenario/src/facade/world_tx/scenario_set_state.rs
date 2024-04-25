mod scenario_set_account;
mod scenario_set_block;
mod scenario_set_new_address;

use crate::{
    scenario::ScenarioRunner,
    scenario_model::{AddressKey, AddressValue, BigUintValue, NewAddress, SetStateStep, U64Value},
    ScenarioWorld,
};

use multiversx_chain_vm::world_mock::EsdtInstanceMetadata;
use multiversx_sc::codec::TopEncode;
use scenario_set_account::AccountItem;
use scenario_set_block::BlockItem;
use scenario_set_new_address::NewAddressItem;

impl ScenarioWorld {
    fn empty_builder(&mut self) -> SetStateBuilder<'_, ()> {
        SetStateBuilder {
            base: Some(SetStateBuilderBase::new(self)),
            item: (),
        }
    }

    pub fn account<A>(&mut self, address_expr: A) -> SetStateBuilder<'_, AccountItem>
    where
        AddressKey: From<A>,
    {
        self.empty_builder().account(address_expr)
    }

    pub fn new_address<CA, CN, NA>(
        &mut self,
        creator_address_expr: CA,
        creator_nonce_expr: CN,
        new_address_expr: NA,
    ) -> SetStateBuilder<'_, NewAddressItem>
    where
        AddressValue: From<CA>,
        U64Value: From<CN>,
        AddressValue: From<NA>,
    {
        self.empty_builder()
            .new_address(creator_address_expr, creator_nonce_expr, new_address_expr)
    }

    pub fn create_account_raw<A, V>(
        &mut self,
        address: A,
        egld_balance: V,
    ) -> SetStateBuilder<'_, AccountItem>
    where
        AddressKey: From<A>,
        BigUintValue: From<V>,
    {
        self.empty_builder().account(address).balance(egld_balance)
    }

    pub fn set_egld_balance<A: Copy, V: Copy>(&mut self, address: A, balance: V)
    where
        AddressKey: From<A>,
        BigUintValue: From<V>,
    {
        let vm_address = AddressKey::from(address).to_vm_address();
        let accounts = &mut self.get_mut_state().accounts;
        for (vm_address_key, account) in accounts.iter_mut() {
            if vm_address_key == &vm_address {
                account.egld_balance = BigUintValue::from(balance).value.clone();
            }
        }
    }

    pub fn set_esdt_balance<A: Copy, V: Copy>(&mut self, address: A, token_id: &[u8], balance: V)
    where
        AddressKey: From<A>,
        BigUintValue: From<V>,
    {
        // let token_id = BytesKey::from(token_id);
        let accounts = &mut self.get_mut_state().accounts;
        for (vm_address, account) in accounts.iter_mut() {
            if vm_address == &AddressKey::from(address).to_vm_address() {
                account.esdt.set_esdt_balance(
                    token_id.to_vec(),
                    0,
                    &BigUintValue::from(balance).value.clone(),
                    EsdtInstanceMetadata::default(),
                )
            }
        }
    }

    pub fn set_nft_balance_all_properties<A: Copy, V: Copy, NR: Copy, T: TopEncode>(
        &mut self,
        address: A,
        token_id: &[u8],
        nonce: NR,
        balance: V,
        attributes: &T,
        royalties: NR,
        creator: Option<A>,
        name: Option<&[u8]>,
        hash: Option<&[u8]>,
        uris: &[Vec<u8>],
    ) where
        AddressKey: From<A>,
        BigUintValue: From<V>,
        U64Value: From<NR>,
    {
        let mut esdt_attributes = Vec::new();
        let _ = attributes.top_encode(&mut esdt_attributes);
        let accounts = &mut self.get_mut_state().accounts;
        for (vm_address, account) in accounts.iter_mut() {
            if vm_address == &AddressKey::from(address).to_vm_address() {
                account.esdt.set_esdt_balance(
                    token_id.to_vec(),
                    U64Value::from(nonce).value,
                    &BigUintValue::from(balance).value.clone(),
                    EsdtInstanceMetadata {
                        creator: match creator {
                            Some(c) => Some(AddressKey::from(c).to_vm_address()),
                            None => None,
                        },
                        attributes: esdt_attributes.clone(),
                        royalties: U64Value::from(royalties).value,
                        name: name.unwrap_or_default().to_vec(),
                        hash: hash.map(|h| h.to_vec()),
                        uri: uris.to_vec(),
                    },
                )
            }
        }
    }

    pub fn current_block(&mut self) -> SetStateBuilder<'_, BlockItem> {
        self.empty_builder().current_block()
    }

    pub fn previous_block(&mut self) -> SetStateBuilder<'_, BlockItem> {
        self.empty_builder().previous_block()
    }
}

pub trait SetStateBuilderItem {
    fn commit_to_step(&mut self, step: &mut SetStateStep);
}

impl SetStateBuilderItem for () {
    fn commit_to_step(&mut self, _step: &mut SetStateStep) {}
}

struct SetStateBuilderBase<'w> {
    world: &'w mut ScenarioWorld,
    set_state_step: SetStateStep,
}

pub struct SetStateBuilder<'w, Current>
where
    Current: SetStateBuilderItem,
{
    base: Option<SetStateBuilderBase<'w>>,
    item: Current,
}

impl<'w> SetStateBuilderBase<'w> {
    fn new(world: &'w mut ScenarioWorld) -> Self {
        SetStateBuilderBase {
            world,
            set_state_step: SetStateStep::new(),
        }
    }

    fn start_account(&self, address: AddressKey) -> AccountItem {
        assert!(
            !self
                .world
                .get_debugger_backend()
                .vm_runner
                .blockchain_mock
                .state
                .account_exists(&address.to_vm_address()),
            "updating existing accounts currently not supported"
        );

        AccountItem::new(address)
    }
}

impl<'w> SetStateBuilder<'w, ()> {}

impl<'w, Item> SetStateBuilder<'w, Item>
where
    Item: SetStateBuilderItem,
{
    /// Starts building of a new account.
    pub fn account<A>(mut self, address_expr: A) -> SetStateBuilder<'w, AccountItem>
    where
        AddressKey: From<A>,
    {
        let mut base = core::mem::take(&mut self.base).unwrap();
        self.item.commit_to_step(&mut base.set_state_step);
        let item = base.start_account(address_expr.into());
        SetStateBuilder {
            base: Some(base),
            item,
        }
    }

    pub fn new_address<CA, CN, NA>(
        mut self,
        creator_address_expr: CA,
        creator_nonce_expr: CN,
        new_address_expr: NA,
    ) -> SetStateBuilder<'w, NewAddressItem>
    where
        AddressValue: From<CA>,
        U64Value: From<CN>,
        AddressValue: From<NA>,
    {
        let mut base = core::mem::take(&mut self.base).unwrap();
        self.item.commit_to_step(&mut base.set_state_step);
        SetStateBuilder {
            base: Some(base),
            item: NewAddressItem {
                new_address: NewAddress {
                    creator_address: AddressValue::from(creator_address_expr),
                    creator_nonce: U64Value::from(creator_nonce_expr),
                    new_address: AddressValue::from(new_address_expr),
                },
            },
        }
    }

    pub fn current_block(&mut self) -> SetStateBuilder<'w, BlockItem> {
        let mut base = core::mem::take(&mut self.base).unwrap();
        self.item.commit_to_step(&mut base.set_state_step);
        SetStateBuilder {
            base: Some(base),
            item: BlockItem::new_current(),
        }
    }

    pub fn previous_block(&mut self) -> SetStateBuilder<'w, BlockItem> {
        let mut base = core::mem::take(&mut self.base).unwrap();
        self.item.commit_to_step(&mut base.set_state_step);
        SetStateBuilder {
            base: Some(base),
            item: BlockItem::new_prev(),
        }
    }

    /// Forces value drop and commit accounts.
    pub fn commit(self) {}
}

impl<'w, Current> Drop for SetStateBuilder<'w, Current>
where
    Current: SetStateBuilderItem,
{
    fn drop(&mut self) {
        if let Some(base) = &mut self.base {
            self.item.commit_to_step(&mut base.set_state_step);
            base.world.run_set_state_step(&base.set_state_step);
        }
    }
}
