use core::marker::PhantomData;

use crate::{
    api::StorageMapperApi,
    imports::{ErrorApiImpl, ManagedType},
    storage::StorageKey,
    storage_set,
};

use super::set_mapper::{CurrentStorage, StorageAddress};

use crate::codec::{
    self,
    derive::{TopDecode, TopEncode},
    NestedDecode, NestedEncode,
};

pub type NodeId = u64;

pub const NULL_NODE_ID: NodeId = 0;

static ROOT_SUFFIX: &[u8] = b"_root";
static ID_SUFFIX: &[u8] = b"_id";
static LAST_ID_KEY_SUFFIX: &[u8] = b"_lastId";

static CORRUPT_TREE_ERR_MGS: &[u8] = b"Corrupt tree";

// https://en.wikipedia.org/wiki/Binary_search_tree

#[derive(TopEncode, TopDecode, Clone)]
pub struct OrderedBinaryTreeNode<T: NestedEncode + NestedDecode + PartialOrd + PartialEq + Clone> {
    pub(crate) current_node_id: NodeId,
    pub(crate) left_id: NodeId,
    pub(crate) right_id: NodeId,
    pub(crate) parent_id: NodeId,
    pub(crate) data: T,
}

impl<T> OrderedBinaryTreeNode<T>
where
    T: NestedEncode + NestedDecode + PartialOrd + PartialEq + Clone,
{
    pub fn new(current_node_id: NodeId, data: T) -> Self {
        Self {
            data,
            current_node_id,
            left_id: NULL_NODE_ID,
            right_id: NULL_NODE_ID,
            parent_id: NULL_NODE_ID,
        }
    }

    #[inline]
    pub fn get_data(&self) -> &T {
        &self.data
    }
}

pub struct OrderedBinaryTreeMapper<SA, T, A = CurrentStorage>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: NestedEncode + NestedDecode + PartialOrd + PartialEq + Clone,
{
    address: A,
    key: StorageKey<SA>,
    _phantom_api: PhantomData<SA>,
    _phantom_item: PhantomData<T>,
}

impl<SA, T, A> OrderedBinaryTreeMapper<SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: NestedEncode + NestedDecode + PartialOrd + PartialEq + Clone,
{
    pub fn get_root(&self) -> Option<OrderedBinaryTreeNode<T>> {
        let root_key = self.build_root_key();
        let storage_len = self.address.address_storage_get_len(root_key.as_ref());
        if storage_len == 0 {
            return None;
        }

        Some(self.address.address_storage_get(root_key.as_ref()))
    }

    pub fn get_depth(&self, node: &OrderedBinaryTreeNode<T>) -> usize {
        if self.get_root().is_none() {
            return 0;
        }

        let opt_left_node = self.get_node_by_id(node.left_id);
        let opt_right_node = self.get_node_by_id(node.right_id);

        let l_depth = match opt_left_node {
            Some(left_node) => self.get_depth(&left_node),
            None => 0,
        };
        let r_depth = match opt_right_node {
            Some(right_node) => self.get_depth(&right_node),
            None => 0,
        };

        core::cmp::max(l_depth, r_depth) + 1
    }

    pub fn recursive_search(
        &self,
        opt_node: Option<OrderedBinaryTreeNode<T>>,
        data: &T,
    ) -> Option<OrderedBinaryTreeNode<T>> {
        if opt_node.is_none() {
            return None;
        }

        let node = unsafe { opt_node.unwrap_unchecked() };
        if &node.data == data {
            return Some(node);
        }

        if data < &node.data {
            let opt_left_node = self.get_node_by_id(node.left_id);
            self.recursive_search(opt_left_node, data)
        } else {
            let opt_right_node = self.get_node_by_id(node.right_id);
            self.recursive_search(opt_right_node, data)
        }
    }

    pub fn iterative_search(
        &self,
        mut opt_node: Option<OrderedBinaryTreeNode<T>>,
        data: &T,
    ) -> Option<OrderedBinaryTreeNode<T>> {
        while opt_node.is_some() {
            let node = unsafe { opt_node.unwrap_unchecked() };
            if &node.data == data {
                return Some(node);
            }

            if data < &node.data {
                opt_node = self.get_node_by_id(node.left_id);
            } else {
                opt_node = self.get_node_by_id(node.right_id);
            }
        }

        None
    }

    pub fn find_max(&self, mut node: OrderedBinaryTreeNode<T>) -> OrderedBinaryTreeNode<T> {
        while node.right_id != NULL_NODE_ID {
            node = self.try_get_node_by_id(node.right_id);
        }

        node
    }

    pub fn find_min(&self, mut node: OrderedBinaryTreeNode<T>) -> OrderedBinaryTreeNode<T> {
        while node.left_id != NULL_NODE_ID {
            node = self.try_get_node_by_id(node.left_id);
        }

        node
    }

