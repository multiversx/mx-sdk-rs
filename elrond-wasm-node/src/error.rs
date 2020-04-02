extern {
    fn signalError(messageOffset: *const u8, messageLength: i32) -> !;
}

#[inline]
pub fn signal_error_raw(message_ptr: *const u8, message_len: usize) -> ! {
    unsafe { signalError(message_ptr, message_len as i32) }
}

#[inline]
pub fn signal_error(message: &str) -> ! {
    signal_error_raw(message.as_ptr(), message.len());
}