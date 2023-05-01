use crate::{derive_external, Get, GetMut, Insert, Remove};
use slab::Slab;

derive_external! {
	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(SimpleCollectionRef, SimpleCollectionMut)]
	#[derive(Capacity, WithCapacity, Reserve, Len, Clear)]
	struct Slab<T>;
}

impl<T> Get<usize> for Slab<T> {
	fn get(&self, key: usize) -> Option<&Self::Item> {
		self.get(key)
	}
}

impl<T> GetMut<usize> for Slab<T> {
	fn get_mut(&mut self, key: usize) -> Option<&mut Self::Item> {
		self.get_mut(key)
	}
}

impl<T> Insert for Slab<T> {
	type Output = usize;

	fn insert(&mut self, element: T) -> usize {
		self.insert(element)
	}
}

impl<T> Remove<usize> for Slab<T> {
	fn remove(&mut self, key: usize) -> Option<T> {
		if self.contains(key) {
			Some(self.remove(key))
		} else {
			None
		}
	}
}
