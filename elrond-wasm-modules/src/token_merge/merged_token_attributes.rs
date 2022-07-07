elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use core::cmp::Ordering;

pub const MAX_MERGED_TOKENS: usize = 10;
pub const MAX_TOKEN_ID_LEN: usize = 17; // 10 for ticker + '-' + 6 random hex chars

pub static TOO_MANY_TOKENS_ERR_MESG: &[u8] = b"Too many tokens to merge";
pub static INSUFFICIENT_BALANCE_IN_MERGED_INST_ERR_MSG: &[u8] =
    b"Insufficient token balance to deduct from merged instance";

#[derive(
    TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TokenAttributesInstance<M: ManagedTypeApi> {
    pub original_token_id_raw: ArrayVec<u8, MAX_TOKEN_ID_LEN>,
    pub original_token_nonce: u64,
    pub original_token_amount: BigUint<M>,
    pub attributes_raw: ManagedBuffer<M>,
    pub royalties: BigUint<M>,
    pub uris: ManagedVec<M, ManagedBuffer<M>>,
}

impl<M: ManagedTypeApi> TokenAttributesInstance<M> {
    pub fn from_single_token(token: EsdtTokenPayment<M>, token_data: EsdtTokenData<M>) -> Self {
        Self {
            original_token_id_raw: token_id_to_array_vec(&token.token_identifier),
            original_token_nonce: token.token_nonce,
            original_token_amount: token.amount,
            attributes_raw: token_data.attributes,
            royalties: token_data.royalties,
            uris: token_data.uris,
        }
    }
}

pub struct MergedTokenAttributes<M: ManagedTypeApi> {
    instances: ArrayVec<TokenAttributesInstance<M>, MAX_MERGED_TOKENS>,
}

impl<M: ManagedTypeApi> Default for MergedTokenAttributes<M> {
    fn default() -> Self {
        Self {
            instances: ArrayVec::new(),
        }
    }
}

impl<M: ManagedTypeApi> MergedTokenAttributes<M> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_from_sorted_instances(
        instances: ArrayVec<TokenAttributesInstance<M>, MAX_MERGED_TOKENS>,
    ) -> Self {
        Self { instances }
    }

    pub fn add_or_update_instance(&mut self, new_instance: TokenAttributesInstance<M>) {
        let search_result = self.binary_search_instance(
            &new_instance.original_token_id_raw,
            new_instance.original_token_nonce,
        );
        match search_result {
            core::result::Result::Ok(existing_index) => {
                self.instances[existing_index].original_token_amount +=
                    new_instance.original_token_amount;
            },
            core::result::Result::Err(index_to_insert) => {
                if self.instances.len() >= MAX_MERGED_TOKENS {
                    M::error_api_impl().signal_error(TOO_MANY_TOKENS_ERR_MESG);
                }

                self.instances.insert(index_to_insert, new_instance);
            },
        }
    }

    pub fn merge_with_other(
        &mut self,
        other: ArrayVec<TokenAttributesInstance<M>, MAX_MERGED_TOKENS>,
    ) {
        for inst in other {
            self.add_or_update_instance(inst);
        }
    }

    pub fn deduct_balance_for_instance(&mut self, tokens_to_deduct: &EsdtTokenPayment<M>) {
        let token_id_raw = token_id_to_array_vec(&tokens_to_deduct.token_identifier);
        let search_result =
            self.binary_search_instance(&token_id_raw, tokens_to_deduct.token_nonce);
        match search_result {
            core::result::Result::Ok(index) => {
                let found_instance = &mut self.instances[index];
                if found_instance.original_token_amount < tokens_to_deduct.amount {
                    M::error_api_impl().signal_error(INSUFFICIENT_BALANCE_IN_MERGED_INST_ERR_MSG);
                }

                found_instance.original_token_amount -= &tokens_to_deduct.amount;
                if found_instance.original_token_amount == 0 {
                    let _ = self.instances.remove(index);
                }
            },
            core::result::Result::Err(_) => {
                M::error_api_impl().signal_error(INSUFFICIENT_BALANCE_IN_MERGED_INST_ERR_MSG)
            },
        }
    }

    pub fn construct_full_uri_list(&self) -> ManagedVec<M, ManagedBuffer<M>> {
        let mut uris = ManagedVec::new();
        for inst in &self.instances {
            uris.append_vec(inst.uris.clone());
        }

        uris
    }

    pub fn get_max_royalties(&self) -> BigUint<M> {
        let zero = BigUint::zero();
        let mut max_ref = &zero;
        for inst in &self.instances {
            if &inst.royalties > max_ref {
                max_ref = &inst.royalties;
            }
        }

        max_ref.clone()
    }

    #[inline]
    pub fn into_instances(self) -> ArrayVec<TokenAttributesInstance<M>, MAX_MERGED_TOKENS> {
        self.instances
    }

    fn binary_search_instance(
        &self,
        original_token_id_raw: &ArrayVec<u8, MAX_TOKEN_ID_LEN>,
        original_token_nonce: u64,
    ) -> Result<usize, usize> {
        self.instances.binary_search_by(|item| {
            let token_id_cmp_result = item.original_token_id_raw.cmp(original_token_id_raw);
            if token_id_cmp_result != Ordering::Equal {
                return token_id_cmp_result;
            }

            item.original_token_nonce.cmp(&original_token_nonce)
        })
    }
}

fn token_id_to_array_vec<M: ManagedTypeApi>(
    token_id: &TokenIdentifier<M>,
) -> ArrayVec<u8, MAX_TOKEN_ID_LEN> {
    let mut array_vec = ArrayVec::new();
    let token_id_buffer = token_id.as_managed_buffer();
    let token_id_len = token_id_buffer.len();

    unsafe {
        array_vec.set_len(token_id_len);
    }

    let copy_result = token_id_buffer.load_slice(0, array_vec.as_mut_slice());
    if matches!(copy_result, core::result::Result::Err(_)) {
        M::error_api_impl().signal_error(b"Failed to copy managed buffer to slice");
    }

    array_vec
}
