use crate::{
    myw::{
        self,
        table::{TabColumn, Table},
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
use myw::button;
use std::{fmt::Debug, sync::Arc};

#[derive(Clone, Debug)]
struct DataMock {
    id: u64,
    name: &'static str,
}
/// Renders the home page of your application.
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);

    // <myw::Gap h=8/>
    let data = vec![
        DataMock {
            id: 1,
            name: "张三",
        },
        DataMock {
            id: 2,
            name: "李四",
        },
        DataMock {
            id: 3,
            name: "王五",
        },
    ];
    let (data_vec, _set_data_vec) = signal(data);

    let col_vec: Vec<TabColumn<DataMock>> = vec![
        TabColumn {
            width: 50,
            title: "id",
            id: "id",
            view: Some(Arc::new(|data: DataMock| {
                ViewFn::from(move || {
                    view! { {data.id} }
                })
            })),
        },
        TabColumn {
            width: 300,
            title: "姓名",
            id: "name",
            view: Some(Arc::new(|data: DataMock| {
                ViewFn::from(move || {
                    view! { {data.name} }
                })
            })),
        },
        TabColumn {
            width: 150,
            title: "自定义列",
            id: "ctrl",
            view: Some(Arc::new(|data: DataMock| {
                let data_clone = data.clone();
                // 方法1：使用 ViewFn::from() - 最推荐的方式
                ViewFn::from(move || {
                    let data = data_clone.clone();
                    view! {
                        <button::I on:click=move |_| {
                            leptos::logging::log!("操作: {:?}", data);
                        }>
                            {format!("操作 {} - {}",data.id, data.name)}
                        </button::I>
                    }
                })
            })),
        },
    ];
    let (col_vec, _set_col_vec) = signal(col_vec);
    view! {
        <myw::Gap/>
        <Table
            data=data_vec
            col_vec=col_vec
            on_row_drop=Box::new(move |_|{})
            > </Table>
    }
}
