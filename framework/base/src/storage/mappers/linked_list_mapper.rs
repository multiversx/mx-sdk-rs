use core::marker::PhantomData;

use super::{StorageClearable, StorageMapper};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::StorageMapperApi,
    codec::{
        self,
        derive::{
            NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode,
            TopEncodeOrDefault,
        },
        CodecFrom, DecodeDefault, EncodeDefault, EncodeErrorHandler, NestedDecode, NestedEncode,
        TopDecode, TopEncode, TopEncodeMulti, TopEncodeMultiOutput,
    },
    storage::{storage_get, storage_set, StorageKey},
    types::{heap::BoxedBytes, ManagedType, MultiValueEncoded},
};
use alloc::vec::Vec;
use storage_get::storage_get_len;

const NULL_ENTRY: u32 = 0;
const INFO_IDENTIFIER: &[u8] = b".info";
const NODE_IDENTIFIER: &[u8] = b".node";

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Copy)]
pub struct LinkedListNode<T: NestedEncode + NestedDecode + TopEncode + TopDecode + Clone> {
    pub(crate) value: T,
    pub(crate) node_id: u32,
    pub(crate) next_id: u32,
    pub(crate) prev_id: u32,
}

impl<T: NestedEncode + NestedDecode + TopEncode + TopDecode + Clone> LinkedListNode<T> {
    pub fn get_value_cloned(&self) -> T {
        self.value.clone()
    }

    pub fn get_value_as_ref(&self) -> &T {
        &self.value
    }

    pub fn into_value(self) -> T {
        self.value
    }

    pub fn get_node_id(&self) -> u32 {
        self.node_id
    }

    pub fn get_next_node_id(&self) -> u32 {
        self.next_id
    }

    pub fn get_prev_node_id(&self) -> u32 {
        self.prev_id
    }
}

#[derive(TopEncodeOrDefault, TopDecodeOrDefault, PartialEq, Eq, Clone, Copy)]
pub struct LinkedListInfo {
    pub len: u32,
    pub front: u32,
    pub back: u32,
    pub new: u32,
}

impl EncodeDefault for LinkedListInfo {
    fn is_default(&self) -> bool {
        self.len == 0
    }
}

impl DecodeDefault for LinkedListInfo {
    fn default() -> Self {
        Self {
            len: 0,
            front: 0,
            back: 0,
            new: 0,
        }
    }
}

impl LinkedListInfo {
    pub fn generate_new_node_id(&mut self) -> u32 {
        self.new += 1;
        self.new
    }
}

pub struct LinkedListMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone + 'static,
{
    _phantom_api: PhantomData<SA>,
    base_key: StorageKey<SA>,
    _phantom_item: PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for LinkedListMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        LinkedListMapper {
            _phantom_api: PhantomData,
            base_key,
            _phantom_item: PhantomData,
        }
    }
}

impl<SA, T> StorageClearable for LinkedListMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone,
{
    fn clear(&mut self) {
        let info = self.get_info();
        let mut node_id = info.front;

        while node_id != NULL_ENTRY {
            let node = self.get_node(node_id);
            self.clear_node(node_id);
            node_id = node.next_id;
        }

        self.set_info(LinkedListInfo::default());
    }
}

impl<SA, T> LinkedListMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone,
{
    fn build_node_id_named_key(&self, name: &[u8], node_id: u32) -> StorageKey<SA> {
        let mut named_key = self.base_key.clone();
        named_key.append_bytes(name);
        named_key.append_item(&node_id);
        named_key
    }

    fn build_name_key(&self, name: &[u8]) -> StorageKey<SA> {
        let mut name_key = self.base_key.clone();
        name_key.append_bytes(name);
        name_key
    }

    fn get_info(&self) -> LinkedListInfo {
        storage_get(self.build_name_key(INFO_IDENTIFIER).as_ref())
    }

    fn set_info(&mut self, value: LinkedListInfo) {
        storage_set(self.build_name_key(INFO_IDENTIFIER).as_ref(), &value);
    }

    fn get_node(&self, node_id: u32) -> LinkedListNode<T> {
        storage_get(
            self.build_node_id_named_key(NODE_IDENTIFIER, node_id)
                .as_ref(),
        )
    }

    fn is_empty_node(&self, node_id: u32) -> bool {
        storage_get_len(
            self.build_node_id_named_key(NODE_IDENTIFIER, node_id)
                .as_ref(),
        ) == 0
    }

    fn set_node(&mut self, node_id: u32, item: &LinkedListNode<T>) {
        storage_set(
            self.build_node_id_named_key(NODE_IDENTIFIER, node_id)
                .as_ref(),
            item,
        );
    }

    fn clear_node(&mut self, node_id: u32) {
        storage_set(
            self.build_node_id_named_key(NODE_IDENTIFIER, node_id)
                .as_ref(),
            &BoxedBytes::empty(),
        );
    }

