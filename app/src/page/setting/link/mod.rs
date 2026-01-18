use std::sync::Arc;

use crate::{
    models::{link::Link, title::Title},
    myw::{
        self,
        button::Button,
        table::{TabColumn, Table},
        tabset::{Tab, Tabs},
    },
    util::open_url,
};
pub mod add;
use leptos::{prelude::*, reactive::spawn_local};
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    let arr_vec: RwSignal<Vec<Title>> = RwSignal::new(vec![]);
    let (link_vec, set_link_vec) = signal(vec![]);
    let get_link = move |id: u64| {
        spawn_local(async move {
            let res = crate::get_link(id).await;
            match res {
                Ok(res) => {
                    let id_temp: u64 = match res.get(0) {
                        Some(t) => t.id,
                        None => 0,
                    };
                    set_link_vec.set(res);
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

    let load_data = {
        get_link(id.get());
    };

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

    let col_vec: Vec<TabColumn<Link>> = vec![
        TabColumn {
            width: 50,
            title: "id",
            id: "id",
            view: Some(Arc::new(|data: Link| {
                ViewFn::from(move || {
                    view! { {data.id} }
                })
            })),
        },
        TabColumn {
            width: 300,
            title: "链接名称",
            id: "title",
            view: Some(Arc::new(|data: Link| {
                ViewFn::from(move || {
                    view! { {data.title.clone()} }
                })
            })),
        },
        TabColumn {
            width: 300,
            title: "链接路径",
            id: "src",
            view: Some(Arc::new(|data: Link| {
                ViewFn::from(move || {
                    view! { {data.src.clone()} }
                })
            })),
        },
        TabColumn {
            width: 100,
            title: "排序",
            id: "index",
            view: Some(Arc::new(|data: Link| {
                ViewFn::from(move || {
                    view! { {data.index} }
                })
            })),
        },
        TabColumn {
            width: 70,
            title: "标题编号",
            id: "title_id",
            view: Some(Arc::new(|data: Link| {
                ViewFn::from(move || {
                    view! { {data.title_id} }
                })
            })),
        },
        TabColumn {
            width: 100,
            title: "操作",
            id: "ctrl",
            view: Some(Arc::new(|data: Link| {
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

    let on_click_cb = move |_success: bool| {
        // 直接调用 load_data，跳过解包 reload_callback（更简洁）
        get_link(id.get());
    };
    let (col_vec, _set_col_vec) = signal(col_vec);
    view! {
        <myw::Gap/>
        {id}
        <h3>首页链接设置</h3>
        <myw::Gap/>
        <add::I on_click=on_click_cb id=id/>
        <div>
            <Tabs tab=tabs id=id />
        </div>
        <myw::Gap/>
        <Table data=link_vec col_vec=col_vec> </Table>
    }
}
