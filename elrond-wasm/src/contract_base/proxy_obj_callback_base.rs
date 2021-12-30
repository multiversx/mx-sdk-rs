use crate::api::VMApi;

pub trait CallbackProxyObjBase {
    type Api: VMApi;

    fn new_cb_proxy_obj() -> Self;
}
