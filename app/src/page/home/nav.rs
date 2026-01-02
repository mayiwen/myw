use std::sync::Arc;

use crate::myw::tabset::{Tab, Tabs, Tabset};
use crate::Title;
use leptos::prelude::*;

#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    let arr_vec: RwSignal<Vec<Title>> = RwSignal::new(vec![]);

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
                    click: None,
                }
            })
            .collect::<Vec<Tab>>()
    });

    let button_click = move |_| {
        arr_vec.set(vec![
            Title {
                id: 3,
                title: "你好3".to_string(),
            },
            Title {
                id: 4,
                title: "你好4".to_string(),
            },
        ]);
        id.set(4);
    };
    view! {
        <button on:click=button_click>你好</button>
        <div style="max-width: 1200px; width: 100%; padding: 0px 4px; margin: auto">
            <Tabs tab=tabs id=id />
        </div>
    }
}
