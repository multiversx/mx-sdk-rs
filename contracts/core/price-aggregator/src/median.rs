multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/// Returns the sorted middle, or the average of the two middle indexed items if the
/// vector has an even number of elements.
pub fn calculate<M: ManagedTypeApi>(
    list: &mut [BigUint<M>],
) -> Result<Option<BigUint<M>>, StaticSCError> {
    if list.is_empty() {
        return Result::Ok(None);
    }
    list.sort_unstable();
    let len = list.len();
    let middle_index = len / 2;
    if len % 2 == 0 {
        let median1 = list.get(middle_index - 1).ok_or("median1 invalid index")?;
        let median2 = list.get(middle_index).ok_or("median2 invalid index")?;
        Result::Ok(Some((median1.clone() + median2.clone()) / 2u64))
    } else {
        let median = list.get(middle_index).ok_or("median invalid index")?;
        Result::Ok(Some(median.clone()))
    }
}
