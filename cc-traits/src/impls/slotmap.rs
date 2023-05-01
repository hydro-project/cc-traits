use slotmap::{DenseSlotMap, HopSlotMap, Key, SecondaryMap, SlotMap, SparseSecondaryMap};

use crate::{derive_external, Get, GetMut, Insert, Iter, IterMut, MapInsert, Remove};

derive_external! {
	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(Keyed, KeyedRef)]
	#[derive(Capacity, WithCapacity, Reserve, Len, Clear)]
	struct SlotMap<K: Key, V>;

	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(Keyed, KeyedRef)]
	#[derive(Capacity, WithCapacity, Reserve, Len, Clear)]
	struct DenseSlotMap<K: Key, V>;

	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(Keyed, KeyedRef)]
	#[derive(Capacity, WithCapacity, Reserve, Len, Clear)]
	struct HopSlotMap<K: Key, V>;

	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(Keyed, KeyedRef)]
	#[derive(Capacity, WithCapacity, Reserve, Len, Clear)]
	struct SecondaryMap<K: Key, V>;

	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(Keyed, KeyedRef)]
	#[derive(Capacity, WithCapacity, Reserve, Len, Clear)]
	struct SparseSecondaryMap<K: Key, V>;
}

impl<K: Key, V> Insert for SlotMap<K, V> {
	type Output = K;

	#[inline(always)]
	fn insert(&mut self, element: Self::Item) -> Self::Output {
		self.insert(element)
	}
}
impl<K: Key, V> Insert for DenseSlotMap<K, V> {
	type Output = K;

	#[inline(always)]
	fn insert(&mut self, element: Self::Item) -> Self::Output {
		self.insert(element)
	}
}
impl<K: Key, V> Insert for HopSlotMap<K, V> {
	type Output = K;

	#[inline(always)]
	fn insert(&mut self, element: Self::Item) -> Self::Output {
		self.insert(element)
	}
}

impl<K: Key, V> MapInsert<K> for SecondaryMap<K, V> {
	type Output = Option<V>;

	#[inline(always)]
	fn insert(&mut self, key: K, value: Self::Item) -> Self::Output {
		self.insert(key, value)
	}
}
impl<K: Key, V> MapInsert<K> for SparseSecondaryMap<K, V> {
	type Output = Option<V>;

	#[inline(always)]
	fn insert(&mut self, key: K, value: Self::Item) -> Self::Output {
		self.insert(key, value)
	}
}
impl<K: Key, V> Remove<K> for SecondaryMap<K, V> {
	fn remove(&mut self, key: K) -> Option<Self::Item> {
		self.remove(key)
	}
}
impl<K: Key, V> Remove<K> for SparseSecondaryMap<K, V> {
	fn remove(&mut self, key: K) -> Option<Self::Item> {
		self.remove(key)
	}
}

impl<K: Key, V> Iter for SlotMap<K, V> {
	type Iter<'a> = slotmap::basic::Values<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.values()
	}
}
impl<K: Key, V> Iter for DenseSlotMap<K, V> {
	type Iter<'a> = slotmap::dense::Values<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.values()
	}
}
impl<K: Key, V> Iter for HopSlotMap<K, V> {
	type Iter<'a> = slotmap::hop::Values<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.values()
	}
}
impl<K: Key, V> Iter for SecondaryMap<K, V> {
	type Iter<'a> = slotmap::secondary::Values<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.values()
	}
}
impl<K: Key, V> Iter for SparseSecondaryMap<K, V> {
	type Iter<'a> = slotmap::sparse_secondary::Values<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.values()
	}
}

impl<K: Key, V> IterMut for SlotMap<K, V> {
	type IterMut<'a> = slotmap::basic::ValuesMut<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.values_mut()
	}
}
impl<K: Key, V> IterMut for DenseSlotMap<K, V> {
	type IterMut<'a> = slotmap::dense::ValuesMut<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.values_mut()
	}
}
impl<K: Key, V> IterMut for HopSlotMap<K, V> {
	type IterMut<'a> = slotmap::hop::ValuesMut<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.values_mut()
	}
}
impl<K: Key, V> IterMut for SecondaryMap<K, V> {
	type IterMut<'a> = slotmap::secondary::ValuesMut<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.values_mut()
	}
}
impl<K: Key, V> IterMut for SparseSecondaryMap<K, V> {
	type IterMut<'a> = slotmap::sparse_secondary::ValuesMut<'a, K, V> where Self: 'a;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.values_mut()
	}
}

impl<K: Key, V> Get<K> for SlotMap<K, V> {
	fn get(&self, key: K) -> Option<Self::ItemRef<'_>> {
		self.get(key)
	}
}
impl<K: Key, V> Get<K> for DenseSlotMap<K, V> {
	fn get(&self, key: K) -> Option<Self::ItemRef<'_>> {
		self.get(key)
	}
}
impl<K: Key, V> Get<K> for HopSlotMap<K, V> {
	fn get(&self, key: K) -> Option<Self::ItemRef<'_>> {
		self.get(key)
	}
}
impl<K: Key, V> Get<K> for SecondaryMap<K, V> {
	fn get(&self, key: K) -> Option<Self::ItemRef<'_>> {
		self.get(key)
	}
}
impl<K: Key, V> Get<K> for SparseSecondaryMap<K, V> {
	fn get(&self, key: K) -> Option<Self::ItemRef<'_>> {
		self.get(key)
	}
}

impl<K: Key, V> GetMut<K> for SlotMap<K, V> {
	fn get_mut(&mut self, key: K) -> Option<Self::ItemMut<'_>> {
		self.get_mut(key)
	}
}
impl<K: Key, V> GetMut<K> for DenseSlotMap<K, V> {
	fn get_mut(&mut self, key: K) -> Option<Self::ItemMut<'_>> {
		self.get_mut(key)
	}
}
impl<K: Key, V> GetMut<K> for HopSlotMap<K, V> {
	fn get_mut(&mut self, key: K) -> Option<Self::ItemMut<'_>> {
		self.get_mut(key)
	}
}
impl<K: Key, V> GetMut<K> for SecondaryMap<K, V> {
	fn get_mut(&mut self, key: K) -> Option<Self::ItemMut<'_>> {
		self.get_mut(key)
	}
}
impl<K: Key, V> GetMut<K> for SparseSecondaryMap<K, V> {
	fn get_mut(&mut self, key: K) -> Option<Self::ItemMut<'_>> {
		self.get_mut(key)
	}
}
