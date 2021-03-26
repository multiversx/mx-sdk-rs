use super::{set_mapper, SetMapper, StorageClearable, StorageMapper};
use crate::api::{ErrorApi, StorageReadApi, StorageWriteApi};
use crate::storage;
use crate::types::BoxedBytes;
use alloc::vec::Vec;
use core::marker::PhantomData;
use elrond_codec::{TopDecode, TopEncode};

const MAPPED_STORAGE_VALUE_IDENTIFIER: &[u8] = b".storage";
type Keys<'a, SA, T> = set_mapper::Iter<'a, SA, T>;

pub struct MapStorageMapper<SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode + 'static,
	V: StorageMapper<SA> + StorageClearable,
{
	api: SA,
	main_key: BoxedBytes,
	keys_set: SetMapper<SA, K>,
	_phantom: core::marker::PhantomData<V>,
}

impl<SA, K, V> StorageMapper<SA> for MapStorageMapper<SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode,
	V: StorageMapper<SA> + StorageClearable,
{
	fn new(api: SA, main_key: BoxedBytes) -> Self {
		Self {
			api: api.clone(),
			main_key: main_key.clone(),
			keys_set: SetMapper::<SA, K>::new(api, main_key),
			_phantom: PhantomData,
		}
	}
}

impl<SA, K, V> StorageClearable for MapStorageMapper<SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode,
	V: StorageMapper<SA> + StorageClearable,
{
	fn clear(&mut self) {
		for mut value in self.values() {
			value.clear();
		}
		self.keys_set.clear();
	}
}

pub fn top_encode_to_vec<T: TopEncode>(obj: &T) -> Vec<u8> {
	let mut bytes = Vec::<u8>::new();
	obj.top_encode(&mut bytes).unwrap();
	bytes
}

impl<SA, K, V> MapStorageMapper<SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode,
	V: StorageMapper<SA> + StorageClearable,
{
	fn build_named_key(&self, name: &[u8], key: &K) -> BoxedBytes {
		let bytes = top_encode_to_vec(&key);
		BoxedBytes::from_concat(&[self.main_key.as_slice(), name, &bytes])
	}

	fn get_mapped_storage_value(&self, key: &K) -> V {
		<V as storage::mappers::StorageMapper<SA>>::new(
			self.api.clone(),
			self.build_named_key(MAPPED_STORAGE_VALUE_IDENTIFIER, key),
		)
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
			return Some(self.get_mapped_storage_value(&k));
		}
		None
	}

	/// Adds a default value for the key, if it is not already present.
	///
	/// If the map did not have this key present, `true` is returned.
	///
	/// If the map did have this value present, `false` is returned.
	pub fn insert_default(&mut self, k: K) -> bool {
		self.keys_set.insert(k)
	}

	/// Returns the value corresponding to the key.
	///
	/// If the key is not found, it is inserted and a new default value is returned.
	pub fn get_or_insert_default(&mut self, k: K) -> V {
		let value = self.get_mapped_storage_value(&k);
		self.keys_set.insert(k);
		value
	}

	/// Takes the value out of the entry, and returns it.
	pub fn remove(&mut self, k: &K) -> bool {
		if self.keys_set.remove(k) {
			self.get_mapped_storage_value(k).clear();
			return true;
		}
		false
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
	V: StorageMapper<SA> + StorageClearable,
{
	key_iter: Keys<'a, SA, K>,
	hash_map: &'a MapStorageMapper<SA, K, V>,
}

impl<'a, SA, K, V> Iter<'a, SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode + 'static,
	V: StorageMapper<SA> + StorageClearable,
{
	fn new(hash_map: &'a MapStorageMapper<SA, K, V>) -> Iter<'a, SA, K, V> {
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
	V: StorageMapper<SA> + StorageClearable,
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
	V: StorageMapper<SA> + StorageClearable,
{
	key_iter: Keys<'a, SA, K>,
	hash_map: &'a MapStorageMapper<SA, K, V>,
}

impl<'a, SA, K, V> Values<'a, SA, K, V>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	K: TopEncode + TopDecode + 'static,
	V: StorageMapper<SA> + StorageClearable,
{
	fn new(hash_map: &'a MapStorageMapper<SA, K, V>) -> Values<'a, SA, K, V> {
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
	V: StorageMapper<SA> + StorageClearable,
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
