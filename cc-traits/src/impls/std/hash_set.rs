use crate::{derive_external, Iter, Remove};
use std::{borrow::Borrow, collections::HashSet, hash::Hash};

derive_external! {
	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(Capacity, WithCapacity, Reserve, Len, Clear)]
	struct HashSet<T>;

	#[derive(Get, Insert)]
	struct HashSet<T: Hash + Eq>;
}

impl<'a, Q, T: Hash + Eq> Remove<&'a Q> for HashSet<T>
where
	T: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn remove(&mut self, t: &'a Q) -> Option<T> {
		self.take(t)
	}
}

impl<T> Iter for HashSet<T> {
	type Iter<'a> = std::collections::hash_set::Iter<'a, T> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}
