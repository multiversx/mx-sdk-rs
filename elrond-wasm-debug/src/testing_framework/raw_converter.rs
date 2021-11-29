use std::collections::BTreeMap;

use crate::world_mock::{AccountData, EsdtData};
use elrond_wasm::{elrond_codec::TopEncode, types::Address};
use mandos::serde_raw::{AccountRaw, EsdtFullRaw, EsdtRaw, InstanceRaw, ValueSubTree};

pub fn account_as_raw(acc: &AccountData) -> AccountRaw {
    let balance_raw = Some(rust_biguint_as_raw(&acc.egld_balance));
    let code_raw = acc.contract_path.clone().map(|c| bytes_as_raw(&c));

    let mut all_esdt_raw = BTreeMap::new();
    for (token_id, esdt_data) in acc.esdt.iter() {
        let token_id_raw = String::from_utf8(token_id.clone()).unwrap();
        let esdt_raw = esdt_data_as_raw(esdt_data);

        let _ = all_esdt_raw.insert(token_id_raw, esdt_raw);
    }

    let mut storage_raw = BTreeMap::new();
    for (key, value) in acc.storage.iter() {
        let key_raw = String::from_utf8(key.clone()).unwrap();
        let value_raw = bytes_as_raw(value);

        let _ = storage_raw.insert(key_raw, value_raw);
    }

    AccountRaw {
        balance: balance_raw,
        code: code_raw,
        comment: None,
        esdt: all_esdt_raw,
        nonce: Some(u64_as_raw(acc.nonce)),
        owner: acc.contract_owner.as_ref().map(|o| address_as_raw(o)),
        storage: storage_raw,
        username: None, // TODO: Add if needed
    }
}

pub fn esdt_data_as_raw(esdt: &EsdtData) -> EsdtRaw {
    let last_nonce_raw = if esdt.last_nonce == 0 {
        None
    } else {
        Some(u64_as_raw(esdt.last_nonce))
    };

    let roles = esdt.get_roles();
    let mut roles_raw = Vec::with_capacity(roles.len());
    for role in roles {
        roles_raw.push(String::from_utf8(role).unwrap());
    }

    let mut instances_raw = Vec::new();
    for (_, inst) in esdt.instances.get_instances() {
        let inst_raw = InstanceRaw {
            attributes: Some(bytes_as_raw(&inst.metadata.attributes)),
            balance: Some(rust_biguint_as_raw(&inst.balance)),
            creator: inst.metadata.creator.as_ref().map(|c| address_as_raw(c)),
            hash: inst.metadata.hash.as_ref().map(|h| bytes_as_raw(h)),
            nonce: Some(u64_as_raw(inst.nonce)),
            royalties: Some(u64_as_raw(inst.metadata.royalties)),
            uri: inst.metadata.uri.as_ref().map(|u| bytes_as_raw(u)),
        };

        instances_raw.push(inst_raw);
    }

    EsdtRaw::Full(EsdtFullRaw {
        frozen: None,
        instances: instances_raw,
        last_nonce: last_nonce_raw,
        roles: roles_raw,
        token_identifier: Some(bytes_as_raw(&esdt.token_identifier)),
    })
}

pub fn rust_biguint_as_raw(big_uint: &num_bigint::BigUint) -> ValueSubTree {
    bytes_as_raw(&big_uint.to_bytes_be())
}

pub fn address_as_raw(address: &Address) -> ValueSubTree {
    bytes_as_raw(address.as_bytes())
}

pub fn u64_as_raw(value: u64) -> ValueSubTree {
    let mut min_bytes = Vec::new();
    value.top_encode(&mut min_bytes).unwrap();

    bytes_as_raw(&min_bytes)
}

pub fn bytes_as_raw(bytes: &[u8]) -> ValueSubTree {
    ValueSubTree::Str(bytes_to_hex(bytes))
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    "0x".to_owned() + &hex::encode(bytes)
}