    pub fn find_successor(
        &self,
        mut node: OrderedBinaryTreeNode<T>,
    ) -> Option<OrderedBinaryTreeNode<T>> {
        if node.right_id != NULL_NODE_ID {
            let right_node = self.try_get_node_by_id(node.right_id);
            return Some(self.find_min(right_node));
        }

        let mut successor_id = node.parent_id;
        let mut opt_successor = self.get_node_by_id(successor_id);
        while successor_id != NULL_NODE_ID {
            if opt_successor.is_none() {
                SA::error_api_impl().signal_error(CORRUPT_TREE_ERR_MGS);
            }

            let successor = unsafe { opt_successor.unwrap_unchecked() };
            if node.current_node_id != successor.right_id {
                return Some(successor);
            }

            successor_id = successor.parent_id;
            opt_successor = self.get_node_by_id(successor_id);
            node = successor;
        }

        opt_successor
    }

    pub fn find_predecessor(
        &self,
        mut node: OrderedBinaryTreeNode<T>,
    ) -> Option<OrderedBinaryTreeNode<T>> {
        if node.left_id != NULL_NODE_ID {
            let left_node = self.try_get_node_by_id(node.left_id);
            return Some(self.find_max(left_node));
        }

        let mut predecessor_id = node.parent_id;
        let mut opt_predecessor = self.get_node_by_id(predecessor_id);
        while predecessor_id != NULL_NODE_ID {
            if opt_predecessor.is_none() {
                SA::error_api_impl().signal_error(CORRUPT_TREE_ERR_MGS);
            }

            let predecessor = unsafe { opt_predecessor.unwrap_unchecked() };
            if node.current_node_id != predecessor.left_id {
                return Some(predecessor);
            }

            predecessor_id = predecessor.parent_id;
            opt_predecessor = self.get_node_by_id(predecessor_id);
            node = predecessor;
        }

        opt_predecessor
    }

    pub fn insert_element(&mut self, new_data: T) {
        let new_node_id = self.get_and_increment_last_id();
        let mut new_node = OrderedBinaryTreeNode::new(new_node_id, new_data);

        let mut opt_new_node_parent = None;
        let mut opt_current_node = self.get_root();
        while opt_current_node.is_some() {
            opt_new_node_parent = opt_current_node.clone();

            let current_node = unsafe { opt_current_node.unwrap_unchecked() };
            if new_node.data < current_node.data {
                opt_current_node = self.get_node_by_id(current_node.left_id);
            } else {
                opt_current_node = self.get_node_by_id(current_node.right_id);
            }
        }

        let new_node_parent_id = match &opt_new_node_parent {
            Some(node) => node.current_node_id,
            None => NULL_NODE_ID,
        };
        new_node.parent_id = new_node_parent_id;

        if opt_new_node_parent.is_none() {
            let root_key = self.build_root_key();
            storage_set(root_key.as_ref(), &new_node);

            return;
        }

        let mut new_node_parent = unsafe { opt_new_node_parent.unwrap_unchecked() };
        if new_node.data < new_node_parent.data {
            new_node_parent.left_id = new_node.current_node_id;
        } else {
            new_node_parent.right_id = new_node.current_node_id;
        }

        self.set_item(new_node_id, &new_node);
        self.set_item(new_node_parent.current_node_id, &new_node_parent);
    }
}

impl<SA, T, A> OrderedBinaryTreeMapper<SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: NestedEncode + NestedDecode + PartialOrd + PartialEq + Clone,
{
    fn get_node_by_id(&self, id: NodeId) -> Option<OrderedBinaryTreeNode<T>> {
        if id == NULL_NODE_ID {
            return None;
        }

        let key = self.build_key_for_item(id);
        let storage_len = self.address.address_storage_get_len(key.as_ref());
        if storage_len == 0 {
            return None;
        }

        Some(self.address.address_storage_get(key.as_ref()))
    }

    fn try_get_node_by_id(&self, id: NodeId) -> OrderedBinaryTreeNode<T> {
        let opt_node = self.get_node_by_id(id);
        if opt_node.is_none() {
            SA::error_api_impl().signal_error(CORRUPT_TREE_ERR_MGS);
        }

        unsafe { opt_node.unwrap_unchecked() }
    }

    fn build_root_key(&self) -> StorageKey<SA> {
        let mut key = self.key.clone();
        key.append_bytes(ROOT_SUFFIX);

        key
    }

    fn build_key_for_item(&self, id: NodeId) -> StorageKey<SA> {
        let mut item_key = self.key.clone();
        item_key.append_bytes(ID_SUFFIX);
        item_key.append_item(&id);

        item_key
    }

    fn build_last_id_key(&self) -> StorageKey<SA> {
        let mut key = self.key.clone();
        key.append_bytes(LAST_ID_KEY_SUFFIX);

        key
    }

    // fn get_last_id(&self) -> NodeId {
    //     let key = self.build_last_id_key();

    //     self.address.address_storage_get(key.as_ref())
    // }

    fn get_and_increment_last_id(&self) -> NodeId {
        let key = self.build_last_id_key();
        let last_id: NodeId = self.address.address_storage_get(key.as_ref());
        let new_id = last_id + 1;
        storage_set(key.as_ref(), &new_id);

        new_id
    }

    fn set_item(&mut self, id: NodeId, node: &OrderedBinaryTreeNode<T>) {
        let key = self.build_key_for_item(id);
        storage_set(key.as_ref(), node);
    }
}
