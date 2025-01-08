use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct StaticVarApiFlags: u8 {
        const NONE                                  = 0b00000000;
        const CALL_VALUE_EGLD_DIRECT_INITIALIZED    = 0b00000001;
        const CALL_VALUE_ESDT_UNCHECKED_INITIALIZED = 0b00000010;
        const CALL_VALUE_ESDT_INITIALIZED           = 0b00000100;
        const CALL_VALUE_ALL_INITIALIZED            = 0b00001000;
    }
}

impl StaticVarApiFlags {
    pub fn check_and_set(&mut self, other: StaticVarApiFlags) -> bool {
        let contains_flag = self.contains(other);
        if !contains_flag {
            *self |= other;
        }
        contains_flag
    }
}

#[cfg(test)]
pub mod tests {
    use super::StaticVarApiFlags;

    #[test]
    fn test_check_and_set() {
        let mut current = StaticVarApiFlags::NONE;

        assert!(current.check_and_set(StaticVarApiFlags::NONE));
        assert_eq!(current, StaticVarApiFlags::NONE);

        assert!(!current.check_and_set(StaticVarApiFlags::CALL_VALUE_EGLD_DIRECT_INITIALIZED));
        assert_eq!(
            current,
            StaticVarApiFlags::CALL_VALUE_EGLD_DIRECT_INITIALIZED
        );
        assert!(current.check_and_set(StaticVarApiFlags::CALL_VALUE_EGLD_DIRECT_INITIALIZED));
        assert_eq!(
            current,
            StaticVarApiFlags::CALL_VALUE_EGLD_DIRECT_INITIALIZED
        );

        assert!(!current.check_and_set(StaticVarApiFlags::CALL_VALUE_ALL_INITIALIZED));
        assert_eq!(
            current,
            StaticVarApiFlags::CALL_VALUE_EGLD_DIRECT_INITIALIZED
                | StaticVarApiFlags::CALL_VALUE_ALL_INITIALIZED
        );
        assert!(current.check_and_set(StaticVarApiFlags::CALL_VALUE_ALL_INITIALIZED));
        assert_eq!(
            current,
            StaticVarApiFlags::CALL_VALUE_EGLD_DIRECT_INITIALIZED
                | StaticVarApiFlags::CALL_VALUE_ALL_INITIALIZED
        );
    }
}
