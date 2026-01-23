use crate::models::SettingTab;
use crate::myw;
use crate::myw::button::Button;
use crate::myw::icon;
use crate::util::open_url;
use leptos::web_sys::{Document, HtmlElement, Window};

use leptos::prelude::*;
#[component]
pub fn MayiwenBeiAn() -> impl IntoView {
    let (count, set_count) = signal(0);
    // 2. 点击事件处理逻辑（完全还原原有功能）
    let toggle_theme = move || {
        // 更新计数器
        set_count.update(|c| *c += 1);

        // 直接获取 html 根元素（不再依赖外部函数，避免找不到的问题）
        if let Some(html_el) = leptos::web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.document_element())
        {
            // 切换主题属性
            let theme = if count.get() % 2 == 1 {
                "black"
            } else {
                "none"
            };
            if let Err(e) = html_el.set_attribute("data-mayiwen-theme", theme) {
                // log::error!("Failed to set theme attribute: {:?}", e);
            }
        }
    };
    view! {
        <div style="text-align: center; margin-top: 40px">
            <div>
                <Button  on_click=move |_| {
                    toggle_theme()
                }><icon::Theme/></Button> <myw::Gap w=8/>
                <Button  on_click=move |_| {
                        let tab: Option<RwSignal<SettingTab>> = use_context::<RwSignal<SettingTab>>();
                        match tab {
                            Some(v) => {
                                v.update(|st| {
                                    st.value = 2;
                                });
                            }
                            None => {}
                        }
                        let global_nav = use_context::<RwSignal<Option<crate::NavFn>>>().expect("请确保 App 组件已提供全局导航信号");
                        global_nav.with(|nav_opt| {
                        if let Some(nav) = nav_opt {
                            nav("/setting"); // 调用 App 封装的导航逻辑
                        };
                    });
                }><icon::Bookmark/></Button> <myw::Gap w=8/>
                <Button  on_click=move |_| {
                    open_url("https://github.com/mayiwen");
                }><icon::Github/></Button><myw::Gap w=8/>
                <Button  on_click=move |_| {
                    // open_url(&child.src);
                     open_url("https://gitlink.org.cn/mayiwen");
                }><icon::GitLink/></Button>
            </div>
            <myw::Gap h=20/>


            <div style="text-align: center;  ">
                <a href="http://mayiwen.com/" target="_blank" style="color:#666; text-decoration: none">mayiwen.com</a>
            </div>
            <div style="text-align: center; margin-top: -0px;">
                <a href="https://beian.miit.gov.cn/" target="_blank" style=" color:#666;">豫ICP备2022018473号-1</a>
            </div>
            <div style="text-align: center; margin-top: -3px;">
                <a target="_blank" href="http://www.beian.gov.cn/portal/registerSystemInfo?recordcode=41162402000184"
                style="position: relative; ">
                <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAUCAMAAAC6V+0/AAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAK7OHOkAAACHUExURUdwTOrNevfowO/CcfLiqenQk5uNe+K0WeCvYvnvwvrkjJhxWd2kVdyxfsqEW8N/PtGudurCXAICcNVAJNqURskGCf3LVu/NafrdbNEgFe+3TOimRt9kL9Z9N3N9icy9fEcRQPB+M2VcacJWKaulfLiNU45gOg4lkFYuUOSVOI4II/0sD6WFVa28QBwAAAARdFJOUwDsXcJAj+/+5iG1/MquXZxvQ0cHGAAAAQlJREFUGNNN0NlywjAMBVCyOiFQalte4t1ZYeD/v6920zLoQQ9ndD2yTqej+rJty/70WfV12LbtNVzLD1zDNpuIlrC+qa/uQ2SUMr3cz/+GwhTHcaQj4y4Ux8N1S16GJaLMeNL2R7glK2UjzeyJO2f91gtZI5sBwxjTJG8SNg+/T3ymxhqQE/EqY2fFTuZIqaHGkF0+ul9UjrCnmW18MuK4zXgF7vfAWNqJBuIlyvEKkHSEkHBPzUmkq4QFwEP4wS1omhzWoIuEJdJgueRcp2YB4TpvX2JsQWCEhQAQRX18s8NJcmkkcHfgWQqEEivQaVr+3alBiiulNMZKFM37ol+3y6XrLlVzZH8APkEYkQTXpHkAAAAASUVORK5CYII=" style="position: absolute; left: -20px; top: 0px" />
                <p
                    style="height:20px;line-height:20px; color:#666; display: inline-block; margin: 0; text-decoration: underline; ">
                    豫公网安备 41162402000184号</p>
                </a>
            </div>
            <div style="height: 30px">" "</div>
        </div>
    }
}
