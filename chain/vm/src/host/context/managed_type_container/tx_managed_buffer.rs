use crate::{
    host::context::{TxFunctionName, TxTokenTransfer},
    types::{RawHandle, VMAddress, VMCodeMetadata},
};

use super::ManagedTypeContainer;

/// Returned if load/copy slice could not be performed.
/// No further data needed.
pub struct InvalidSliceError;

impl ManagedTypeContainer {
    pub fn mb_get_owned(&self, handle: RawHandle) -> Vec<u8> {
        self.managed_buffer_map.get(handle).to_vec()
    }

    pub fn mb_get(&self, handle: RawHandle) -> &[u8] {
        self.managed_buffer_map.get(handle).as_slice()
    }

    pub fn mb_len(&self, handle: RawHandle) -> usize {
        self.managed_buffer_map.get(handle).len()
    }

    pub fn mb_to_bytes(&self, handle: RawHandle) -> Vec<u8> {
        self.mb_get(handle).to_vec()
    }

    pub fn mb_to_address(&self, handle: RawHandle) -> VMAddress {
        VMAddress::from_slice(self.mb_get(handle))
    }

    pub fn mb_to_function_name(&self, handle: RawHandle) -> TxFunctionName {
        TxFunctionName::from(self.mb_get(handle))
    }

    pub fn mb_to_code_metadata(&self, handle: RawHandle) -> VMCodeMetadata {
        let bytes: [u8; 2] = self.mb_get(handle).try_into().unwrap();
        VMCodeMetadata::from(bytes)
    }

    pub fn mb_get_slice(
        &self,
        source_handle: RawHandle,
        starting_position: usize,
        slice_len: usize,
    ) -> Result<Vec<u8>, InvalidSliceError> {
        let all_bytes = self.mb_get(source_handle);
        if starting_position + slice_len <= all_bytes.len() {
            let slice = &all_bytes[starting_position..starting_position + slice_len];
            Ok(slice.to_vec())
        } else {
            Err(InvalidSliceError)
        }
    }

    pub fn mb_set(&mut self, handle: RawHandle, value: Vec<u8>) {
        self.managed_buffer_map.insert(handle, value);
    }

    pub fn mb_new(&mut self, value: Vec<u8>) -> RawHandle {
        self.managed_buffer_map.insert_new_handle_raw(value)
    }

    pub fn mb_update<R, F: FnOnce(&mut Vec<u8>) -> R>(&mut self, handle: RawHandle, f: F) -> R {
        let value = self.managed_buffer_map.get_mut(handle);
        f(value)
    }

    pub fn mb_set_slice(
        &mut self,
        dest_handle: RawHandle,
        starting_position: usize,
        source_slice: &[u8],
    ) -> Result<(), InvalidSliceError> {
        self.mb_update(dest_handle, |bytes| {
            let end_position = starting_position + source_slice.len();
            if end_position <= bytes.len() {
                bytes[starting_position..end_position].copy_from_slice(source_slice);
                Ok(())
            } else {
                Err(InvalidSliceError)
            }
        })
    }

    pub fn mb_append_bytes(&mut self, accumulator_handle: RawHandle, bytes: &[u8]) {
        self.mb_update(accumulator_handle, |accumulator| {
            accumulator.extend_from_slice(bytes);
        });
    }

    /// Retrieves data saved in the format of a ManagedVec<ManagedBuffer>,
    /// i.e. the main data structure encodes the handles of other buffers.
    pub fn mb_get_vec_of_bytes(&self, source_handle: RawHandle) -> (Vec<Vec<u8>>, usize) {
        let mut result = Vec::new();
        let mut num_bytes_copied = 0;
        let data = self.mb_get(source_handle);
        assert!(
            data.len().is_multiple_of(4),
            "malformed ManagedVec<ManagedBuffer> data"
        );
        for chunk in data.chunks(4) {
            let item_handle = i32::from_be_bytes(chunk.try_into().unwrap());
            let data = self.mb_get(item_handle).to_vec();
            num_bytes_copied += data.len();
            result.push(data);
        }
        (result, num_bytes_copied)
    }

