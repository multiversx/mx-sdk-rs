use crate::api::EndpointFinishApi;

/// Any type that implements this trait can be used to signal errors
/// when returning from a SC endpoint.
pub trait SCError {
    fn finish_err<FA: EndpointFinishApi>(&self, api: FA) -> !;
}
