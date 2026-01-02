use super::button;
use leptos::prelude::*;
#[slot]
#[derive(Debug)]
pub struct Tab {
    pub children: ChildrenFn,
    pub title: String,
    pub id: u64,
    #[prop(optional, into)] // optional 表示可选，into 允许自动转换
    pub icon: Option<ViewFn>,
    #[prop(optional)]
    pub click: Option<Callback<()>>,
}
impl Clone for Tab {
    fn clone(&self) -> Self {
        Tab {
            children: self.children.clone(),
            title: self.title.clone(),
            id: self.id,
            icon: self.icon.clone(),
            click: self.click.clone(),
        }
    }
}
#[component]
pub fn Tabset(
    #[prop(default=vec![])] tab: Vec<Tab>,
    id: RwSignal<u64>,
    #[prop(optional)] show_line: Option<bool>, // 改为 Option<bool>
) -> impl IntoView {
    let (select_id, set_select_id) = signal(0);
    let (arr_vec, _set_arr_vec) = signal(tab);
    let vec_first_vlue = id.get_untracked();
    // if let Some(tab) = value.get(0) {
    //     tab.id
    // } else {
    //     0 as u64
    // };
    // 明确处理默认值
    let should_show_line = move || show_line.unwrap_or(true);
    Effect::new(move |_| {
        // immediately prints "Value: 0" and subscribes to `a`
        set_select_id.set(id.get());
    });
    set_select_id.set(vec_first_vlue);
    view! {
        <div class="pr">
            <For
                each=move || arr_vec.get()
                key=|state| state.id.clone()
                let:child
            >

                <button::I
                    border=Signal::derive(move || if select_id.get() == child.id {"both".to_string()} else { "none".to_string()})
                    active= Signal::derive(move || select_id.get() == child.id)
                    on:click=move |_| {
                        set_select_id.set(child.id);
                        id.set(child.id);
                        if let Some(cb) = &child.click {
                            cb.run(());
                        }
                    }
                >
                    {
                        let icon_view = match child.icon.as_ref() {
                            Some(view_fn) => {
                                view! {
                                        {view_fn.run()}  // 图标
                                        <span style="line-height: 38px">{child.title.clone()}</span>  // 标题
                                }.into_any()
                            }
                            None => {
                                view! { <span style="line-height: 38px">{child.title.clone()}</span> }.into_any()
                            }
                        };
                        view! {
                            {icon_view}
                        }
                    }
                </button::I>
            </For>
            <Show when=should_show_line>
                <div style="border-bottom: 1px solid var(--myw-border); text-align: left; bottom: 0; left: 0; width: 100%;" class="pa"></div>
            </Show>
         </div>
        <div>
            {
                move || {
                    if let Some(tab) = arr_vec.get().iter().find(|tab| tab.id == select_id.get()) {
                        (tab.children)().into_any()
                    } else {
                        ().into_any()
                    }
                }
            }
        </div>
    }
}

#[component]
pub fn Tabs(
    tab: Signal<Vec<Tab>>,
    id: RwSignal<u64>,
    #[prop(optional)] show_line: Option<bool>, // 改为 Option<bool>
) -> impl IntoView {
    let (select_id, set_select_id) = signal(0);
    // let (arr_vec, _set_arr_vec) = signal(tab);
    let vec_first_vlue = id.get_untracked();
    let should_show_line = move || show_line.unwrap_or(true);
    set_select_id.set(vec_first_vlue);
    Effect::new(move |_| {
        set_select_id.set(id.get());
    });
    view! {
        <div class="pr">
            <For
                each=move || tab.get()
                key=|state| state.id.clone()
                let:child
            >

                <button::I
                    border=Signal::derive(move || if select_id.get() == child.id {"both".to_string()} else { "none".to_string()})
                    active= Signal::derive(move || select_id.get() == child.id)
                    on:click=move |_| {
                        set_select_id.set(child.id);
                        id.set(child.id);
                        if let Some(cb) = &child.click {
                            cb.run(());
                        }
                    }
                >
                    {
                        let icon_view = match child.icon.as_ref() {
                            Some(view_fn) => {
                                view! {
                                        {view_fn.run()}  // 图标
                                        <span style="line-height: 38px">{child.title.clone()}</span>  // 标题
                                }.into_any()
                            }
                            None => {
                                view! { <span style="line-height: 38px">{child.title.clone()}</span> }.into_any()
                            }
                        };
                        view! {
                            {icon_view}
                        }
                    }
                </button::I>
            </For>
            <Show when=should_show_line>
                <div style="border-bottom: 1px solid var(--myw-border); text-align: left; bottom: 0; left: 0; width: 100%;" class="pa"></div>
            </Show>
         </div>
        <div>
            {
                move || {
                    if let Some(tab) = tab.get().iter().find(|tab| tab.id == select_id.get()) {
                        (tab.children)().into_any()
                    } else {
                        ().into_any()
                    }
                }
            }
        </div>
    }
}
