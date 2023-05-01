use crate::{derive_external, Get, GetMut, PopBack, PopFront, PushBack, PushFront};
use alloc::collections::VecDeque;

derive_external! {
	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(SimpleCollectionRef, SimpleCollectionMut)]
	#[derive(Capacity, WithCapacity, Reserve, Len, Clear)]
	struct VecDeque<T>;
}

impl<T> Get<usize> for VecDeque<T> {
	#[inline(always)]
	fn get(&self, index: usize) -> Option<&T> {
		self.get(index)
	}
}

impl<T> GetMut<usize> for VecDeque<T> {
	#[inline(always)]
	fn get_mut(&mut self, index: usize) -> Option<&mut T> {
		self.get_mut(index)
	}
}

impl<T> PushFront for VecDeque<T> {
	type Output = ();

	#[inline(always)]
	fn push_front(&mut self, t: T) {
		self.push_front(t)
	}
}

impl<T> PopFront for VecDeque<T> {
	#[inline(always)]
	fn pop_front(&mut self) -> Option<T> {
		self.pop_front()
	}
}

impl<T> PushBack for VecDeque<T> {
	type Output = ();

	#[inline(always)]
	fn push_back(&mut self, t: T) {
		self.push_back(t)
	}
}

impl<T> PopBack for VecDeque<T> {
	#[inline(always)]
	fn pop_back(&mut self) -> Option<T> {
		self.pop_back()
	}
}
