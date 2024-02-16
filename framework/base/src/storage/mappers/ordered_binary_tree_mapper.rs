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
    NestedDecode, NestedEncode, TopDecode, TopEncode,
};

pub type NodeId = u64;

const NULL_ID: NodeId = 0;

static ID_SUFFIX: &[u8] = b"_id";
static LAST_ID_KEY_SUFFIX: &[u8] = b"_lastId";

static CORRUPT_TREE_ERR_MGS: &[u8] = b"Corrupt tree";

#[derive(TopEncode, TopDecode, Clone)]
pub struct OrderedBinaryTreeNode<
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + PartialOrd + PartialEq + Clone,
> {
    pub(crate) left_id: NodeId,
    pub(crate) right_id: NodeId,
    pub(crate) data: T,
}

impl<T> OrderedBinaryTreeNode<T>
where
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + PartialOrd + PartialEq + Clone,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            left_id: NULL_ID,
            right_id: NULL_ID,
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
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + PartialOrd + PartialEq + Clone,
{
    opt_root: Option<OrderedBinaryTreeNode<T>>,
    address: A,
    key: StorageKey<SA>,
    _phantom_api: PhantomData<SA>,
    _phantom_item: PhantomData<T>,
}

impl<SA, T, A> OrderedBinaryTreeMapper<SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + PartialOrd + PartialEq + Clone,
{
    pub fn get_root(&self) -> Option<OrderedBinaryTreeNode<T>> {
        self.opt_root.clone()
    }

    pub fn get_node_by_id(&self, id: NodeId) -> Option<OrderedBinaryTreeNode<T>> {
        if id == NULL_ID {
            return None;
        }

        let key = self.build_key_for_item(id);
        let storage_len = self.address.address_storage_get_len(key.as_ref());
        if storage_len == 0 {
            return None;
        }

        Some(self.address.address_storage_get(key.as_ref()))
    }

    pub fn try_get_node_by_id(&self, id: NodeId) -> OrderedBinaryTreeNode<T> {
        let opt_node = self.get_node_by_id(id);
        if opt_node.is_none() {
            SA::error_api_impl().signal_error(CORRUPT_TREE_ERR_MGS);
        }

        unsafe { opt_node.unwrap_unchecked() }
    }

    pub fn get_depth(&self, node: &OrderedBinaryTreeNode<T>) -> usize {
        if self.opt_root.is_none() {
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
        while node.right_id != NULL_ID {
            node = self.try_get_node_by_id(node.right_id);
        }

        node
    }

    pub fn find_min(&self, mut node: OrderedBinaryTreeNode<T>) -> OrderedBinaryTreeNode<T> {
        while node.left_id != NULL_ID {
            node = self.try_get_node_by_id(node.left_id);
        }

        node
    }

    pub fn find_successor(&self, node: OrderedBinaryTreeNode<T>) -> OrderedBinaryTreeNode<T> {
        if node.right_id != NULL_ID {
            let right_node = self.try_get_node_by_id(node.right_id);
            return self.find_min(right_node);
        }
    }

    // pub fn insert_element(&mut self, data: T) {
    //     let new_node = OrderedBinaryTreeNode::new(data);
    //     if self.opt_root.is_none() {
    //         let root_id = self.get_and_increment_last_id();
    //         self.set_item(root_id, &new_node);
    //     }
    // }
}

impl<SA, T, A> OrderedBinaryTreeMapper<SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + PartialOrd + PartialEq + Clone,
{
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

    fn get_last_id(&self) -> NodeId {
        let key = self.build_last_id_key();

        self.address.address_storage_get(key.as_ref())
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
}
