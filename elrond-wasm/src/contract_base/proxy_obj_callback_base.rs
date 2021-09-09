// use super::{ErrorApi, ManagedTypeApi, SendApi, StorageReadApi, StorageWriteApi};
use crate::api::VMApi;

pub trait CallbackProxyObjBase {
    type Api: VMApi;

    fn new_cb_proxy_obj(api: Self::Api) -> Self;

    fn cb_call_api(self) -> Self::Api;
}
