use yew::prelude::*;

use crate::context::ConfigContext;

#[function_component(NetworkStatusComponent)]
pub fn network_status() -> Html {
    let context = use_context::<ConfigContext>().unwrap();
    let response = format!("{:?}", &context.network_status);

    html! {
        <div class = "network-status-container">
            <p>
               {&response}
            </p>
        </div>
    }
}
