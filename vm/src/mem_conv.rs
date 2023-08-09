use multiversx_chain_vm_executor::{MemLength, MemPtr};

pub fn with_mem_ptr<F, R>(bytes: &[u8], f: F) -> R
where
    F: FnOnce(MemPtr, MemLength) -> R,
{
    f(bytes.as_ptr() as MemPtr, bytes.len() as MemLength)
}

#[allow(clippy::needless_pass_by_ref_mut)]
pub fn with_mem_ptr_mut<F, R>(bytes: &mut [u8], f: F) -> R
where
    F: FnOnce(MemPtr, MemLength) -> R,
{
    f(bytes.as_ptr() as MemPtr, bytes.len() as MemLength)
}

/// Interprets an offset and length (both isize) as a byte slice.
///
/// # Safety
///
/// Should only be called with arguments that originate from `with_mem_ptr`.
pub unsafe fn with_bytes<F, R>(offset: MemPtr, length: MemLength, f: F) -> R
where
    F: FnOnce(&[u8]) -> R,
{
    let bytes = std::ptr::slice_from_raw_parts(offset as *const u8, length as usize);
    f(&*bytes)
}

/// Interprets an offset and length (both isize) as a mutable byte slice.
///
/// # Safety
///
/// Should only be called with arguments that originate from `with_mem_ptr_mut`.
pub unsafe fn with_bytes_mut<F, R>(offset: MemPtr, length: MemLength, f: F) -> R
where
    F: FnOnce(&mut [u8]) -> R,
{
    let bytes = std::ptr::slice_from_raw_parts_mut(offset as *mut u8, length as usize);
    f(&mut *bytes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mem_ptr_conversion() {
        assert_mem_conv_sound(vec![]);
        assert_mem_conv_sound(vec![1]);
        assert_mem_conv_sound(vec![1, 2, 3]);
    }

    fn assert_mem_conv_sound(data: Vec<u8>) {
        let cloned = with_mem_ptr(data.as_slice(), |offset, length| unsafe {
            with_bytes(offset, length, |bytes| bytes.to_vec())
        });
        assert_eq!(data, cloned);
    }

    #[test]
    fn test_mem_ptr_mut() {
        let mut data = vec![1, 2, 3];
        with_mem_ptr_mut(data.as_mut_slice(), |offset, length| unsafe {
            with_bytes_mut(offset, length, |bytes| {
                for b in bytes {
                    *b += 1;
                }
            })
        });
        assert_eq!(data, vec![2, 3, 4]);
    }
}
