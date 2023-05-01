use crate::{derive_external, Get, GetMut, Iter, IterMut, PopBack, PushBack, Remove};
use alloc::vec::Vec;

derive_external! {
	#[derive(Collection, CollectionRef, CollectionMut)]
	#[derive(Capacity, WithCapacity, Reserve, Len, Clear)]
	struct Vec<T>;
}

impl<T> Get<usize> for Vec<T> {
	#[inline(always)]
	fn get(&self, index: usize) -> Option<&T> {
		self.as_slice().get(index)
	}
}

impl<T> GetMut<usize> for Vec<T> {
	#[inline(always)]
	fn get_mut(&mut self, index: usize) -> Option<&mut T> {
		self.as_mut_slice().get_mut(index)
	}
}

impl<T> PushBack for Vec<T> {
	type Output = ();

	#[inline(always)]
	fn push_back(&mut self, t: T) {
		self.push(t)
	}
}

impl<T> PopBack for Vec<T> {
	#[inline(always)]
	fn pop_back(&mut self) -> Option<T> {
		self.pop()
	}
}

impl<T> Remove<usize> for Vec<T> {
	#[inline(always)]
	fn remove(&mut self, index: usize) -> Option<T> {
		if index < self.len() {
			Some(self.remove(index))
		} else {
			None
		}
	}
}
impl<T> Iter for Vec<T> {
	type Iter<'a> = core::slice::Iter<'a, T> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.as_slice().iter()
	}
}

impl<T> IterMut for Vec<T> {
	type IterMut<'a> = core::slice::IterMut<'a, T> where Self: 'a;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.as_mut_slice().iter_mut()
	}
}
