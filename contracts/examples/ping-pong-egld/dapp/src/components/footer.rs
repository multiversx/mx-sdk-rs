use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class = "footer">
            <p>
                { "Made with " }
                <Icon class="icon" icon_id={IconId::BootstrapHeartFill}/>
                { " by the MultiversX team" }
            </p>
        </footer>
    }
}
