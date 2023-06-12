use multiversx_sc::{
    api::{handle_to_be_bytes, InvalidSliceError, RawHandle},
    types::BoxedBytes,
};

use crate::tx_mock::TxTokenTransfer;

use super::TxManagedTypes;

impl TxManagedTypes {
    pub fn mb_get(&self, handle: RawHandle) -> &[u8] {
        self.managed_buffer_map.get(handle).as_slice()
    }

    pub fn mb_len(&self, handle: RawHandle) -> usize {
        self.managed_buffer_map.get(handle).len()
    }

    pub fn mb_to_boxed_bytes(&self, handle: RawHandle) -> BoxedBytes {
        let data = self.mb_get(handle);
        data.into()
    }

    pub fn mb_get_slice(
        &self,
        source_handle: RawHandle,
        starting_position: usize,
        slice_len: usize,
    ) -> Result<Vec<u8>, InvalidSliceError> {
        let all_bytes = self.mb_get(source_handle);
        if starting_position + slice_len <= all_bytes.len() {
            Ok(all_bytes[starting_position..starting_position + slice_len].to_vec())
        } else {
            Err(InvalidSliceError)
        }
    }

    pub fn mb_load_slice(
        &self,
        source_handle: RawHandle,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        let slice = self.mb_get_slice(source_handle, starting_position, dest_slice.len())?;
        dest_slice.copy_from_slice(slice.as_slice());
        Ok(())
    }

    pub fn mb_set(&mut self, handle: RawHandle, value: Vec<u8>) {
        self.managed_buffer_map.insert(handle, value);
    }

    pub fn mb_new(&mut self, value: Vec<u8>) -> RawHandle {
        self.managed_buffer_map.insert_new_handle(value)
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

    /// Creates the underlying structure of a ManagedVec<ManagedBuffer> from raw data.
    pub fn mb_set_new_vec(&mut self, destination_handle: RawHandle, data: Vec<Vec<u8>>) {
        let mut m_vec_raw_data = Vec::new();
        for item in data.into_iter() {
            let handle = self.managed_buffer_map.insert_new_handle_raw(item);
            m_vec_raw_data.extend_from_slice(handle.to_be_bytes().as_slice());
        }
        self.mb_set(destination_handle, m_vec_raw_data);
    }
    
    pub fn write_all_esdt_transfers_to_managed_vec(
        &mut self,
        dest_handle: RawHandle,
        transfers: &[TxTokenTransfer],
    ) {
        self.mb_set(dest_handle, vec![]);

        for transfer in transfers {
            let token_identifier_handle = self.mb_new(transfer.token_identifier.clone());
            let amount_handle = self.bi_new_from_big_int(transfer.value.clone().into());

            self.mb_append_bytes(
                dest_handle,
                &handle_to_be_bytes(token_identifier_handle)[..],
            );
            self.mb_append_bytes(dest_handle, &transfer.nonce.to_be_bytes()[..]);
            self.mb_append_bytes(dest_handle, &handle_to_be_bytes(amount_handle)[..]);
        }
    }
}
