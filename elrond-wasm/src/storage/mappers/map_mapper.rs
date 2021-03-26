use super::{set_mapper, SetMapper, StorageClearable, StorageMapper};
use crate::api::{ErrorApi, StorageReadApi, StorageWriteApi};
use crate::storage::{storage_get, storage_set};
use crate::types::BoxedBytes;
use core::marker::PhantomData;
use elrond_codec::{top_encode_to_vec, TopDecode, TopEncode};

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

impl<SA, K, V> StorageClearable for MapMapper<SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode,
	V: TopEncode + TopDecode,
{
	fn clear(&mut self) {
		for key in self.keys_set.iter() {
			self.clear_mapped_value(&key);
		}
		self.keys_set.clear();
	}
}

impl<SA, K, V> MapMapper<SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode,
	V: TopEncode + TopDecode,
{
	fn build_named_key(&self, name: &[u8], key: &K) -> BoxedBytes {
		let bytes = top_encode_to_vec(&key).unwrap();
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

	/// Returns `true` if the map contains no elements.
	pub fn is_empty(&self) -> bool {
		self.keys_set.is_empty()
	}

	/// Returns the number of elements in the map.
	pub fn len(&self) -> usize {
		self.keys_set.len()
	}

	/// Returns `true` if the map contains a value for the specified key.
	pub fn contains_key(&self, k: &K) -> bool {
		self.keys_set.contains(k)
	}

	/// Gets a reference to the value in the entry.
	pub fn get(&self, k: &K) -> Option<V> {
		if self.keys_set.contains(k) {
			return Some(self.get_mapped_value(&k));
		}
		None
	}

	/// Sets the value of the entry, and returns the entry's old value.
	pub fn insert(&mut self, k: K, v: V) -> Option<V> {
		let old_value = self.get(&k);
		self.set_mapped_value(&k, &v);
		self.keys_set.insert(k);
		old_value
	}

	/// Takes the value out of the entry, and returns it.
	pub fn remove(&mut self, k: &K) -> Option<V> {
		if self.keys_set.remove(k) {
			let value = self.get_mapped_value(k);
			self.clear_mapped_value(k);
			return Some(value);
		}
		None
	}

	/// An iterator visiting all keys in arbitrary order.
	/// The iterator element type is `&'a K`.
	pub fn keys(&self) -> Keys<SA, K> {
		self.keys_set.iter()
	}

	/// An iterator visiting all values in arbitrary order.
	/// The iterator element type is `&'a V`.
	pub fn values(&self) -> Values<SA, K, V> {
		Values::new(self)
	}

	/// An iterator visiting all key-value pairs in arbitrary order.
	/// The iterator element type is `(&'a K, &'a V)`.
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
