use crate::{
    scenario::ScenarioRunner,
    scenario_model::{Account, AddressKey, SetStateStep},
    ScenarioWorld,
};

use super::{block_info_builder::BlockItem, scenario_set_state_account::CurrentAccount};

impl ScenarioWorld {
    pub fn account<A>(&mut self, address_expr: A) -> SetStateBuilder<'_, CurrentAccount>
    where
        AddressKey: From<A>,
    {
        let base = SetStateBuilderBase::new(self);
        let item = base.start_account(address_expr.into());
        SetStateBuilder {
            base: Some(base),
            item,
        }
    }

    pub fn current_block(&mut self) -> SetStateBuilder<'_, BlockItem> {
        SetStateBuilder {
            base: Some(SetStateBuilderBase::new(self)),
            item: BlockItem::new_current(),
        }
    }

    pub fn previous_block(&mut self) -> SetStateBuilder<'_, BlockItem> {
        SetStateBuilder {
            base: Some(SetStateBuilderBase::new(self)),
            item: BlockItem::new_current(),
        }
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

    fn start_account(&self, address: AddressKey) -> CurrentAccount {
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

        CurrentAccount {
            address,
            account: Account::default(),
        }
    }
}

impl<'w> SetStateBuilder<'w, ()> {}

impl<'w, Item> SetStateBuilder<'w, Item>
where
    Item: SetStateBuilderItem,
{
    /// Starts building of a new account.
    pub fn account<A>(mut self, address_expr: A) -> SetStateBuilder<'w, CurrentAccount>
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
