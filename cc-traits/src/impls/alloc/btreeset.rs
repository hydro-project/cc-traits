use crate::{derive_external, Iter, Remove};
use alloc::collections::BTreeSet;
use core::borrow::Borrow;

derive_external! {
	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(SimpleCollectionRef, SimpleCollectionMut)]
	#[derive(Capacity, WithCapacity, Reserve, Len, Clear)]
	struct BTreeSet<T>;

	#[derive(Get, Insert)]
	struct BTreeSet<T: Ord>;
}

impl<'a, Q, T: Ord> Remove<&'a Q> for BTreeSet<T>
where
	T: Borrow<Q>,
	Q: Ord + ?Sized,
{
	#[inline(always)]
	fn remove(&mut self, t: &'a Q) -> Option<T> {
		self.take(t)
	}
}

impl<T> Iter for BTreeSet<T> {
	type Iter<'a> = alloc::collections::btree_set::Iter<'a, T> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}
