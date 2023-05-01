use crate::{derive_external, Front, PopFront, PushBack};
use alloc::collections::BinaryHeap;

derive_external! {
	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(Capacity, WithCapacity, Reserve, Len, Clear)]
	struct BinaryHeap<T>;
}

impl<T> Front for BinaryHeap<T> {
	#[inline(always)]
	fn front(&self) -> Option<&T> {
		self.peek()
	}
}

// impl<T: Ord> FrontMut for BinaryHeap<T> {
// 	#[inline(always)]
// 	fn front_mut(&mut self) -> Option<&mut T> {
// 		self.peek_mut().as_deref_mut()
// 	}
// }

impl<T: Ord> PopFront for BinaryHeap<T> {
	#[inline(always)]
	fn pop_front(&mut self) -> Option<T> {
		self.pop()
	}
}

impl<T: Ord> PushBack for BinaryHeap<T> {
	type Output = ();

	#[inline(always)]
	fn push_back(&mut self, t: T) {
		self.push(t)
	}
}
