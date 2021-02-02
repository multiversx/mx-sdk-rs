use super::{set_mapper, SetMapper, StorageMapper};
use crate::api::{ErrorApi, StorageReadApi, StorageWriteApi};
use crate::storage::{storage_get, storage_set};
use crate::types::BoxedBytes;
use alloc::vec::Vec;
use core::marker::PhantomData;
use elrond_codec::{TopDecode, TopEncode};

const MAPPED_VALUE_IDENTIFIER: &[u8] = b".mapped";
type Keys<'a, SA, T> = set_mapper::Iter<'a, SA, T>;

pub struct MapMapper<SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode + 'static,
	V: TopEncode + TopDecode + 'static,
{
	api: SA,
	main_key: BoxedBytes,
	keys_set: SetMapper<SA, K>,
	_phantom: core::marker::PhantomData<V>,
}

impl<SA, K, V> StorageMapper<SA> for MapMapper<SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode,
	V: TopEncode + TopDecode,
{
	fn new(api: SA, main_key: BoxedBytes) -> Self {
		MapMapper {
			api: api.clone(),
			main_key: main_key.clone(),
			keys_set: SetMapper::<SA, K>::new(api, main_key),
			_phantom: PhantomData,
		}
	}
}

pub fn top_encode_to_vec<T: TopEncode>(obj: &T) -> Vec<u8> {
	let mut bytes = Vec::<u8>::new();
	obj.top_encode(&mut bytes).unwrap();
	bytes
}

impl<SA, K, V> MapMapper<SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode,
	V: TopEncode + TopDecode,
{
	fn build_named_key(&self, name: &[u8], key: &K) -> BoxedBytes {
		let bytes = top_encode_to_vec(&key);
		BoxedBytes::from_concat(&[self.main_key.as_slice(), name, &bytes])
	}

	fn get_mapped_value(&self, key: &K) -> V {
		storage_get(
			self.api.clone(),
			self.build_named_key(MAPPED_VALUE_IDENTIFIER, key)
				.as_slice(),
		)
	}

	fn set_mapped_value(&self, key: &K, value: &V) {
		storage_set(
			self.api.clone(),
			self.build_named_key(MAPPED_VALUE_IDENTIFIER, key)
				.as_slice(),
			&value,
		);
	}

	fn clear_mapped_value(&self, key: &K) {
		storage_set(
			self.api.clone(),
			self.build_named_key(MAPPED_VALUE_IDENTIFIER, key)
				.as_slice(),
			&BoxedBytes::empty(),
		);
	}

	pub fn is_empty(&self) -> bool {
		self.keys_set.is_empty()
	}

	pub fn len(&self) -> usize {
		self.keys_set.len()
	}

	pub fn contains_key(&self, k: &K) -> bool {
		self.keys_set.contains(k)
	}

	pub fn get(&self, k: &K) -> Option<V> {
		if self.keys_set.contains(k) {
			return Some(self.get_mapped_value(&k));
		}
		None
	}

	pub fn insert(&mut self, k: K, v: V) -> Option<V> {
		let old_value = self.get(&k);
		self.set_mapped_value(&k, &v);
		self.keys_set.insert(k);
		old_value
	}

	pub fn remove(&mut self, k: &K) -> Option<V> {
		if self.keys_set.remove(k) {
			let value = self.get_mapped_value(k);
			self.clear_mapped_value(k);
			return Some(value);
		}
		None
	}

	pub fn keys(&self) -> Keys<SA, K> {
		self.keys_set.iter()
	}

	pub fn values(&self) -> Values<SA, K, V> {
		Values::new(self)
	}

	pub fn iter(&self) -> Iter<SA, K, V> {
		Iter::new(self)
	}
}

pub struct Iter<'a, SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode + 'static,
	V: TopEncode + TopDecode + 'static,
{
	key_iter: Keys<'a, SA, K>,
	hash_map: &'a MapMapper<SA, K, V>,
}

impl<'a, SA, K, V> Iter<'a, SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode + 'static,
	V: TopEncode + TopDecode + 'static,
{
	fn new(hash_map: &'a MapMapper<SA, K, V>) -> Iter<'a, SA, K, V> {
		Iter {
			key_iter: hash_map.keys(),
			hash_map,
		}
	}
}

impl<'a, SA, K, V> Iterator for Iter<'a, SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode + 'static,
	V: TopEncode + TopDecode + 'static,
{
	type Item = (K, V);

	#[inline]
	fn next(&mut self) -> Option<(K, V)> {
		if let Some(key) = self.key_iter.next() {
			let value = self.hash_map.get(&key).unwrap();
			return Some((key, value));
		}
		None
	}
}

pub struct Values<'a, SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode + 'static,
	V: TopEncode + TopDecode + 'static,
{
	key_iter: Keys<'a, SA, K>,
	hash_map: &'a MapMapper<SA, K, V>,
}

impl<'a, SA, K, V> Values<'a, SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode + 'static,
	V: TopEncode + TopDecode + 'static,
{
	fn new(hash_map: &'a MapMapper<SA, K, V>) -> Values<'a, SA, K, V> {
		Values {
			key_iter: hash_map.keys(),
			hash_map,
		}
	}
}

impl<'a, SA, K, V> Iterator for Values<'a, SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode + 'static,
	V: TopEncode + TopDecode + 'static,
{
	type Item = V;

	#[inline]
	fn next(&mut self) -> Option<V> {
		if let Some(key) = self.key_iter.next() {
			let value = self.hash_map.get(&key).unwrap();
			return Some(value);
		}
		None
	}
}
