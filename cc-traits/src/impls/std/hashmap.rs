use crate::macros::derive_external;
use crate::{Clear, GetKeyValue, Iter, MapInsert, MapIter, MapIterMut, Remove};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

derive_external! {
	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(SimpleCollectionRef, SimpleCollectionMut)]
	#[derive(Keyed, KeyedRef, SimpleKeyedRef)]
	#[derive(Len)]
	struct HashMap<K, V> {}

	#[derive(Get, GetMut)]
	struct HashMap<K: Hash + Eq, V> {}
}

impl<'a, Q, K: Hash + Eq, V> GetKeyValue<&'a Q> for HashMap<K, V>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn get_key_value(&self, key: &'a Q) -> Option<(&K, &V)> {
		self.get_key_value(key)
	}
}

impl<K: Hash + Eq, V> MapInsert<K> for HashMap<K, V> {
	type Output = Option<V>;

	#[inline(always)]
	fn insert(&mut self, key: K, value: V) -> Option<V> {
		self.insert(key, value)
	}
}

impl<'a, Q, K: Hash + Eq, V> Remove<&'a Q> for HashMap<K, V>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn remove(&mut self, key: &'a Q) -> Option<V> {
		self.remove(key)
	}
}

impl<K, V> Clear for HashMap<K, V> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}

impl<K, V> Iter for HashMap<K, V> {
	type Iter<'a> = std::collections::hash_map::Values<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.values()
	}
}

impl<K, V> MapIter for HashMap<K, V> {
	type Iter<'a> = std::collections::hash_map::Iter<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl<K, V> MapIterMut for HashMap<K, V> {
	type IterMut<'a> = std::collections::hash_map::IterMut<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut()
	}
}
