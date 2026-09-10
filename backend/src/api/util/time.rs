use sea_orm::sqlx::types::chrono;

pub async fn ssr_get_date() -> String {
    let now = chrono::Local::now();
    let date: String = now.format("%Y-%m-%d").to_string();
    let time: String = now.format("%H:%M:%S").to_string();
    format!("{} {}", date, time)
}