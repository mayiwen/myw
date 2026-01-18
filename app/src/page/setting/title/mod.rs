use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{
    myw::{
        self,
        button::Button,
        message::Message,
        modal::Modal,
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
    let is_open_delete: RwSignal<bool> = RwSignal::new(false);
    let is_open_update: RwSignal<bool> = RwSignal::new(false);
    let title_delete: RwSignal<String> = RwSignal::new("是否确认删除？".to_string());
    let title_update: RwSignal<String> = RwSignal::new("是否确认修改？".to_string());
    let is_open_delete_col = is_open_delete.clone();
    let is_open_update_col = is_open_update.clone();
    let delete_id: RwSignal<String> = RwSignal::new("删除id".to_string());
    let delete_title: RwSignal<String> = RwSignal::new("删除title".to_string());
    let update_id: RwSignal<String> = RwSignal::new("删除id".to_string());
    let update_title: RwSignal<String> = RwSignal::new("删除title".to_string());
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
            width: 100,
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
            view: Some(Arc::new(move |data: Title| {
                let data_clone = data.clone();
                // 方法1：使用 ViewFn::from() - 最推荐的方式
                ViewFn::from(move || {
                    let data = data_clone.clone();
                    let data_clone = data_clone.clone();
                    let is_open_delete = is_open_delete_col.clone();
                    let is_open_update = is_open_update_col.clone();
                    let delete_id = delete_id.clone();
                    let delete_title = delete_title.clone();
                    let data_title: String = data.title.clone();
                    view! {
                        <Button on_click=move |_| {
                            is_open_update.set(true);
                            update_id.set(data.id.to_string());
                            update_title.set(data.title.clone());
                        }>{format!("修改")}</Button> <myw::Gap w=4/>
                        <Button on_click=move |_| {
                            is_open_delete.set(true);
                            delete_id.set(data_clone.id.to_string());
                            delete_title.set(data_clone.title.clone());
                        }>"删除"</Button>
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

    let delete_confirm = move |_| {
        is_open_delete.set(false);
        let id = delete_id.get(); // 获取到要删除的id;
        let id = str_to_i64(&id);
        match id {
            Ok(id) => {
                spawn_local(async move {
                    let res = crate::title_delete(id, crate::get_global_token_with_bearer()).await;
                    match res {
                        Ok(res) => {
                            let message: RwSignal<Vec<Message>> =
                                use_context::<RwSignal<Vec<Message>>>()
                                    .expect("Message context must exist");
                            // 使用 update 方法修改 RwSignal 中的值
                            let another_msg4 = Message {
                                t: myw::message::MessageType::INFO,
                                m: "删除成功".to_string(),
                            };
                            // 向 Vec 中添加新元素
                            message.update(|msgs| {
                                msgs.push(another_msg4); // push 是 Vec 的标准添加方法
                            });
                            load_data();
                        }
                        Err(_) => {
                            let message: RwSignal<Vec<Message>> =
                                use_context::<RwSignal<Vec<Message>>>()
                                    .expect("Message context must exist");
                            // 使用 update 方法修改 RwSignal 中的值
                            let another_msg4 = Message {
                                t: myw::message::MessageType::INFO,
                                m: "删除失败".to_string(),
                            };
                            // 向 Vec 中添加新元素
                            message.update(|msgs| {
                                msgs.push(another_msg4); // push 是 Vec 的标准添加方法
                            });
                        }
                    }
                });
            }
            Err(_) => {}
        }
    };

    let update_confirm = move |_| {
        let id = update_id.get(); // 获取到要删除的id;
        let title = update_title.get().clone(); // 获取到要删除的id;
        let id = str_to_i64(&id);
        match id {
            Ok(id) => {
                spawn_local(async move {
                    let res =
                        crate::title_update(id, title, crate::get_global_token_with_bearer()).await;
                    match res {
                        Ok(res) => {
                            let message: RwSignal<Vec<Message>> =
                                use_context::<RwSignal<Vec<Message>>>()
                                    .expect("Message context must exist");
                            // 使用 update 方法修改 RwSignal 中的值
                            let another_msg4 = Message {
                                t: myw::message::MessageType::INFO,
                                m: "修改成功".to_string(),
                            };
                            // 向 Vec 中添加新元素
                            message.update(|msgs| {
                                msgs.push(another_msg4); // push 是 Vec 的标准添加方法
                            });
                            is_open_update.set(false);
                            load_data();
                        }
                        Err(_) => {
                            let message: RwSignal<Vec<Message>> =
                                use_context::<RwSignal<Vec<Message>>>()
                                    .expect("Message context must exist");
                            // 使用 update 方法修改 RwSignal 中的值
                            let another_msg4 = Message {
                                t: myw::message::MessageType::INFO,
                                m: "修改失败".to_string(),
                            };
                            // 向 Vec 中添加新元素
                            message.update(|msgs| {
                                msgs.push(another_msg4); // push 是 Vec 的标准添加方法
                            });
                        }
                    }
                });
            }
            Err(_) => {}
        }
    };
    let drag = move |start: usize, end: usize| {
        let message: RwSignal<Vec<Message>> =
            use_context::<RwSignal<Vec<Message>>>().expect("Message context must exist");
        let another_msg4 = Message {
            t: myw::message::MessageType::INFO,
            m: format!("{}-{}", start, end),
        };
        message.update(|msgs| {
            msgs.push(another_msg4); // push 是 Vec 的标准添加方法
        });
        let link_v = data_vec.clone().get().clone();
        let mut nav_title_one = link_v;
        let item_save = nav_title_one.remove(start);
        nav_title_one.insert(end, item_save);

        spawn_local(async move {
            let res = crate::title_sort(nav_title_one, crate::get_global_token_with_bearer()).await;
            match res {
                Ok(res) => {
                    let message: RwSignal<Vec<Message>> = use_context::<RwSignal<Vec<Message>>>()
                        .expect("Message context must exist");
                    let another_msg4 = Message {
                        t: myw::message::MessageType::INFO,
                        m: "修改成功".to_string(),
                    };
                    message.update(|msgs| {
                        msgs.push(another_msg4); // push 是 Vec 的标准添加方法
                    });
                    let id_i64 = id.get() as u64;
                    let id_u64 = if id_i64 >= 0 {
                        id_i64 as u64 // 仅当非负时转换
                    } else {
                        0u64
                    };
                    load_data();
                }
                Err(_) => {
                    let message: RwSignal<Vec<Message>> = use_context::<RwSignal<Vec<Message>>>()
                        .expect("Message context must exist");
                    let another_msg4 = Message {
                        t: myw::message::MessageType::INFO,
                        m: "修改失败".to_string(),
                    };
                    message.update(|msgs| {
                        msgs.push(another_msg4); // push 是 Vec 的标准添加方法
                    });
                }
            }
        });
    };
    view! {
        <myw::Gap/>
        <h3>首页标题设置</h3>
        <add::I on_click=on_click_cb />
        <myw::Gap/>
        <Table data=data_vec col_vec=col_vec on_row_drop=Box::new(move |(start, end)|{
            drag(start, end);
        })> </Table>
        <Modal is_open=is_open_delete title=title_delete  on_click=delete_confirm >
            <div style="width: 200px;">
                <input class="myw-input" placeholder="id" bind:value=delete_id disabled/>
                <myw::Gap h=16/>
                <input class="myw-input" placeholder="标题" bind:value=delete_title disabled/>
            </div>
        </Modal>
        <Modal is_open=is_open_update title=title_update  on_click=update_confirm >
            <div style="width: 200px;">
                <input class="myw-input" placeholder="id" bind:value=update_id disabled/>
                <myw::Gap h=16/>
                <input class="myw-input" placeholder="标题" bind:value=update_title />
            </div>
        </Modal>
    }
}
fn str_to_i64(s: &str) -> Result<i64, Box<dyn std::error::Error>> {
    // parse 返回 Result<i64, ParseIntError>，直接返回即可
    let num = s.parse::<i64>()?;
    Ok(num)
}
