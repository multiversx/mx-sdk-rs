/// The version of the SC environment interface (EI), it deals with the VM hooks available at a certain point in time.
///
/// It is not tied to the version of the VM, hence the different numbering.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum EIVersion {
    /// This is not necessarily the first version of the EI,
    /// but rather the oldest version when we started keeping track of the EI.
    V1_0,

    /// New hooks added in Q4 2021.
    ///
    /// Added a few more managed type & ESDT utilities.
    V1_1,

    /// New hooks added in Q2 2022. This is the EI version of VM 1.4.
    ///
    /// This is the version currently on mainnet.
    ///
    /// Added:
    /// - more managed type conversions
    /// - more managed crypto hooks
    /// - big floats
    /// - some managed ESDT properties.
    V1_2,

    /// Latest VM Hooks version, released with VM 1.5 in January 2024: https://multiversx.com/release/release-sirius-v1-6-7
    ///
    /// It adds the new async call functionality (promises).
    #[default]
    V1_3,

    /// Version to be released to mainnet in August 2024.
    V1_4,

    /// Version planned for Q3 2024.
    V1_5,
}

impl EIVersion {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "1.0" => Some(EIVersion::V1_0),
            "1.1" => Some(EIVersion::V1_1),
            "1.2" => Some(EIVersion::V1_2),
            "1.3" => Some(EIVersion::V1_3),
            "1.4" => Some(EIVersion::V1_4),
            "1.5" => Some(EIVersion::V1_5),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            EIVersion::V1_0 => "1.0",
            EIVersion::V1_1 => "1.1",
            EIVersion::V1_2 => "1.2",
            EIVersion::V1_3 => "1.3",
            EIVersion::V1_4 => "1.4",
            EIVersion::V1_5 => "1.5",
        }
    }

    pub fn vm_hook_names(&self) -> &'static [&'static str] {
        match self {
            EIVersion::V1_0 => super::EI_1_0_NAMES,
            EIVersion::V1_1 => super::EI_1_1_NAMES,
            EIVersion::V1_2 => super::EI_1_2_NAMES,
            EIVersion::V1_3 => super::EI_1_3_NAMES,
            EIVersion::V1_4 => super::EI_1_4_NAMES,
            EIVersion::V1_5 => super::EI_1_5_NAMES,
        }
    }

    pub fn contains_vm_hook(&self, vm_hook_names: &str) -> bool {
        self.vm_hook_names().contains(&vm_hook_names)
    }
}

/// Parses an EIVersion, or returns None, if "ignore" was specifically stated.
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
