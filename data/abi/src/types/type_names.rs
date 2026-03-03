pub type TypeName = alloc::string::String;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct TypeNames {
    pub abi: alloc::string::String,
    pub rust: alloc::string::String,
    pub specific: Option<alloc::string::String>,
}

impl TypeNames {
    pub const fn new() -> Self {
        TypeNames {
            abi: alloc::string::String::new(),
            rust: alloc::string::String::new(),
            specific: None,
        }
    }

    pub const fn from_abi(abi_name: alloc::string::String) -> Self {
        TypeNames {
            abi: abi_name,
            rust: alloc::string::String::new(),
            specific: None,
        }
    }

    pub fn specific_or_abi(&self) -> &str {
        self.specific.as_deref().unwrap_or(&self.abi)
    }
}
