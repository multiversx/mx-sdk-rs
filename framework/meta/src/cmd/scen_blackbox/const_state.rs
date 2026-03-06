use std::collections::HashMap;

/// Grouping for generated constants. Variants are ordered by desired output order.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstGroup {
    CodePath,
    Address,
    TokenId,
    Hash,
    ByteArray,
}

/// Data about a generated constant: its group, type, and initialization expression.
pub struct ConstData {
    pub const_group: ConstGroup,
    pub const_type: String,
    pub initialization: String,
}

/// Holds all constant-related state: lookup maps, counters, and the constant registry.
#[derive(Default)]
pub struct ConstState {
    /// Maps address value to constant name (for TestAddress/TestSCAddress)
    test_address_map: HashMap<String, String>,
    /// Maps hex address to constant name
    hex_address_map: HashMap<String, String>,
    /// Counter for hex address constants
    hex_address_counter: usize,
    /// Maps code path expression to constant name
    code_path_map: HashMap<String, String>,
    /// Maps token identifier to constant name
    token_id_map: HashMap<String, String>,
    /// Maps H256 hex value to constant name
    h256_map: HashMap<String, String>,
    /// Counter for H256 constants
    h256_counter: usize,
    /// Maps byte array hex value to constant name (for arrayN<u8> types)
    hex_array_map: HashMap<String, String>,
    /// Counter for byte array constants, per size
    hex_array_counter: HashMap<usize, usize>,
    /// Map from constant name to its data (type and initialization)
    const_map: HashMap<String, ConstData>,
}

impl ConstState {
    /// Registers a new constant declaration.
    fn add_const(
        &mut self,
        name: String,
        const_group: ConstGroup,
        const_type: String,
        initialization: String,
    ) {
        self.const_map.insert(
            name,
            ConstData {
                const_group,
                const_type,
                initialization,
            },
        );
    }

    /// Renders all registered constants, sorted by group, then by type name, then by const name.
    pub fn render_constants(&self) -> String {
        let mut entries: Vec<_> = self.const_map.iter().collect();
        entries.sort_by(|a, b| {
            a.1.const_group
                .cmp(&b.1.const_group)
                .then(a.1.const_type.cmp(&b.1.const_type))
                .then(a.0.cmp(b.0))
        });

        let mut buf = String::new();
        for (name, data) in entries {
            buf.push_str(&format!(
                "const {}: {} = {};\n",
                name, data.const_type, data.initialization
            ));
        }
        buf
    }

    /// Converts an address name to an upper-case constant name.
    /// Example: "my-address" -> "MY_ADDRESS_ADDRESS"
    fn address_to_const_name(name: &str) -> String {
        format!(
            "{}_ADDRESS",
            name.to_uppercase().replace(['-', '.', ' '], "_")
        )
    }

    /// Derives a constant name from a code path expression.
    /// Example: "mxsc:../output/adder.mxsc.json" -> "ADDER_CODE_PATH"
    fn derive_code_path_const_name(code_path_expr: &str) -> String {
        let path_str = code_path_expr
            .strip_prefix("mxsc:")
            .unwrap_or(code_path_expr);
        let filename = path_str.rsplit('/').next().unwrap_or(path_str);

        let contract_name = filename
            .strip_suffix(".mxsc.json")
            .unwrap_or(filename)
            .replace('-', "_");

        format!("{}_CODE_PATH", contract_name.to_uppercase())
    }

    /// Formats a code path expression, generating a constant if needed.
    pub fn format_code_path(&mut self, code_path_expr: &str) -> String {
        if let Some(const_name) = self.code_path_map.get(code_path_expr) {
            return const_name.clone();
        }

        let const_name = Self::derive_code_path_const_name(code_path_expr);

        let path_value = code_path_expr
            .strip_prefix("mxsc:")
            .unwrap_or(code_path_expr);
        let path_value = path_value.strip_prefix("../").unwrap_or(path_value);

        self.add_const(
            const_name.clone(),
            ConstGroup::CodePath,
            "MxscPath".to_string(),
            format!("MxscPath::new(\"{}\")", path_value),
        );

        self.code_path_map
            .insert(code_path_expr.to_string(), const_name.clone());

        const_name
    }

