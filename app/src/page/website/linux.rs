use crate::myw;
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn I() -> impl IntoView {
    view! {
        <div style=" max-width: 800px; margin: auto;">
            <myw::Gap/>
            <h3>"linux 主界面"</h3>
            <myw::Gap/>
            <img style="width: 100%; max-width: 800px;" src="img/nuoruo_browser/linux-0.4.18-home.png" alt="linux home" />
            <myw::Gap/>
            <h3>"linux webview"</h3>
            <myw::Gap/>
            <img style="width: 100%; max-width: 800px;" src="img/nuoruo_browser/linux-0.4.18-webview.png" alt="linux webview" />
            <h3>"dev linux"</h3>
            <myw::Gap/>
            <img style="width: 100%; max-width: 800px;" src="img/nuoruo_browser/dev_linux.png" alt="linux webview" />
        </div>
    }
}
