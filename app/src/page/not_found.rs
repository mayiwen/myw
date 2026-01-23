use super::home::logo::MayiwenLogo;
use crate::myw;
use leptos::prelude::*;

#[component]
pub fn I() -> impl IntoView {
    view! {
        <myw::Gap h=30/>
        <div style="text-align: center">
            <MayiwenLogo/>
            <myw::Gap h=30/>
            <h2>您访问了不存在的网页</h2>
        </div>
    }
}
