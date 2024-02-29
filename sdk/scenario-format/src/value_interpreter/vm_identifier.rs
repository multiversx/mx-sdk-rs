pub const VM_TYPE_LENGTH: usize = 2;

#[derive(Default, Clone, Copy)]
pub struct VMIdentifier {
    pub vm_type: [u8; VM_TYPE_LENGTH],
}