    pub fn is_empty(&self) -> bool {
        self.get_info().len == 0
    }

    pub fn len(&self) -> usize {
        self.get_info().len as usize
    }

    pub fn front(&self) -> Option<LinkedListNode<T>> {
        let info = self.get_info();

        self.get_node_by_id(info.front)
    }

    pub fn back(&self) -> Option<LinkedListNode<T>> {
        let info = self.get_info();

        self.get_node_by_id(info.back)
    }

    pub fn pop_back(&mut self) -> Option<LinkedListNode<T>> {
        let info = self.get_info();

        self.remove_node_by_id(info.back)
    }

    pub fn pop_front(&mut self) -> Option<LinkedListNode<T>> {
        let info = self.get_info();

        self.remove_node_by_id(info.front)
    }

    pub fn push_after(
        &mut self,
        node: &mut LinkedListNode<T>,
        element: T,
    ) -> Option<LinkedListNode<T>> {
        if self.is_empty_node(node.node_id) {
            return None;
        }

        let mut info = self.get_info();
        let new_node_id = info.generate_new_node_id();

        let new_node_next_id = node.next_id;
        node.next_id = new_node_id;
        self.set_node(node.node_id, node);

        if new_node_next_id == NULL_ENTRY {
            info.back = new_node_id;
        } else {
            let mut next_node = self.get_node(new_node_next_id);
            next_node.prev_id = new_node_id;
            self.set_node(new_node_next_id, &next_node);
        }

        let new_node = LinkedListNode {
            value: element,
            node_id: new_node_id,
            next_id: new_node_next_id,
            prev_id: node.node_id,
        };
        self.set_node(new_node_id, &new_node);

        info.len += 1;
        self.set_info(info);
        Some(new_node)
    }

    pub fn push_before(
        &mut self,
        node: &mut LinkedListNode<T>,
        element: T,
    ) -> Option<LinkedListNode<T>> {
        if self.is_empty_node(node.node_id) {
            return None;
        }

        let mut info = self.get_info();
        let new_node_id = info.generate_new_node_id();

        let new_node_prev_id = node.prev_id;
        node.prev_id = new_node_id;
        self.set_node(node.node_id, node);

        if new_node_prev_id == NULL_ENTRY {
            info.front = new_node_id;
        } else {
            let mut previous_node = self.get_node(new_node_prev_id);
            previous_node.next_id = new_node_id;
            self.set_node(new_node_prev_id, &previous_node);
        }

        let new_node = LinkedListNode {
            value: element,
            node_id: new_node_id,
            next_id: node.node_id,
            prev_id: new_node_prev_id,
        };
        self.set_node(new_node_id, &new_node);

        info.len += 1;
        self.set_info(info);
        Some(new_node)
    }

    pub fn push_after_node_id(&mut self, node_id: u32, element: T) -> Option<LinkedListNode<T>> {
        if !self.is_empty_node(node_id) {
            let mut node = self.get_node(node_id);
            self.push_after(&mut node, element)
        } else {
            None
        }
    }

    pub fn push_before_node_id(&mut self, node_id: u32, element: T) -> Option<LinkedListNode<T>> {
        if !self.is_empty_node(node_id) {
            let mut node = self.get_node(node_id);
            self.push_before(&mut node, element)
        } else {
            None
        }
    }

    pub fn push_back(&mut self, element: T) -> LinkedListNode<T> {
        let mut info = self.get_info();
        let new_node_id = info.generate_new_node_id();
        let mut previous = NULL_ENTRY;

        if info.len == 0 {
            info.front = new_node_id;
        } else {
            let back = info.back;
            let mut back_node = self.get_node(back);
            back_node.next_id = new_node_id;
            previous = back;
            self.set_node(back, &back_node);
        }

        let node = LinkedListNode {
            value: element,
            node_id: new_node_id,
            prev_id: previous,
            next_id: NULL_ENTRY,
        };
        self.set_node(new_node_id, &node);

        info.back = new_node_id;
        info.len += 1;
        self.set_info(info);
        node
    }

    pub fn push_front(&mut self, element: T) -> LinkedListNode<T> {
        let mut info = self.get_info();
        let new_node_id = info.generate_new_node_id();
        let mut next = NULL_ENTRY;

        if info.len == 0 {
            info.back = new_node_id;
        } else {
            let front = info.front;
            let mut front_node = self.get_node(front);
            front_node.prev_id = new_node_id;
            next = front;
            self.set_node(front, &front_node);
        }

        let node = LinkedListNode {
            value: element,
            node_id: new_node_id,
            prev_id: NULL_ENTRY,
            next_id: next,
        };
        self.set_node(new_node_id, &node);

        info.front = new_node_id;
        info.len += 1;
        self.set_info(info);
        node
    }

    pub fn set_node_value(&mut self, mut node: LinkedListNode<T>, new_value: T) {
        if self.is_empty_node(node.node_id) {
            return;
        }

        node.value = new_value;
        self.set_node(node.node_id, &node);
    }

