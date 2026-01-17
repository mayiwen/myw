use std::sync::Arc;

use crate::{
    myw::{
        self,
        button::Button,
        table::{TabColumn, Table},
        tabset::{Tab, Tabset},
    },
    util::open_url,
};

use crate::models::title::Title;
use leptos::{prelude::*, reactive::spawn_local};
pub mod add;
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);

    // <myw::Gap h=8/>
    let data = vec![];
    let (data_vec, set_data_vec) = signal(data);

    let col_vec: Vec<TabColumn<Title>> = vec![
        TabColumn {
            width: 50,
            title: "id",
            id: "id",
            view: Some(Arc::new(|data: Title| {
                ViewFn::from(move || {
                    view! { {data.id} }
                })
            })),
        },
        TabColumn {
            width: 300,
            title: "标题名称",
            id: "title",
            view: Some(Arc::new(|data: Title| {
                ViewFn::from(move || {
                    view! { {data.title.clone()} }
                })
            })),
        },
        TabColumn {
            width: 70,
            title: "排序",
            id: "index",
            view: Some(Arc::new(|data: Title| {
                ViewFn::from(move || {
                    view! { {data.index} }
                })
            })),
        },
        TabColumn {
            width: 100,
            title: "操作",
            id: "ctrl",
            view: Some(Arc::new(|data: Title| {
                let data_clone = data.clone();
                // 方法1：使用 ViewFn::from() - 最推荐的方式
                ViewFn::from(move || {
                    let data = data_clone.clone();
                    view! {
                        <Button on_click=move |_| {}>{format!("修改")}</Button> <myw::Gap w=4/>
                        <Button on_click=move |_| {}>"删除"</Button>
                    }
                })
            })),
        },
    ];
    let (col_vec, _set_col_vec) = signal(col_vec);

    Effect::new(move |_| {
        spawn_local(async move {
            let res = crate::get_title().await;
            match res {
                Ok(res) => {
                    let id_temp: u64 = match res.get(0) {
                        Some(t) => t.id,
                        None => 0,
                    };
                    set_data_vec.set(res);
                    // id.set(id_temp);
                    // get_link(id_temp)
                }
                Err(_) => {}
            }
        });
    });
    view! {
        <myw::Gap/>
        <h3>首页标题设置</h3>
        <add::I/>
        <myw::Gap/>
        <Table data=data_vec col_vec=col_vec> </Table>
    }
}
