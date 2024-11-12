use crate::requests::query;

use html::ChildrenProps;
use multiversx_sc_snippets::sdk::data::network_status::NetworkStatus;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigContext {
    pub network_status: NetworkStatus,
    pub set_network_status: Callback<NetworkStatus>,
}

pub async fn refresh_context() -> NetworkStatus {
    log::info!("refreshing context");
    query::get_network_status().await.unwrap_or_default()
}

impl Default for ConfigContext {
    fn default() -> Self {
        ConfigContext {
            network_status: NetworkStatus::default(),
            set_network_status: Callback::noop(),
        }
    }
}

#[function_component(ConfigProvider)]
pub fn config_provider(props: &ChildrenProps) -> Html {
    let network_status = use_state(NetworkStatus::default);

    // Clone the state here for use in the callback
    let set_network_status = {
        let network_status = network_status.clone();
        Callback::from(move |new_status: NetworkStatus| {
            network_status.set(new_status);
        })
    };

    // Clone the callback for async usage in the effect
    let set_network_status_async = set_network_status.clone();

    // refresh context on component mount
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let new_status = refresh_context().await;

                // Emit the new status inside the async block
                set_network_status_async.emit(new_status);
            });
            || () // no cleanup fn
        },
        (), // empty dependency array, run once on mount
    );

    let context = ConfigContext {
        network_status: (*network_status).clone(),
        set_network_status,
    };

    html! {
        <ContextProvider<ConfigContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<ConfigContext>>
    }
}
