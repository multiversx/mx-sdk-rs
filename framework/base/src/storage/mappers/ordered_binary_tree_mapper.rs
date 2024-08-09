use core::marker::PhantomData;

use codec::Empty;

use crate::{
    api::{ErrorApiImpl, StorageMapperApi},
    storage::StorageKey,
    storage_set,
    types::ManagedType,
};

use super::{
    set_mapper::{CurrentStorage, StorageAddress},
    StorageMapper,
};

use crate::codec::{
    self,
    derive::{TopDecode, TopEncode},
    NestedDecode, NestedEncode,
};

pub type NodeId = u64;

pub const NULL_NODE_ID: NodeId = 0;

static ROOT_ID_SUFFIX: &[u8] = b"_rootId";
static ID_SUFFIX: &[u8] = b"_id";
static LAST_ID_KEY_SUFFIX: &[u8] = b"_lastId";

static CORRUPT_TREE_ERR_MGS: &[u8] = b"Corrupt tree";

// https://en.wikipedia.org/wiki/Binary_search_tree

#[derive(TopEncode, TopDecode, Clone, PartialEq, Debug)]
pub struct OrderedBinaryTreeNode<T: NestedEncode + NestedDecode + PartialOrd + PartialEq + Clone> {
    pub current_node_id: NodeId,
    pub left_id: NodeId,
    pub right_id: NodeId,
    pub parent_id: NodeId,
    pub data: T,
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

impl<SA, T> StorageMapper<SA> for OrderedBinaryTreeMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: NestedEncode + NestedDecode + PartialOrd + PartialEq + Clone + 'static,
{
    #[inline]
    fn new(base_key: StorageKey<SA>) -> Self {
        OrderedBinaryTreeMapper {
            address: CurrentStorage,
            key: base_key,
            _phantom_api: PhantomData,
            _phantom_item: PhantomData,
        }
    }
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
        opt_node.as_ref()?;

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

    pub fn insert_element(&mut self, new_data: T) -> u64 {
        let new_node_id = self.get_and_increment_last_id();
        let mut new_node = OrderedBinaryTreeNode::new(new_node_id, new_data);

        let mut opt_new_node_parent = None;
        let mut opt_current_node = self.get_root();
        while opt_current_node.is_some() {
            opt_new_node_parent.clone_from(&opt_current_node);

            let current_node = unsafe { opt_current_node.unwrap_unchecked() };
            if new_node.data == current_node.data {
                return 0u64;
            }

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
            let root_id_key = self.build_root_id_key();
            storage_set(root_id_key.as_ref(), &new_node.current_node_id);

            let root_key = self.build_root_key();
            storage_set(root_key.as_ref(), &new_node);

            return 0u64;
        }

        let mut new_node_parent = unsafe { opt_new_node_parent.unwrap_unchecked() };
        if new_node.data < new_node_parent.data {
            new_node_parent.left_id = new_node.current_node_id;
        } else {
            new_node_parent.right_id = new_node.current_node_id;
        }

        self.set_item(new_node_id, &new_node);
        self.set_item(new_node_parent.current_node_id, &new_node_parent);

        new_node_id
    }

    pub fn delete_node(&mut self, data: T) {
        let opt_root = self.get_root();
        let opt_node = self.iterative_search(opt_root, &data);
        if opt_node.is_none() {
            SA::error_api_impl().signal_error(b"Node not found");
        }

        let node = unsafe { opt_node.unwrap_unchecked() };
        if node.left_id == NULL_NODE_ID {
            let opt_to_add = self.get_node_by_id(node.right_id);
            self.shift_nodes(&node, opt_to_add);

            return;
        }

        if node.right_id == NULL_NODE_ID {
            let opt_to_add = self.get_node_by_id(node.left_id);
            self.shift_nodes(&node, opt_to_add);

            return;
        }

        let opt_successor = self.find_successor(node.clone());
        if opt_successor.is_none() {
            SA::error_api_impl().signal_error(CORRUPT_TREE_ERR_MGS);
        }

        let mut successor = unsafe { opt_successor.unwrap_unchecked() };
        if successor.parent_id != node.current_node_id {
            let opt_right = self.get_node_by_id(successor.right_id);
            self.shift_nodes(&successor, opt_right);

            successor = self.try_get_node_by_id(successor.current_node_id);
            successor.right_id = node.right_id;

            let opt_successor_right_node = self.get_node_by_id(successor.right_id);
            if opt_successor_right_node.is_none() {
                SA::error_api_impl().signal_error(CORRUPT_TREE_ERR_MGS);
            }

            let mut successor_right_node = unsafe { opt_successor_right_node.unwrap_unchecked() };
            successor_right_node.parent_id = successor.current_node_id;

            self.set_item(successor_right_node.current_node_id, &successor_right_node);
        }

        self.shift_nodes(&node, Some(successor.clone()));
        successor = self.try_get_node_by_id(successor.current_node_id);
        successor.left_id = node.left_id;

        let opt_successor_left_node = self.get_node_by_id(successor.left_id);
        if opt_successor_left_node.is_none() {
            SA::error_api_impl().signal_error(CORRUPT_TREE_ERR_MGS);
        }

        let mut successor_left_node = unsafe { opt_successor_left_node.unwrap_unchecked() };
        successor_left_node.parent_id = successor.current_node_id;

        self.set_item(successor_left_node.current_node_id, &successor_left_node);
        self.set_item(successor.current_node_id, &successor);
    }

    fn shift_nodes(
        &mut self,
        to_delete: &OrderedBinaryTreeNode<T>,
        mut opt_to_add: Option<OrderedBinaryTreeNode<T>>,
    ) {
        if to_delete.parent_id == NULL_NODE_ID {
            let root_id_key = self.build_root_id_key();
            match &mut opt_to_add {
                Some(to_add) => {
                    to_add.parent_id = NULL_NODE_ID;
                    storage_set(root_id_key.as_ref(), &to_add.current_node_id);

                    let root_key = self.build_root_key();
                    storage_set(root_key.as_ref(), to_add);
                },
                None => {
                    let root_key = self.build_root_key();

                    storage_set(root_id_key.as_ref(), &Empty);
                    storage_set(root_key.as_ref(), &Empty);
                },
            };

            return;
        }

        let to_add_id = match &opt_to_add {
            Some(to_add) => to_add.current_node_id,
            None => NULL_NODE_ID,
        };

        let mut parent = self.try_get_node_by_id(to_delete.parent_id);
        if to_delete.current_node_id == parent.left_id {
            parent.left_id = to_add_id;
        } else {
            parent.right_id = to_add_id;
        }

        if let Some(to_add) = &mut opt_to_add {
            to_add.parent_id = to_delete.parent_id;

            self.set_item(to_add.current_node_id, to_add);
        }

        self.set_item(parent.current_node_id, &parent);
        self.clear_item(to_delete.current_node_id);
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

    fn build_root_id_key(&self) -> StorageKey<SA> {
        let mut key = self.key.clone();
        key.append_bytes(ROOT_ID_SUFFIX);

        key
    }

    fn build_root_key(&self) -> StorageKey<SA> {
        let mut key = self.key.clone();
        key.append_bytes(ROOT_ID_SUFFIX);

        let root_id = self.address.address_storage_get(key.as_ref());

        self.build_key_for_item(root_id)
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

    fn clear_item(&mut self, id: NodeId) {
        let key = self.build_key_for_item(id);
        storage_set(key.as_ref(), &Empty);
    }
}
