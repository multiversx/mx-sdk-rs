use core::ops::Deref;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const MAX_MERGED_TOKENS: usize = 25;

pub static TOO_MANY_TOKENS_ERR_MSG: &[u8] = b"Too many tokens to merge";
pub static INSUFFICIENT_BALANCE_IN_MERGED_INST_ERR_MSG: &[u8] =
    b"Insufficient token balance to deduct from merged instance";

pub type InstanceArray<M> = ArrayVec<EsdtTokenPayment<M>, MAX_MERGED_TOKENS>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergedTokenInstances<M: ManagedTypeApi> {
    instances: InstanceArray<M>,
}

impl<M: ManagedTypeApi> Default for MergedTokenInstances<M> {
    #[inline]
    fn default() -> Self {
        Self {
            instances: ArrayVec::new(),
        }
    }
}

impl<M: ManagedTypeApi> MergedTokenInstances<M> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn new_from_instances(instances: InstanceArray<M>) -> Self {
        Self { instances }
    }

    pub fn decode_from_first_uri(uris: &ManagedVec<M, ManagedBuffer<M>>) -> Self {
        let first_uri = uris.get(0);
        let decode_result = InstanceArray::<M>::top_decode(first_uri.deref().clone());
        match decode_result {
            core::result::Result::Ok(instances) => Self::new_from_instances(instances),
            core::result::Result::Err(_) => {
                M::error_api_impl().signal_error(b"Error decoding tokens from URI")
            },
        }
    }

    #[inline]
    pub fn get_instances(&self) -> &InstanceArray<M> {
        &self.instances
    }

    pub fn add_or_update_instance(&mut self, new_instance: EsdtTokenPayment<M>) {
        let search_result =
            self.find_instance(&new_instance.token_identifier, new_instance.token_nonce);
        match search_result {
            Some(existing_index) => {
                self.instances[existing_index].amount += new_instance.amount;
            },
            None => {
                if self.instances.len() >= MAX_MERGED_TOKENS {
                    M::error_api_impl().signal_error(TOO_MANY_TOKENS_ERR_MSG);
                }

                unsafe {
                    self.instances.push_unchecked(new_instance);
                }
            },
        }
    }

    pub fn merge_with_other(&mut self, other: Self) {
        for inst in other.instances {
            self.add_or_update_instance(inst);
        }
    }

    pub fn deduct_balance_for_instance(&mut self, tokens_to_deduct: &EsdtTokenPayment<M>) {
        let search_result = self.find_instance(
            &tokens_to_deduct.token_identifier,
            tokens_to_deduct.token_nonce,
        );
        match search_result {
            Some(index) => {
                let found_instance = &mut self.instances[index];
                if tokens_to_deduct.amount == 0 || found_instance.amount < tokens_to_deduct.amount {
                    M::error_api_impl().signal_error(INSUFFICIENT_BALANCE_IN_MERGED_INST_ERR_MSG);
                }

                found_instance.amount -= &tokens_to_deduct.amount;
                if found_instance.amount == 0 {
                    let _ = self.instances.remove(index);
                }
            },
            None => M::error_api_impl().signal_error(INSUFFICIENT_BALANCE_IN_MERGED_INST_ERR_MSG),
        }
    }

    #[inline]
    pub fn into_instances(self) -> InstanceArray<M> {
        self.instances
    }

    fn find_instance(
        &self,
        original_token_id: &TokenIdentifier<M>,
        original_token_nonce: u64,
    ) -> Option<usize> {
        self.instances.iter().position(|item| {
            &item.token_identifier == original_token_id && item.token_nonce == original_token_nonce
        })
    }
}
