use super::{StorageClearable, StorageMapper};
use crate::abi::{TypeAbi, TypeDescriptionContainer, TypeName};
use crate::api::{EndpointFinishApi, ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi};
use crate::io::EndpointResult;
use crate::storage::{storage_get, storage_set, StorageKey};
use crate::types::{BoxedBytes, MultiResultVec};
use alloc::vec::Vec;
use core::marker::PhantomData;
use elrond_codec::elrond_codec_derive::{
    NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode, TopEncodeOrDefault,
};
use elrond_codec::{
    DecodeDefault, EncodeDefault, NestedDecode, NestedEncode, TopDecode, TopEncode,
};
use elrond_wasm_derive::TypeAbi;
use storage_get::storage_get_len;

const NULL_ENTRY: u32 = 0;
const INFO_IDENTIFIER: &[u8] = b".info";
const NODE_IDENTIFIER: &[u8] = b".node";

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Clone, Copy)]
pub struct LinkedListNode<T: NestedEncode + NestedDecode + TopEncode + TopDecode> {
    pub value: T,
    pub(crate) node_id: u32,
    pub(crate) next_id: u32,
    pub(crate) prev_id: u32,
}

impl<T: NestedEncode + NestedDecode + TopEncode + TopDecode> LinkedListNode<T> {
    pub fn get_node_id(&self) -> u32 {
        self.node_id
    }

    pub fn get_next_node_id(&self) -> u32 {
        self.node_id
    }

    pub fn get_prev_node_id(&self) -> u32 {
        self.prev_id
    }
}

#[derive(TopEncodeOrDefault, TopDecodeOrDefault, PartialEq, Clone, Copy)]
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
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    api: SA,
    base_key: StorageKey<SA>,
    _phantom: core::marker::PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for LinkedListMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn new(api: SA, base_key: StorageKey<SA>) -> Self {
        LinkedListMapper {
            api,
            base_key,
            _phantom: PhantomData,
        }
    }
}

impl<SA, T> StorageClearable for LinkedListMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
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
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
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
        storage_get(self.api.clone(), &self.build_name_key(INFO_IDENTIFIER))
    }

    fn set_info(&mut self, value: LinkedListInfo) {
        storage_set(
            self.api.clone(),
            &self.build_name_key(INFO_IDENTIFIER),
            &value,
        );
    }

    fn get_node(&self, node_id: u32) -> LinkedListNode<T> {
        storage_get(
            self.api.clone(),
            &self.build_node_id_named_key(NODE_IDENTIFIER, node_id),
        )
    }

    fn is_empty_node(&self, node_id: u32) -> bool {
        storage_get_len(
            self.api.clone(),
            &self.build_node_id_named_key(NODE_IDENTIFIER, node_id),
        ) == 0
    }

    fn set_node(&mut self, node_id: u32, item: &LinkedListNode<T>) {
        storage_set(
            self.api.clone(),
            &self.build_node_id_named_key(NODE_IDENTIFIER, node_id),
            item,
        );
    }

    fn clear_node(&mut self, node_id: u32) {
        storage_set(
            self.api.clone(),
            &self.build_node_id_named_key(NODE_IDENTIFIER, node_id),
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

    pub fn pop_back(&mut self) -> Option<T> {
        self.remove_node_by_id(self.get_info().back)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.remove_node_by_id(self.get_info().front)
    }

    pub fn push_after(
        &mut self,
        node: &mut LinkedListNode<T>,
        element: T,
    ) -> Option<LinkedListNode<T>> {
        self.push_after_node_id(node.node_id, element)
    }

    pub fn push_before(
        &mut self,
        node: &mut LinkedListNode<T>,
        element: T,
    ) -> Option<LinkedListNode<T>> {
        self.push_before_node_id(node.node_id, element)
    }

    pub fn push_before_node_id(&mut self, node_id: u32, element: T) -> Option<LinkedListNode<T>> {
        if self.is_empty_node(node_id) {
            return None;
        }

        let mut node = self.get_node(node_id);
        let mut info = self.get_info();
        let new_node_id = info.generate_new_node_id();

        let new_node_prev_id = node.prev_id;
        node.prev_id = new_node_id;
        self.set_node(node.node_id, &node);

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
        if self.is_empty_node(node_id) {
            return None;
        }

        let mut node = self.get_node(node_id);
        let mut info = self.get_info();
        let new_node_id = info.generate_new_node_id();

        let new_node_next_id = node.next_id;
        node.next_id = new_node_id;
        self.set_node(node.node_id, &node);

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

    pub fn remove_node_by_id(&mut self, node_id: u32) -> Option<T> {
        if self.is_empty_node(node_id) {
            return None;
        }

        let node = self.get_node(node_id);
        self.remove_node(&node);
        Some(node.value)
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

pub struct Iter<'a, SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    node_id: u32,
    linked_list: &'a LinkedListMapper<SA, T>,
}

impl<'a, SA, T> Iter<'a, SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    fn new(linked_list: &'a LinkedListMapper<SA, T>) -> Iter<'a, SA, T> {
        Iter {
            node_id: linked_list.get_info().front,
            linked_list,
        }
    }

    fn new_from_node_id(linked_list: &'a LinkedListMapper<SA, T>, node_id: u32) -> Iter<'a, SA, T> {
        Iter {
            node_id,
            linked_list,
        }
    }
}

impl<'a, SA, T> Iterator for Iter<'a, SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let current_node_id = self.node_id;

        if current_node_id == NULL_ENTRY {
            return None;
        }

        let node = self.linked_list.get_node(current_node_id);
        self.node_id = node.next_id;
        Some(node.value)
    }
}

impl<SA, T> EndpointResult for LinkedListMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + EndpointResult,
{
    type DecodeAs = MultiResultVec<T::DecodeAs>;

    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        let v: Vec<T> = self.iter().collect();
        MultiResultVec::<T>::from(v).finish(api);
    }
}

impl<SA, T> TypeAbi for LinkedListMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + TypeAbi,
{
    fn type_name() -> TypeName {
        crate::types::MultiResultVec::<T>::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_multi_arg_or_result() -> bool {
        true
    }
}
