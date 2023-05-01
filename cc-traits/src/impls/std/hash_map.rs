use crate::{derive_external, Iter, IterMut, MapIter, MapIterMut};

use std::collections::HashMap;
use std::hash::Hash;

derive_external! {
	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(Keyed, KeyedRef)]
	#[derive(Capacity, WithCapacity, Reserve, Len, Clear)]
	struct HashMap<K, V>;

	#[derive(Get, GetMut, GetKeyValue)]
	#[derive(MapInsert, Remove)]
	struct HashMap<K: Hash + Eq, V>;
}

impl<K, V> Iter for HashMap<K, V> {
	type Iter<'a> = std::collections::hash_map::Values<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.values()
	}
}

impl<K, V> IterMut for HashMap<K, V> {
	type IterMut<'a> = std::collections::hash_map::ValuesMut<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.values_mut()
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
