use super::button;
use leptos::prelude::*;
use std::{fmt::Debug, sync::Arc};
// pub struct TabColumn<T> {
//     pub width: u32,
//     pub title: &'static str,
//     pub id: &'static str,
//     pub view: Option<Box<dyn Fn(T) -> ViewFn>>,
// }
// 为 T 添加 Send 约束
#[derive(Clone)]
pub struct TabColumn<T: Clone + 'static> {
    pub width: u32,
    pub title: &'static str,
    pub id: &'static str,
    // 添加 Send 约束
    // pub view: Option<Box<dyn Fn(T) -> ViewFn + Send + Sync>>,
    pub view: Option<Arc<dyn Fn(T) -> ViewFn + Send + Sync + 'static>>,
}
// 手动实现 Debug for TabColumn
impl<T: Clone + 'static> Debug for TabColumn<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TabColumn")
            .field("width", &self.width)
            .field("title", &self.title)
            .field("id", &self.id)
            .field(
                "view",
                &match &self.view {
                    Some(_) => "Some(view_fn)",
                    None => "None",
                },
            )
            .finish()
    }
}

#[derive(Clone, Debug)]
struct DataMock {
    id: u64,
    name: &'static str,
}

#[component]
pub fn Table<T: Clone + 'static + Debug + Send + Sync>(
    data: ReadSignal<Vec<T>>,
    col_vec: ReadSignal<Vec<TabColumn<T>>>,
) -> impl IntoView {
    view! {
        <div style="overflow: auto;">
        <table style="border-collapse: collapse;" >
            <thead>
                <tr style={"height: 40px"}>
                    {move || col_vec.get().iter().map(|col| view! {
                        <th style={format!("width: {}px; font-size: 16px; padding: 0 4px; border: 1px solid var(--myw-border);background-color: var(--myw-boxBc);", col.width)}>{col.title}</th>
                    }).collect::<Vec<_>>()}
                </tr>
            </thead>
            <tbody>
                {move || data.get().into_iter().enumerate().map(|(_row_index, item)| {
                    let item = item.clone();
                    view! {
                        <tr style={"height: 40px;  "}>
                            {col_vec.get().iter().enumerate().map(|(_col_index, col)| {
                            let cell_view = match &col.view {
                                Some(view_fn) => {
                                    let view_fn_instance = view_fn(item.clone());
                                    view_fn_instance.run()
                                }
                                None => {
                                    view! { <span>{col.id}</span> }.into_view().into_any()
                                }
                            };

                            view! {
                                <td style=" border: 1px solid var(--myw-border); padding: 0 4px;">
                                    {cell_view}  // 这里会自动转换为合适的类型
                                </td>
                            }
                        }).collect::<Vec<_>>()}
                        </tr>
                    }
                }).collect::<Vec<_>>()}
            </tbody>
        </table>
        </div>
    }
}

// #[component]
// pub fn Test() -> impl IntoView {
//     let data = vec![
//         DataMock {
//             id: 1,
//             name: "张三",
//         },
//         DataMock {
//             id: 2,
//             name: "李四",
//         },
//         DataMock {
//             id: 3,
//             name: "王五",
//         },
//     ];
//     let (data_vec, _set_data_vec) = signal(data);

//     let col_vec: Vec<TabColumn<DataMock>> = vec![
//         TabColumn {
//             width: 100,
//             title: "id",
//             id: "id",
//             view: Some(Arc::new(|data: DataMock| {
//                 ViewFn::from(move || {
//                     view! { {data.id} }
//                 })
//             })),
//         },
//         TabColumn {
//             width: 100,
//             title: "姓名",
//             id: "name",
//             view: Some(Arc::new(|data: DataMock| {
//                 ViewFn::from(move || {
//                     view! { {data.id} }
//                 })
//             })),
//         },
//         TabColumn {
//             width: 100,
//             title: "操作",
//             id: "ctrl",
//             view: Some(Arc::new(|data: DataMock| {
//                 let data_clone = data.clone();
//                 // 方法1：使用 ViewFn::from() - 最推荐的方式
//                 ViewFn::from(move || {
//                     let data = data_clone.clone();
//                     view! {
//                         <button::I on:click=move |_| {
//                             leptos::logging::log!("操作: {:?}", data);
//                         }>
//                             {format!("操作 {}", data.name)}
//                         </button::I>
//                     }
//                 })

//                 // 方法2：或者更简单，让闭包自动转换为 ViewFn
//                 // let view_closure = move || {
//                 //     let data = data.clone();
//                 //     view! {
//                 //         <button on:click=move |_| {
//                 //             leptos::logging::log!("操作: {:?}", data);
//                 //         }>
//                 //             {format!("操作 {}", data.name)}
//                 //         </button>
//                 //     }
//                 // };
//                 // view_closure.into()
//             })),
//         },
//     ];
//     let (col_vec, _set_col_vec) = signal(col_vec);
//     view! {
//         <I
//             data=data_vec
//             col_vec=col_vec
//         ></I>
//     }
// }
// thread_local! {}
