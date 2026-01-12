use leptos::prelude::ServerFnError;
use serde::{Deserialize, Serialize};

type ServerFnResult<T> = Result<T, ServerFnError<String>>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub id: u64,
    pub title: String,
    pub index: i32,
}