    /// Creates the underlying structure of a ManagedVec<ManagedBuffer> from memory..
    pub fn mb_set_vec_of_bytes(
        &mut self,
        destination_handle: RawHandle,
        data: Vec<Vec<u8>>,
    ) -> usize {
        let mut m_vec_raw_data = Vec::new();
        let mut num_bytes_copied = 0;
        for item in data.into_iter() {
            num_bytes_copied += item.len();
            let handle = self.managed_buffer_map.insert_new_handle_raw(item);
            m_vec_raw_data.extend_from_slice(handle.to_be_bytes().as_slice());
        }
        self.mb_set(destination_handle, m_vec_raw_data);
        num_bytes_copied
    }

    pub fn mb_get_vec_of_esdt_payments(
        &self,
        source_handle: RawHandle,
    ) -> (Vec<TxTokenTransfer>, usize) {
        let mut result = Vec::new();
        let mut num_bytes_copied = 0;
        let data = self.mb_get(source_handle);
        assert!(
            data.len().is_multiple_of(16),
            "malformed ManagedVec<EsdtTokenPayment> data"
        );
        for chunk in data.chunks(16) {
            let token_id_handle = i32::from_be_bytes(chunk[0..4].try_into().unwrap());
            let token_id = self.mb_get(token_id_handle).to_vec();
            num_bytes_copied += token_id.len();

            let nonce = u64::from_be_bytes(chunk[4..12].try_into().unwrap());

            let amount_handle = i32::from_be_bytes(chunk[12..16].try_into().unwrap());
            let amount = self.bu_get(amount_handle);
            num_bytes_copied += (amount.bits() / 8) as usize;

            result.push(TxTokenTransfer {
                token_identifier: token_id,
                nonce,
                value: amount,
            });
        }
        (result, num_bytes_copied)
    }

    pub fn mb_set_vec_of_esdt_payments(
        &mut self,
        dest_handle: RawHandle,
        transfers: &[TxTokenTransfer],
    ) -> usize {
        let mut num_bytes_copied = 0;

        self.mb_set(dest_handle, vec![]);

        for transfer in transfers {
            num_bytes_copied += transfer.token_identifier.len();
            let token_identifier_handle = self.mb_new(transfer.token_identifier.clone());

            num_bytes_copied += (transfer.value.bits() / 8) as usize;
            let amount_handle = self.bi_new_from_big_int(transfer.value.clone().into());

            self.mb_append_bytes(
                dest_handle,
                &handle_to_be_bytes(token_identifier_handle)[..],
            );
            self.mb_append_bytes(dest_handle, &transfer.nonce.to_be_bytes()[..]);
            self.mb_append_bytes(dest_handle, &handle_to_be_bytes(amount_handle)[..]);
        }

        num_bytes_copied
    }
}

pub fn handle_to_be_bytes(handle: RawHandle) -> [u8; 4] {
    handle.to_be_bytes()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_vec_of_bytes() {
        let mut m_types = ManagedTypeContainer::new();
        let handle = m_types.mb_new(vec![]);
        let data = vec![b"abc".to_vec(), b"defghi".to_vec(), b"jk".to_vec()];
        let _ = m_types.mb_set_vec_of_bytes(handle, data.clone());
        let (retrieved, _) = m_types.mb_get_vec_of_bytes(handle);
        assert_eq!(data, retrieved);
    }

    #[test]
    fn test_vec_of_esdt_payments() {
        let mut m_types = ManagedTypeContainer::new();
        let handle = m_types.mb_new(vec![]);
        let transfers = vec![TxTokenTransfer {
            token_identifier: b"TOKEN-12345".to_vec(),
            nonce: 6,
            value: 789u32.into(),
        }];
        let _ = m_types.mb_set_vec_of_esdt_payments(handle, transfers.as_slice());
        let (retrieved, _) = m_types.mb_get_vec_of_esdt_payments(handle);
        assert_eq!(transfers, retrieved);
    }
}
