use leptos::prelude::*;

#[component]
pub fn I() -> impl IntoView {
    view! {
        <div style="text-align: center">
            "你好"
        </div>
    }
}

// #[server]
// pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
//     // 服务器逻辑
//     println!("Adding todo: {}", title);
//     Ok(())
// }
