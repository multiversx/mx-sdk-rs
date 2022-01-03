const BUFFER_1_SIZE: usize = 32;
const BUFFER_2_SIZE: usize = 32;

/// A static mutable buffer acting as temporary storage for certain operations,
/// such as handling temporary big uint representations.
/// Highly unsafe, use with caution.
///
/// It doesn't matter what we initialize with, since it needs to be cleared before each use.
static mut BUFFER_1: [u8; BUFFER_1_SIZE] = [0u8; BUFFER_1_SIZE];

/// The second buffer is for when the first one is busy with something else.
static mut BUFFER_2: [u8; BUFFER_2_SIZE] = [0u8; BUFFER_2_SIZE];

pub(crate) unsafe fn clear_buffer_1() {
    core::ptr::write_bytes(BUFFER_1.as_mut_ptr(), 0u8, BUFFER_1_SIZE);
}

pub(crate) unsafe fn buffer_1_ptr() -> *mut u8 {
    BUFFER_1.as_mut_ptr()
}

pub(crate) unsafe fn buffer_2_ptr() -> *mut u8 {
    BUFFER_2.as_mut_ptr()
}
