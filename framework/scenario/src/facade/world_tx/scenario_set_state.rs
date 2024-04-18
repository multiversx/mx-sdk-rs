mod scenario_set_account;
mod scenario_set_block;
mod scenario_set_new_address;

use crate::{
    scenario::ScenarioRunner,
    scenario_model::{AddressKey, AddressValue, NewAddress, SetStateStep, U64Value},
    ScenarioWorld,
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
