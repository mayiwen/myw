use super::button;
use leptos::prelude::*;
use std::{
    cell::RefCell,
    fmt::Debug,
    rc::Rc,
    sync::{Arc, Mutex},
};
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
    /// 拖拽完成回调：改用 Box<dyn FnMut> 做类型擦除（核心修复）
    #[prop(optional)]
    on_row_drop: Option<Box<dyn FnMut((usize, usize)) + Send + 'static>>,
) -> impl IntoView {
    // 计算表格总宽度
    let table_total_width = move || {
        let total_width: u32 = col_vec.get().iter().map(|col| col.width).sum();
        if total_width == 0 {
            "100%".to_string()
        } else {
            format!("{}px", total_width)
        }
    };

    // 拖拽索引信号
    let (dragged_index, set_dragged_index) = signal::<Option<usize>>(None);
    // Arc<Mutex> 包装类型擦除后的回调
    let on_row_drop = Arc::new(Mutex::new(on_row_drop));

    view! {
        <div style="overflow: auto;">
            <table style=move || format!(
                    "border-collapse: collapse; table-layout: fixed; width: {};",
                    table_total_width()
                ) >
                <thead>
                    <tr style={"height: 40px"}>
                        {move || col_vec.get().iter().map(|col| view! {
                            <th style={format!("width: {}px; font-size: 16px; padding: 0 4px; border: 1px solid var(--myw-border);background-color: var(--myw-boxBc);", col.width)} class="ellipsis">{col.title}</th>
                        }).collect::<Vec<_>>()}
                    </tr>
                </thead>
                <tbody>
                    {move || data.get().into_iter().enumerate().map(|(current_row_idx, item)| {
                        let item = item.clone();
                        // 克隆 Arc（线程安全）
                        let on_row_drop = on_row_drop.clone();
                        let dragged_index = dragged_index.clone();

                        view! {
                            <tr
                                style={"height: 40px; "}
                                draggable="true"
                                on:dragstart=move |ev| {
                                    set_dragged_index.set(Some(current_row_idx));
                                }
                                on:dragover=move |ev| {
                                    ev.prevent_default();
                                }
                                on:drop=move |_| {
                                    if let Some(start_idx) = dragged_index.get() {
                                        if start_idx != current_row_idx {
                                            // 安全调用回调
                                            if let Ok(mut callback_opt) = on_row_drop.lock() {
                                                if let Some(ref mut callback) = *callback_opt {
                                                    callback((start_idx, current_row_idx));
                                                }
                                            }
                                        }
                                    }
                                }
                                on:dragend=move |_| {
                                    set_dragged_index.set(None);
                                }
                            >
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
                                        <td style=" border: 1px solid var(--myw-border); padding: 0 4px;" class="ellipsis">
                                            {cell_view}
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
