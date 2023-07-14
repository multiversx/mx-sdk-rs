use std::{cell::RefCell, rc::Rc};

use multiversx_sc_snippets::multiversx_sc::types::Address;

#[derive(Default, Clone)]
pub struct CallState {
    pub root: Rc<RefCell<ForwarderQueueTarget>>,
    pub forwarders: Vec<Rc<RefCell<ForwarderQueueTarget>>>,
    pub vaults: Vec<Rc<RefCell<VaultTarget>>>,
}

#[derive(Default)]
pub struct ForwarderQueueTarget {
    pub name: String,
    pub address: Option<Address>,
    pub children: Vec<CallNode>,
}

pub struct VaultTarget {
    pub name: String,
    pub address: Option<Address>,
}

#[derive(Clone)]
pub enum CallNode {
    ForwarderQueue(Rc<RefCell<ForwarderQueueTarget>>),
    Vault(Rc<RefCell<VaultTarget>>),
}

pub fn add_vault_child(
    call_state: &mut CallState,
    fwd: Rc<RefCell<ForwarderQueueTarget>>,
    name: String,
) -> Rc<RefCell<VaultTarget>> {
    let vault_target = Rc::new(RefCell::new(VaultTarget {
        name,
        address: None,
    }));
    fwd.borrow_mut()
        .children
        .push(CallNode::Vault(vault_target.clone()));
    call_state.vaults.push(vault_target.clone());
    vault_target
}

pub fn add_forwarder_child(
    call_state: &mut CallState,
    fwd: Rc<RefCell<ForwarderQueueTarget>>,
    name: String,
) -> Rc<RefCell<ForwarderQueueTarget>> {
    let fwd_target = Rc::new(RefCell::new(ForwarderQueueTarget {
        name,
        address: None,
        children: Vec::new(),
    }));
    fwd.borrow_mut()
        .children
        .push(CallNode::ForwarderQueue(fwd_target.clone()));
    call_state.forwarders.push(fwd_target.clone());
    fwd_target
}

impl CallState {
    pub fn create_root() -> Self {
        let root = Rc::new(RefCell::new(ForwarderQueueTarget {
            name: "root".to_string(),
            ..Default::default()
        }));
        CallState {
            root: root.clone(),
            forwarders: vec![root],
            vaults: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn simple_example_1() -> CallState {
        let mut state = Self::create_root();
        let root_rc = state.root.clone();
        let _ = add_vault_child(&mut state, root_rc, "single vault".to_string());
        state
    }

    pub fn simple_example_2() -> CallState {
        let mut state = Self::create_root();
        let root_rc = state.root.clone();
        let fwd1 = add_forwarder_child(&mut state, root_rc, "fwd1".to_string());
        let _ = add_vault_child(&mut state, fwd1, "vault1".to_string());
        state
    }
}

fn print_node(call_node: &CallNode, indent: usize) {
    for _ in 0..indent {
        print!("   ");
    }
    match call_node {
        CallNode::ForwarderQueue(fwd_rc) => {
            let fwd = (**fwd_rc).borrow();
            println!("{}", &fwd.name);
            for child in &fwd.children {
                print_node(child, indent + 1);
            }
        },
        CallNode::Vault(vault_rc) => {
            let vault = (**vault_rc).borrow();
            println!("{}", &vault.name)
        },
    }
}

impl CallState {
    pub fn print(&self) {
        print!("Forwarders: [");
        for fwd_rc in &self.forwarders {
            let fwd = (**fwd_rc).borrow();
            print!(" {},", &fwd.name);
        }
        println!(" ]");
        print!("Vaults: [");
        for vault_rc in &self.vaults {
            let vault = (**vault_rc).borrow();
            print!(" {},", &vault.name);
        }
        println!(" ]");
        print_node(&CallNode::ForwarderQueue(self.root.clone()), 0);
    }
}
