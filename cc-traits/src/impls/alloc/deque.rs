use crate::{derive_external, Back, BackMut, Front, FrontMut, PopBack, PushBack};
use alloc::collections::VecDeque;

derive_external! {
	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(SimpleCollectionRef, SimpleCollectionMut)]
	#[derive(Capacity, WithCapacity, Reserve, Len, Clear)]
	struct VecDeque<T>;
}

impl<T> Front for VecDeque<T> {
	#[inline(always)]
	fn front(&self) -> Option<&T> {
		self.front()
	}
}

impl<T> FrontMut for VecDeque<T> {
	#[inline(always)]
	fn front_mut(&mut self) -> Option<&mut T> {
		self.front_mut()
	}
}

impl<T> Back for VecDeque<T> {
	#[inline(always)]
	fn back(&self) -> Option<&T> {
		self.back()
	}
}

impl<T> BackMut for VecDeque<T> {
	#[inline(always)]
	fn back_mut(&mut self) -> Option<&mut T> {
		self.back_mut()
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
