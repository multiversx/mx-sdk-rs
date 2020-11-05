use alloc::boxed::Box;
use alloc::vec::Vec;

#[inline(never)]
pub fn boxed_slice_into_vec<T>(mut bs: Box<[T]>) -> Vec<T> {
	let l = bs.len();
	if l == 0 {
		return Vec::new();
	}
	let ptr = &mut bs[0] as *mut T;
	core::mem::forget(bs);
	unsafe { Vec::from_raw_parts(ptr, l, l) }
}

#[inline(never)]
pub fn vec_into_boxed_slice<T>(v: Vec<T>) -> Box<[T]> {
	v.into_boxed_slice()
}

#[cfg(test)]
pub mod tests {
	use super::*;

	#[test]
	fn test_boxed_slice_into_vec_0() {
		let bs = Box::<[u8]>::from([]);
		let v = boxed_slice_into_vec(bs);
		assert_eq!(v, Vec::<u8>::new());
	}

	#[test]
	fn test_boxed_slice_into_vec_1() {
		let bs = Box::<[u8]>::from([1, 2, 3]);
		let v = boxed_slice_into_vec(bs);
		assert_eq!(v, [1u8, 2, 3].to_vec());
	}

	#[test]
	fn test_vec_into_boxed_slice_0() {
		let v = Vec::<u8>::new();
		let bs = vec_into_boxed_slice(v);
		assert_eq!(bs, Box::<[u8]>::from([]));
	}

	#[test]
	fn test_vec_into_boxed_slice_1() {
		let v = [1u8, 2, 3].to_vec();
		let bs = vec_into_boxed_slice(v);
		assert_eq!(bs, Box::<[u8]>::from([1, 2, 3]));
	}
}
