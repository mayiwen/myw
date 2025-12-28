use crate::myw::Gap;
use crate::page::home::logo::MayiwenLogo;
use crate::util::open_url;
use leptos::prelude::*;
mod logo;
mod nav;
/// Renders the home page of your application.
#[component]
pub fn I() -> impl IntoView {
    view! {
        <Gap/>
        <MayiwenLogo/>
        <Gap/>
        <nav::I/>



    }
}
