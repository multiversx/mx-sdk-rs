use crate::api::VMApi;

pub trait CallbackProxyObjBase {
    type Api<'a>: VMApi<'a>;

    fn new_cb_proxy_obj() -> Self;
}
