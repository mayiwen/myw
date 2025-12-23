use leptos::prelude::*;
use leptos::server_fn::ServerFnError;

#[server]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    println!("Adding todo: {}", title);
    Ok(())
}
