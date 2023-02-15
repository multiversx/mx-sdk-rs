use num_bigint::BigUint;

use crate::serde_raw::ValueSubTree;

type ExprReconstructorHint = u64;

// NoHint indicates that the type if not known
const NO_HINT: ExprReconstructorHint = 0;

// NumberHint hints that value should be a number
const UNSIGNED_NUMBER_HINT: ExprReconstructorHint = 1;

// AddressHint hints that value should be an address
const ADDRESS_HINT: ExprReconstructorHint = 2;

// StrHint hints that value should be a string expression, e.g. a username, "str:..."
const STR_HINT: ExprReconstructorHint = 3;

// CodeHint hints that value should be a smart contract code, normally loaded from a file
const CODE_HINT: ExprReconstructorHint = 4;

const MAX_BYTES_INTERPRETED_AS_NUMBER: usize = 15;
const SC_ADDRESS_NUM_LEADING_ZEROS: usize = 8;
const SC_ADDRESS_RESERVED_PREFIX_LENGTH: usize = SC_ADDRESS_NUM_LEADING_ZEROS + 2;
const SC_ADDRESS_LENGTH: usize = 32;
const SC_CODE_LENGTH: usize = 20;

fn reconstruct(value: &[u8], hint: ExprReconstructorHint) -> ValueSubTree {
    let str: String = match hint {
        UNSIGNED_NUMBER_HINT => BigUint::from_bytes_be(value).to_string(),
        STR_HINT => String::from_utf8_lossy(value).to_string(),
        ADDRESS_HINT => address_pretty(value),
        CODE_HINT => code_pretty(value),
        _ => unknown_byte_array_pretty(value),
    };
    ValueSubTree::Str(str)
}

fn reconstruct_from_biguint(value: BigUint) -> ValueSubTree {
    reconstruct(&value.to_bytes_be(), UNSIGNED_NUMBER_HINT)
}

fn reconstruct_from_u64(value: u64) -> ValueSubTree {
    reconstruct(&BigUint::from(value).to_bytes_be(), UNSIGNED_NUMBER_HINT)
}

fn reconstruction_list(values: &[&[u8]], hint: ExprReconstructorHint) -> ValueSubTree {
    let mut strings: Vec<ValueSubTree> = Vec::new();
    for value in values.iter() {
        strings.push(reconstruct(value, hint));
    }
    ValueSubTree::List(strings)
}

fn unknown_byte_array_pretty(bytes: &[u8]) -> String {
    if bytes.len() == 0 {
        return String::new();
    }

    // fully interpret as string
    if can_interpret_as_string(bytes) {
        return format!(
            "0x{} (str:{})",
            hex::encode(bytes),
            String::from_utf8_lossy(bytes).to_string()
        );
    }

    // interpret as number
    if bytes.len() < MAX_BYTES_INTERPRETED_AS_NUMBER {
        let as_uint = BigUint::from_bytes_be(bytes).to_string();
        return format!("0x{} ({})", hex::encode(bytes), as_uint);
    }

    // default interpret as string with escaped bytes
    return format!(
        "0x{} (str:{:?})",
        hex::encode(bytes),
        String::from_utf8_lossy(bytes).to_string(),
    );
}

fn address_pretty(value: &[u8]) -> String {
    if value.len() != 32 {
        return unknown_byte_array_pretty(value);
    }

    // smart contract address
    if value[..SC_ADDRESS_NUM_LEADING_ZEROS]
        .partial_cmp(&vec![0; 8])
        .is_some()
    {
        if value[SC_ADDRESS_LENGTH - 1] == b'_' {
            let address_str =
                String::from_utf8_lossy(&value[SC_ADDRESS_RESERVED_PREFIX_LENGTH..]).to_string();
            return format!("sc:{}", address_str.trim_end_matches('_').to_owned());
        } else {
            // last byte is the shard id and is explicit

            let address_str = String::from_utf8_lossy(
                &value[SC_ADDRESS_RESERVED_PREFIX_LENGTH..SC_ADDRESS_LENGTH - 1],
            )
            .to_string();
            let shard_id = value[SC_ADDRESS_LENGTH - 1];
            return format!(
                "sc:{}{:#x}",
                address_str.trim_end_matches('_').to_owned(),
                shard_id
            );
        }
    }

    // regular addresses
    if value[SC_ADDRESS_LENGTH - 1] == b'_' {
        let address_str = String::from_utf8_lossy(&value).to_string();
        return format!("address:{}", address_str.trim_end_matches('_').to_owned());
    } else {
        let mut address_str = String::from_utf8_lossy(&value[..SC_ADDRESS_LENGTH - 1]).to_string();
        address_str = address_str.trim_end_matches('_').to_owned();
        let shard_id = value[SC_ADDRESS_LENGTH - 1];
        let address_expr = format!("address:{}{:#02x}", address_str, shard_id);
        if !can_interpret_as_string(&value[SC_ADDRESS_LENGTH - 1..SC_ADDRESS_LENGTH - 1]) {
            return format!("0x{} ({})", hex::encode(value), address_expr);
        }
        address_expr
    }
}

fn can_interpret_as_string(bytes: &[u8]) -> bool {
    if bytes.len() == 0 {
        return false;
    }
    return bytes.iter().find(|&&b| b < 32 || b > 126).is_none();
}

fn code_pretty(bytes: &[u8]) -> String {
    if bytes.len() == 0 {
        return String::new();
    }
    let encoded = hex::encode(bytes);

    if encoded.len() > SC_CODE_LENGTH {
        return format!("0x{}...", &encoded[..SC_CODE_LENGTH]);
    }

    format!("0x{}", encoded)
}
