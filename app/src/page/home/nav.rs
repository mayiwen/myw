use std::sync::Arc;

use crate::myw::tabset::{Tab, Tabset};
use crate::Title;
use leptos::prelude::*;

#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    let arr_vec = RwSignal::new(vec![
        Title {
            id: 0,
            title: "你好".to_string(),
        },
        Title {
            id: 1,
            title: "你好2".to_string(),
        },
    ]);

    let tabs = move || {
        arr_vec
            .get()
            .into_iter()
            .map(|title| {
                let id = title.id;
                // 正确创建 ChildrenFn
                let children_fn: ChildrenFn = Arc::new(move || view! { "111" }.into_any());

                Tab {
                    children: children_fn,
                    title: title.title.clone(),
                    id: title.id,
                    icon: None,
                    click: None,
                }
            })
            .collect::<Vec<Tab>>()
    };
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
        id.set(1);
    };
    view! {
        <button on:click=button_click>你好</button>
        <Tabset tab=tabs() id=id />
    }
}