    pub fn set_node_value_by_id(&mut self, node_id: u32, new_value: T) {
        if let Some(node) = self.get_node_by_id(node_id) {
            self.set_node_value(node, new_value)
        }
    }

    pub fn remove_node(&mut self, node: &LinkedListNode<T>) {
        let node_id = node.node_id;

        if self.is_empty_node(node_id) {
            return;
        }

        let mut info = self.get_info();
        if node.prev_id == NULL_ENTRY {
            info.front = node.next_id;
        } else {
            let mut previous = self.get_node(node.prev_id);
            previous.next_id = node.next_id;
            self.set_node(node.prev_id, &previous);
        }

        if node.next_id == NULL_ENTRY {
            info.back = node.prev_id;
        } else {
            let mut next = self.get_node(node.next_id);
            next.prev_id = node.prev_id;
            self.set_node(node.next_id, &next);
        }

        self.clear_node(node_id);
        info.len -= 1;
        self.set_info(info);
    }

    pub fn remove_node_by_id(&mut self, node_id: u32) -> Option<LinkedListNode<T>> {
        if self.is_empty_node(node_id) {
            return None;
        }

        let node = self.get_node_by_id(node_id).unwrap();
        self.remove_node(&node);
        Some(node)
    }

    pub fn get_node_by_id(&self, node_id: u32) -> Option<LinkedListNode<T>> {
        if self.is_empty_node(node_id) {
            return None;
        }

        Some(self.get_node(node_id))
    }

    pub fn iter(&self) -> Iter<SA, T> {
        Iter::new(self)
    }

    pub fn iter_from_node_id(&self, node_id: u32) -> Iter<SA, T> {
        Iter::new_from_node_id(self, node_id)
    }

    pub fn check_internal_consistency(&self) -> bool {
        let info = self.get_info();
        let mut front = info.front;
        let mut back = info.back;

        if info.len == 0 {
            if front != NULL_ENTRY {
                return false;
            }
            if back != NULL_ENTRY {
                return false;
            }
            true
        } else {
            if front == NULL_ENTRY {
                return false;
            }
            if back == NULL_ENTRY {
                return false;
            }

            if self.get_node(front).prev_id != NULL_ENTRY {
                return false;
            }
            if self.get_node(back).next_id != NULL_ENTRY {
                return false;
            }

            let mut forwards = Vec::new();
            while front != NULL_ENTRY {
                forwards.push(front);
                front = self.get_node(front).next_id;
            }
            if forwards.len() != info.len as usize {
                return false;
            }

            let mut backwards = Vec::new();
            while back != NULL_ENTRY {
                backwards.push(back);
                back = self.get_node(back).prev_id;
            }
            if backwards.len() != info.len as usize {
                return false;
            }

            let backwards_reversed: Vec<u32> = backwards.iter().rev().cloned().collect();
            if forwards != backwards_reversed {
                return false;
            }

            forwards.sort_unstable();
            forwards.dedup();
            if forwards.len() != info.len as usize {
                return false;
            }
            true
        }
    }
}

impl<'a, SA, T> IntoIterator for &'a LinkedListMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone + 'static,
{
    type Item = LinkedListNode<T>;

    type IntoIter = Iter<'a, SA, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone + 'static,
{
    node_opt: Option<LinkedListNode<T>>,
    linked_list: &'a LinkedListMapper<SA, T>,
}

impl<'a, SA, T> Iter<'a, SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone,
{
    fn new(linked_list: &'a LinkedListMapper<SA, T>) -> Iter<'a, SA, T> {
        Iter {
            node_opt: linked_list.front(),
            linked_list,
        }
    }

    fn new_from_node_id(linked_list: &'a LinkedListMapper<SA, T>, node_id: u32) -> Iter<'a, SA, T> {
        Iter {
            node_opt: linked_list.get_node_by_id(node_id),
            linked_list,
        }
    }
}

impl<'a, SA, T> Iterator for Iter<'a, SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone + 'static,
{
    type Item = LinkedListNode<T>;

    #[inline]
    fn next(&mut self) -> Option<LinkedListNode<T>> {
        self.node_opt.as_ref()?;
        let node = self.node_opt.clone().unwrap();
        self.node_opt = self.linked_list.get_node_by_id(node.next_id);
        Some(node)
    }
}

impl<SA, T> TopEncodeMulti for LinkedListMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        for elem in self.iter() {
            elem.into_value().multi_encode_or_handle_err(output, h)?;
        }
        Ok(())
    }
}

impl<SA, T, U> CodecFrom<LinkedListMapper<SA, T>> for MultiValueEncoded<SA, U>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone,
    U: CodecFrom<T>,
{
}

impl<SA, T> TypeAbi for LinkedListMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone + TypeAbi,
{
    fn type_name() -> TypeName {
        crate::abi::type_name_variadic::<T>()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}