    /// Returns the constant name for a token ID, creating the constant if needed.
    pub fn get_or_create_token_id(&mut self, name: &str) -> String {
        if let Some(const_name) = self.token_id_map.get(name) {
            return const_name.clone();
        }

        let const_name = name.to_uppercase().replace('-', "_");

        self.add_const(
            const_name.clone(),
            ConstGroup::TokenId,
            "TestTokenId".to_string(),
            format!("TestTokenId::new(\"{}\")", name),
        );

        self.token_id_map
            .insert(name.to_string(), const_name.clone());

        const_name
    }

    /// Returns the constant name for a 32-byte H256 value, creating the constant if needed.
    pub fn get_or_create_h256(&mut self, hex_str: &str) -> String {
        if let Some(const_name) = self.h256_map.get(hex_str) {
            return const_name.clone();
        }

        self.h256_counter += 1;
        let const_name = format!("H256_{}", self.h256_counter);

        self.add_const(
            const_name.clone(),
            ConstGroup::Hash,
            "H256".to_string(),
            format!("H256::from_hex(\"{}\")", hex_str),
        );

        self.h256_map
            .insert(hex_str.to_string(), const_name.clone());

        const_name
    }

    /// Returns a reference expression (`&HEX_{size}_{N}`) for a fixed-size byte array,
    /// creating the constant if needed.
    pub fn get_or_create_byte_array(&mut self, hex_str: &str, size: usize) -> String {
        if let Some(const_name) = self.hex_array_map.get(hex_str) {
            return format!("&{}", const_name);
        }

        let counter = self.hex_array_counter.entry(size).or_insert(0);
        *counter += 1;
        let const_name = format!("HEX_{size}_{counter:02}");

        self.add_const(
            const_name.clone(),
            ConstGroup::ByteArray,
            format!("[u8; {}]", size),
            format!("hex!(\"{}\")", hex_str),
        );

        self.hex_array_map
            .insert(hex_str.to_string(), const_name.clone());

        format!("&{}", const_name)
    }

    /// Returns the constant name for a user address (`address:` prefix),
    /// creating the constant if needed.
    pub fn get_or_create_address(&mut self, addr: &str, name: &str) -> String {
        self.get_or_create_test_address_impl(
            addr,
            name,
            "TestAddress",
            &format!("TestAddress::new(\"{}\")", name),
        )
    }

    /// Returns the constant name for a smart-contract address (`sc:` prefix),
    /// creating the constant if needed.
    pub fn get_or_create_sc_address(&mut self, addr: &str, name: &str) -> String {
        self.get_or_create_test_address_impl(
            addr,
            name,
            "TestSCAddress",
            &format!("TestSCAddress::new(\"{}\")", name),
        )
    }

    /// Shared implementation for named-address constant creation.
    fn get_or_create_test_address_impl(
        &mut self,
        addr: &str,
        name: &str,
        const_type: &str,
        initialization: &str,
    ) -> String {
        if let Some(const_name) = self.test_address_map.get(addr) {
            return const_name.clone();
        }

        let const_name = Self::address_to_const_name(name);

        self.add_const(
            const_name.clone(),
            ConstGroup::Address,
            const_type.to_string(),
            initialization.to_string(),
        );

        self.test_address_map
            .insert(addr.to_string(), const_name.clone());

        const_name
    }

    /// Returns the constant name for a raw hex address, creating the constant if needed.
    pub fn get_or_create_hex_address(&mut self, hex: &str) -> String {
        if let Some(const_name) = self.hex_address_map.get(hex) {
            return const_name.clone();
        }

        self.hex_address_counter += 1;
        let const_name = format!("ADDRESS_HEX_{}", self.hex_address_counter);

        self.add_const(
            const_name.clone(),
            ConstGroup::Address,
            "Address".to_string(),
            format!("Address::from_hex(\"{}\")", hex),
        );

        self.hex_address_map
            .insert(hex.to_string(), const_name.clone());

        const_name
    }
}
