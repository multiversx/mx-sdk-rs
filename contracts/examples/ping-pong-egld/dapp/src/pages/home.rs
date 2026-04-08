use crate::{
    components::{Button, NetworkStatusComponent},
    requests::transaction,
};
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let show_status = use_state(|| false);
    let ping_result = use_state(String::new);
    let new_sc_address = use_state(String::new);

    let show_network_status = {
        let show_status = show_status.clone();

        Callback::from(move |_| {
            show_status.set(true);
        })
    };

    let deploy_sc = {
        let new_sc_address = new_sc_address.clone();

        Callback::from(move |_| {
            let new_sc_address = new_sc_address.clone();

            log::info!("SC setup request triggered");

            wasm_bindgen_futures::spawn_local(async move {
                match transaction::deploy_sc().await {
                    Ok(result) => {
                        new_sc_address.set(format!(
                            "New deployed address: {}",
                            result.to_bech32_string()
                        ));
                    }
                    Err(err) => {
                        log::error!("SC Setup failed: {:?}", err);
                        new_sc_address.set("SC Setup failed!".to_string());
                    }
                }
            });
        })
    };

    let ping = {
        let ping_result = ping_result.clone();

        Callback::from(move |_| {
            let ping_result = ping_result.clone();

            log::info!("Ping request triggered");

            wasm_bindgen_futures::spawn_local(async move {
                match transaction::ping().await {
                    Ok(result) => {
                        ping_result.set(result);
                    }
                    Err(err) => {
                        log::error!("Ping failed: {:?}", err);
                        ping_result.set("Ping failed!".to_string());
                    }
                }
            });
        })
    };

    html! {
        <>
        <div class = "btns">
        <Button name="Network Status" class_name="query-btn" button_type = "button" on_click={show_network_status.clone()} />
        <Button name="Deploy SC" class_name="deploy-btn" button_type = "button" on_click={deploy_sc.clone()} />
        <Button name="Ping" class_name="transaction-btn" button_type = "button" on_click={ping.clone()} />
        </div>
        {
            if *show_status {
                html! {
                    <NetworkStatusComponent />
                }
            }
            else {
                html! {}
            }
        }
        {
            if !new_sc_address.is_empty() {
                html! {
                    <p>
                    {
                        (*new_sc_address).clone()
                    }
                    </p>
                }
            }
            else {
                html! {}
            }
        }
        {
            if !ping_result.is_empty() {
                html! {
                    <p>
                    {
                        (*ping_result).clone()
                    }
                    </p>
                }
            }
            else {
                html! {}
            }
        }
        </>
    }
}
