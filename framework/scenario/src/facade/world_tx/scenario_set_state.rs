mod scenario_set_account;
mod scenario_set_block;

use crate::{
    scenario::ScenarioRunner,
    scenario_model::{AddressKey, SetStateStep},
    ScenarioWorld,
};

use scenario_set_account::AccountItem;
use scenario_set_block::BlockItem;

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

pub(crate) struct SetStateBuilderBase<'w> {
    pub(crate) world: &'w mut ScenarioWorld,
    pub(crate) set_state_step: SetStateStep,
}

pub struct SetStateBuilder<'w, Current>
where
    Current: SetStateBuilderItem,
{
    pub(crate) base: Option<SetStateBuilderBase<'w>>,
    pub(crate) item: Current,
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
