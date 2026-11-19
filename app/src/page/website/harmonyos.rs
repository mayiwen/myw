use crate::myw;
use leptos::prelude::*;

#[component]
pub fn I() -> impl IntoView {
    view! {
        <div style=" max-width: 400px; margin: auto;">
            <myw::Gap/>
            <h3>"harmonyos 主界面"</h3>
            <myw::Gap/>
            <img style="width: 100%; max-width: 400px;" src="img/nuoruo_browser/harmonyos/harmonyos-0.4.50-home.png" alt="harmonyos home" />
            <myw::Gap/>
            <h3>"harmonyos webview"</h3>
            <myw::Gap/>
            <img style="width: 100%; max-width: 400px;" src="img/nuoruo_browser/harmonyos/harmonyos-0.4.50-webview.png" alt="harmonyos webview" />
        </div>
    }
}
