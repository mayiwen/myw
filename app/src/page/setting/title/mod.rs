use std::{cell::RefCell, rc::Rc, sync::Arc};

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
    // 1. 重构 load_data 为【无参闭包】，去掉多余的 |_| 参数
    //    同时克隆 set_data_vec 到异步闭包，避免所有权提前转移
    let load_data = {
        let set_data_vec = set_data_vec.clone();
        move || {
            let set_data_vec = set_data_vec.clone();
            spawn_local(async move {
                let res = crate::get_title().await;
                match res {
                    Ok(res) => {
                        let id_temp: u64 = res.get(0).map(|t| t.id).unwrap_or(0); // 简化写法
                        set_data_vec.set(res);
                        // id.set(id_temp);
                        // get_link(id_temp)
                    }
                    Err(_) => {}
                }
            });
        }
    };

    // 2. 修正 Effect 调用（Leptos 新版推荐 create_effect，旧版 Effect::new 也需适配无参）
    //    组件挂载时自动执行一次 load_data
    Effect::new(move |_| {
        load_data(); // 无参调用，符合 Effect 要求
    });

    let on_click_cb = move |_success: bool| {
        // 直接调用 load_data，跳过解包 reload_callback（更简洁）
        load_data();
    };
    view! {
        <myw::Gap/>
        <h3>首页标题设置</h3>
        <add::I on_click=on_click_cb />
        <myw::Gap/>
        <Table data=data_vec col_vec=col_vec> </Table>
    }
}
