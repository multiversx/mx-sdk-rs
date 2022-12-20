extern "C" {
    fn signalError(messageOffset: *const u8, messageLength: i32) -> !;
}

#[inline(always)]
pub(crate) fn signal_error(message: &[u8]) -> ! {
    unsafe { signalError(message.as_ptr(), message.len() as i32) }
}
