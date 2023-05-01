use crate::{derive_external, Iter, MapIter, MapIterMut};
use alloc::collections::BTreeMap;

derive_external! {
	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(Keyed, KeyedRef, SimpleKeyedRef)]
	#[derive(Capacity, WithCapacity, Reserve, Len, Clear)]
	struct BTreeMap<K, V>;

	#[derive(Get, GetMut, GetKeyValue)]
	#[derive(MapInsert, Remove)]
	struct BTreeMap<K: Ord, V>;
}

impl<K, V> Iter for BTreeMap<K, V> {
	type Iter<'a> = alloc::collections::btree_map::Values<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.values()
	}
}

impl<K, V> MapIter for BTreeMap<K, V> {
	type Iter<'a> = alloc::collections::btree_map::Iter<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl<K, V> MapIterMut for BTreeMap<K, V> {
	type IterMut<'a> = alloc::collections::btree_map::IterMut<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut()
	}
}
