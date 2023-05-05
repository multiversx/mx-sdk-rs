/// The version of the SC environment interface (EI), it deals with the VM hooks available at a certain point in time.
///
/// It is not tied to the version of the VM, hence the different numbering.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
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
    #[default]
    V1_2,

    /// VM Hooks version planned to be released with VM 1.5 in Q2 2023.
    ///
    /// It adds the new async call functionality (promises).
    V1_3,
}

impl EIVersion {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "1.0" => Some(EIVersion::V1_0),
            "1.1" => Some(EIVersion::V1_1),
            "1.2" => Some(EIVersion::V1_2),
            "1.3" => Some(EIVersion::V1_3),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            EIVersion::V1_0 => "1.0",
            EIVersion::V1_1 => "1.1",
            EIVersion::V1_2 => "1.2",
            EIVersion::V1_3 => "1.3",
        }
    }

    pub fn vm_hook_names(&self) -> &'static [&'static str] {
        match self {
            EIVersion::V1_0 => super::EI_1_0_NAMES,
            EIVersion::V1_1 => super::EI_1_1_NAMES,
            EIVersion::V1_2 => super::EI_1_2_NAMES,
            EIVersion::V1_3 => super::EI_1_3_NAMES,
        }
    }

    pub fn contains_vm_hook(&self, vm_hook_names: &str) -> bool {
        self.vm_hook_names().contains(&vm_hook_names)
    }
}
