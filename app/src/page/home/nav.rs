use std::sync::Arc;

use crate::myw::button::Button;
use crate::myw::tabset::{Tab, Tabs, Tabset};
use crate::util::open_url;
use crate::{Link, Title};
use leptos::prelude::*;
use leptos::reactive::spawn_local;

#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    let arr_vec: RwSignal<Vec<Title>> = RwSignal::new(vec![]);
    let link_vec: RwSignal<Vec<Link>> = RwSignal::new(vec![]);
    let get_link = move |id: u64| {
        spawn_local(async move {
            let res = crate::get_link(id).await;
            match res {
                Ok(res) => {
                    let id_temp: u64 = match res.get(0) {
                        Some(t) => t.id,
                        None => 0,
                    };
                    link_vec.set(res);
                    // id.set(id_temp);
                }
                Err(_) => {}
            }
        });
    };
    // 使用派生信号（根据依赖自动更新）
    let tabs = Signal::derive(move || {
        arr_vec
            .get()
            .into_iter()
            .map(|title| {
                let children_fn: ChildrenFn = Arc::new(move || view! { "" }.into_any());
                Tab {
                    children: children_fn,
                    title: title.title.clone(),
                    id: title.id,
                    icon: None,
                    click: Some(Callback::from(move || get_link(title.id))),
                }
            })
            .collect::<Vec<Tab>>()
    });

    Effect::new(move |_| {
        spawn_local(async move {
            let res = crate::get_title().await;
            match res {
                Ok(res) => {
                    let id_temp: u64 = match res.get(0) {
                        Some(t) => t.id,
                        None => 0,
                    };
                    arr_vec.set(res);
                    id.set(id_temp);
                    get_link(id_temp)
                }
                Err(_) => {}
            }
        });
    });

    view! {
        <div style="max-width: 1200px; width: 100%; padding: 0px 4px; margin: auto">
            <Tabs tab=tabs id=id />
        </div>
        <div style="display: grid; gap: 4px 4px;grid-template-columns: repeat(auto-fill, minmax(125px, 1fr)); padding: 4px 0px;  max-width: 1200px;  margin: auto">
            <For
                each=move || link_vec.get()
                key=|state| state.id.clone()
                let:child
            >
                <Button  on_click=move |_| {
                    open_url(&child.src);
                }>{child.title}</Button>

            </For>
        </div>
    }
}
