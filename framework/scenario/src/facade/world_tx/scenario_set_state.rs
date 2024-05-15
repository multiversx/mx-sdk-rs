mod scenario_set_account;
mod scenario_set_block;
mod scenario_set_new_address;

use crate::{
    imports::StaticApi,
    scenario::{
        tx_to_step::{address_annotated, big_uint_annotated, u64_annotated},
        ScenarioRunner,
    },
    scenario_model::{AddressKey, BigUintValue, NewAddress, SetStateStep},
    ScenarioTxEnvData, ScenarioWorld,
};

use multiversx_chain_vm::world_mock::EsdtInstanceMetadata;
use multiversx_sc::{
    proxy_imports::TopEncode,
    types::{AnnotatedValue, BigUint, EsdtLocalRole, ManagedAddress},
};
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
        A: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
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
        CA: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
        CN: AnnotatedValue<ScenarioTxEnvData, u64>,
        NA: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
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
        A: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
        V: AnnotatedValue<ScenarioTxEnvData, BigUint<StaticApi>>,
    {
        self.empty_builder().account(address).balance(egld_balance)
    }

    pub fn set_egld_balance<A, V>(&mut self, address: A, balance: V)
    where
        A: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
        V: AnnotatedValue<ScenarioTxEnvData, BigUint<StaticApi>>,
    {
        let env = self.new_env_data();
        let address_value = address_annotated(&env, &address);
        let balance_value = big_uint_annotated(&env, &balance);
        let accounts = &mut self.get_mut_state().accounts;
        for (vm_address_key, account) in accounts.iter_mut() {
            if vm_address_key == &address_value.to_vm_address() {
                account.egld_balance.clone_from(&balance_value.value);
            }
        }
    }

    pub fn set_esdt_balance<A, V>(&mut self, address: A, token_id: &[u8], balance: V)
    where
        A: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
        V: AnnotatedValue<ScenarioTxEnvData, BigUint<StaticApi>>,
    {
        let env = self.new_env_data();
        let address_value = address_annotated(&env, &address);
        let balance_value = big_uint_annotated(&env, &balance);
        let accounts = &mut self.get_mut_state().accounts;
        for (vm_address, account) in accounts.iter_mut() {
            if vm_address == &address_value.to_vm_address() {
                account.esdt.set_esdt_balance(
                    token_id.to_vec(),
                    0,
                    &balance_value.value,
                    EsdtInstanceMetadata::default(),
                )
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_nft_balance_all_properties<A, B, N, T: TopEncode, R, C>(
        &mut self,
        address: A,
        token_id: &[u8],
        nonce: N,
        balance: B,
        attributes: T,
        royalties: R,
        creator: Option<C>,
        name: Option<&[u8]>,
        hash: Option<&[u8]>,
        uris: &[Vec<u8>],
    ) where
        A: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
        B: AnnotatedValue<ScenarioTxEnvData, BigUint<StaticApi>>,
        N: AnnotatedValue<ScenarioTxEnvData, u64>,
        R: AnnotatedValue<ScenarioTxEnvData, u64>,
        C: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
    {
        let env = self.new_env_data();
        let address_value = address_annotated(&env, &address);
        let balance_value = big_uint_annotated(&env, &balance);
        let nonce_value = u64_annotated(&env, &nonce);
        let royalties_value = u64_annotated(&env, &royalties);

        let mut esdt_attributes = Vec::new();
        let _ = attributes.top_encode(&mut esdt_attributes);
        let accounts = &mut self.get_mut_state().accounts;
        for (vm_address, account) in accounts.iter_mut() {
            if vm_address == &address_value.to_vm_address() {
                account.esdt.set_esdt_balance(
                    token_id.to_vec(),
                    nonce_value.value,
                    &balance_value.value,
                    EsdtInstanceMetadata {
                        creator: creator
                            .as_ref()
                            .map(|c| address_annotated(&env, c).to_vm_address()),
                        attributes: esdt_attributes.clone(),
                        royalties: royalties_value.value,
                        name: name.unwrap_or_default().to_vec(),
                        hash: hash.map(|h| h.to_vec()),
                        uri: uris.to_vec(),
                    },
                )
            }
        }
    }

    pub fn set_developer_rewards<A: Copy, V: Copy>(&mut self, address: A, developer_rewards: V)
    where
        AddressKey: From<A>,
        BigUintValue: From<V>,
    {
        let accounts = &mut self.get_mut_state().accounts;
        for (vm_address, account) in accounts.iter_mut() {
            if vm_address == &AddressKey::from(address).to_vm_address() {
                account
                    .developer_rewards
                    .clone_from(&BigUintValue::from(developer_rewards).value);
            }
        }
    }

    pub fn set_esdt_local_roles<A>(&mut self, address: A, token_id: &[u8], roles: &[EsdtLocalRole])
    where
        A: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
    {
        let env = self.new_env_data();
        let address_value = address_annotated(&env, &address);
        let accounts = &mut self.get_mut_state().accounts;
        for (vm_address, account) in accounts.iter_mut() {
            if vm_address == &address_value.to_vm_address() {
                account.esdt.set_roles(
                    token_id.to_vec(),
                    roles
                        .iter()
                        .map(|role| role.as_role_name().to_vec())
                        .collect(),
                );
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
    fn new_env_data(&self) -> ScenarioTxEnvData {
        self.base.as_ref().unwrap().world.new_env_data()
    }

    /// Starts building of a new account.
    pub fn account<A>(mut self, address_expr: A) -> SetStateBuilder<'w, AccountItem>
    where
        A: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
    {
        let mut base = core::mem::take(&mut self.base).unwrap();
        let env = base.world.new_env_data();
        let address_value = address_annotated(&env, &address_expr);
        self.item.commit_to_step(&mut base.set_state_step);
        let item = base.start_account(address_value.into());
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
        CA: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
        CN: AnnotatedValue<ScenarioTxEnvData, u64>,
        NA: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
    {
        let mut base = core::mem::take(&mut self.base).unwrap();
        self.item.commit_to_step(&mut base.set_state_step);
        let env = base.world.new_env_data();
        SetStateBuilder {
            base: Some(base),
            item: NewAddressItem {
                new_address: NewAddress {
                    creator_address: address_annotated(&env, &creator_address_expr),
                    creator_nonce: u64_annotated(&env, &creator_nonce_expr),
                    new_address: address_annotated(&env, &new_address_expr),
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
