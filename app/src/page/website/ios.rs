use crate::myw;
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn I() -> impl IntoView {
    view! {
        <div style=" max-width: 400px; margin: auto;">
            <myw::Gap/>
            <h3>"ios 主界面"</h3>
            <myw::Gap/>
            <img style="width: 100%; max-width: 400px;" src="img/nuoruo_browser/ios-0.4.18-home.png" alt="ios home" />
            <myw::Gap/>
            <h3>"ios webview"</h3>
            <myw::Gap/>
            <img style="width: 100%; max-width: 400px;" src="img/nuoruo_browser/ios-0.4.18-webview.png" alt="ios webview" />
        </div>
    }
}
