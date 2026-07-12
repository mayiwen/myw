use crate::myw;
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn I() -> impl IntoView {
    view! {
        <div style=" max-width: 800px; margin: auto;">
            <myw::Gap/>
            <h3>"macos 主界面"</h3>
            <myw::Gap/>
            <img style="width: 100%; max-width: 800px;" src="img/nuoruo_browser/macos-0.4.18-home.png" alt="macos home" />
            <myw::Gap/>
            <h3>"macos webview"</h3>
            <myw::Gap/>
            <img style="width: 100%; max-width: 800px;" src="img/nuoruo_browser/macos-0.4.18-webview.png" alt="macos webview" />
        </div>
    }
}
