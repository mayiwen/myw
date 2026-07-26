use crate::myw;
use leptos::prelude::*;


#[component]
pub fn I() -> impl IntoView {
    view! {
        <div style=" max-width: 800px; margin: auto;">
            <myw::Gap/>
            <h3>"windows 主界面"</h3>
            <myw::Gap/>
            <img style="width: 100%; max-width: 800px;" src="img/nuoruo_browser/windows-0.4.18-home.png" alt="windows home" />
            <myw::Gap/>
            <h3>"windows webview"</h3>
            <myw::Gap/>
            <img style="width: 100%; max-width: 800px;" src="img/nuoruo_browser/windows-0.4.18-webview.png" alt="windows webview" />
        </div>
    }
}
