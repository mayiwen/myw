use leptos::prelude::*;

pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    println!("Adding todo: {}", title);
    Ok(())
}
