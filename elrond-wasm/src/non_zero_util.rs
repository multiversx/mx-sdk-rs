use core::iter::Iterator;
use core::num::NonZeroUsize;

/// This is safe because 1 != 0.
#[inline]
pub const fn non_zero_usize_one() -> NonZeroUsize {
	unsafe { NonZeroUsize::new_unchecked(1) }
}

/// This is safe because adding 1 to a positive number makes it greater than one.
#[inline]
pub const fn non_zero_usize_from_n_plus_1(n: usize) -> NonZeroUsize {
	unsafe { NonZeroUsize::new_unchecked(n + 1) }
}

/// This is safe because adding a non-zero number with a positive one yields a non-zero number.
#[inline]
pub const fn non_zero_usize_plus(a: NonZeroUsize, b: usize) -> NonZeroUsize {
	unsafe { NonZeroUsize::new_unchecked(a.get() + b) }
}

/// Iterator that can give us a range of NonZeroUsize.
pub struct NonZeroUsizeIterator {
	prev_num: usize,
	limit: usize,
}

impl NonZeroUsizeIterator {
	/// Creates an Iterator that runs from 1 to n, inclusively.
	/// The iterator will produce n numbers,
	/// e.g. for 3 it will produce [1, 2, 3].
	pub fn from_1_to_n(n: usize) -> Self {
		NonZeroUsizeIterator {
			prev_num: 0,
			limit: n,
		}
	}
}

impl Iterator for NonZeroUsizeIterator {
	type Item = NonZeroUsize;

	fn next(&mut self) -> Option<NonZeroUsize> {
		if self.prev_num >= self.limit {
			return None;
		}

		self.prev_num += 1;
		unsafe { Some(NonZeroUsize::new_unchecked(self.prev_num)) }
	}
}

#[cfg(test)]
pub mod tests {
	use super::*;
	use alloc::vec::Vec;

	#[test]
	fn test_iter_0() {
		let mut v = Vec::<usize>::new();
		for nz in NonZeroUsizeIterator::from_1_to_n(0) {
			v.push(nz.get());
		}

		assert_eq!(v, [].to_vec());
	}

	#[test]
	fn test_iter_1() {
		let mut v = Vec::<usize>::new();
		for nz in NonZeroUsizeIterator::from_1_to_n(1) {
			v.push(nz.get());
		}

		assert_eq!(v, [1].to_vec());
	}

	#[test]
	fn test_iter_3() {
		let mut v = Vec::<usize>::new();
		for nz in NonZeroUsizeIterator::from_1_to_n(3) {
			v.push(nz.get());
		}

		assert_eq!(v, [1, 2, 3].to_vec());
	}

	const ONE: NonZeroUsize = non_zero_usize_one();
	const TWO: NonZeroUsize = non_zero_usize_from_n_plus_1(1);
	const TEN: NonZeroUsize = non_zero_usize_plus(ONE, 9);

	#[test]
	fn test_const() {
		assert_eq!(ONE.get(), 1);
		assert_eq!(TWO.get(), 2);
		assert_eq!(TEN.get(), 10);
	}
}
