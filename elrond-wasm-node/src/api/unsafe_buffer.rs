use core::arch::wasm32;
use elrond_wasm::api::ErrorApi;

const BUFFER_SIZE: usize = 10000;

/// A static mutable buffer acting as temporary storage for certain operations,
/// such as handling temporary big uint representations.
/// Highly unsafe, use with caution.
///
/// It doesn't matter what we initialize with, since it needs to be cleared before each use.
static mut BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

pub(crate) unsafe fn clear_buffer() {
    core::ptr::write_bytes(BUFFER.as_mut_ptr(), 0u8, BUFFER_SIZE);
}

pub(crate) unsafe fn buffer_ptr() -> *mut u8 {
    BUFFER.as_mut_ptr()
}

fn ptr_to_string(p: *const u8) -> [u8; 8] {
    static CHARS: &'static [u8] = b"0123456789abcdef";
    let v = ptr_to_bytes(p);
    [
        CHARS[(v[0] >> 4) as usize],
        CHARS[(v[0] & 0xf) as usize],
        CHARS[(v[1] >> 4) as usize],
        CHARS[(v[1] & 0xf) as usize],
        CHARS[(v[2] >> 4) as usize],
        CHARS[(v[2] & 0xf) as usize],
        CHARS[(v[3] >> 4) as usize],
        CHARS[(v[3] & 0xf) as usize],
    ]
}

fn ptr_to_bytes(p: *const u8) -> [u8; 4] {
    (p as usize).to_be_bytes()
}

pub fn test_memory<A>(api: A)
where
    A: ErrorApi,
{
    let x = alloc::vec![1, 2, 3, 4, 5, 6];
    let n: usize = 1;
    let page_size: usize = 65536;

    let page = wasm32::memory_grow(0, n);
    let ptr = (page * page_size) as *mut u8;
    let end_ptr = ((page + n) * page_size) as *mut u8;

    let y = alloc::vec![10, 11, 12, 13, 14, 15];
    let z = alloc::vec![0; 80000];

    let page2 = wasm32::memory_grow(0, n);
    let ptr2 = (page2 * page_size) as *mut u8;
    let end_ptr2 = ((page2 + n) * page_size) as *mut u8;

    let t = [0; 5000];
    unsafe {
        api.signal_error(
            &[
                b"x:".iter().as_slice(),
                &ptr_to_string(x.as_slice().as_ptr()).iter().as_slice(),
                b" ps:".iter().as_slice(),
                &ptr_to_string(ptr).iter().as_slice(),
                b" pe:".iter().as_slice(),
                &ptr_to_string(end_ptr).iter().as_slice(),
                b" y:".iter().as_slice(),
                &ptr_to_string(y.as_slice().as_ptr()).iter().as_slice(),
                b" z:".iter().as_slice(),
                &ptr_to_string(z.as_slice().as_ptr()).iter().as_slice(),
                b" p2s:".iter().as_slice(),
                &ptr_to_string(ptr2).iter().as_slice(),
                b" p2e:".iter().as_slice(),
                &ptr_to_string(end_ptr2).iter().as_slice(),
                b" t:".iter().as_slice(),
                &ptr_to_string(t.as_ptr()).iter().as_slice(),
                b" B:".iter().as_slice(),
                &ptr_to_string(BUFFER.as_ptr()).iter().as_slice(),
            ]
            .concat(),
        );
    }
}
