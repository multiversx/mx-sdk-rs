use crate::ei::EIVersion;

use super::ContractAllocator;

pub fn parse_check_ei(ei: &Option<String>) -> Option<EIVersion> {
    if let Some(ei_name) = ei {
        if ei_name == "ignore" {
            None
        } else {
            let ei_version = EIVersion::from_name(ei_name)
                .unwrap_or_else(|| panic!("invalid EI version: {ei_name}"));
            Some(ei_version)
        }
    } else {
        Some(EIVersion::default())
    }
}

pub fn parse_allocator(allocator: &Option<String>) -> ContractAllocator {
    allocator
        .as_ref()
        .map(|s| ContractAllocator::parse_or_panic(s))
        .unwrap_or_default()
}
