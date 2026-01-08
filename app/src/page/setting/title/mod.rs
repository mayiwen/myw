use crate::{
    myw::{
        self,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    view! {
        <myw::Gap/>
        <h1>标题</h1>
        <myw::Gap/>

    }
}
