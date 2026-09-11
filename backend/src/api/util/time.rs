use sea_orm::sqlx::types::chrono;

pub async fn ssr_get_date() -> String {
    let now = chrono::Local::now();
    let date: String = now.format("%Y-%m-%d").to_string();
    let time: String = now.format("%H:%M:%S").to_string();
    format!("{} {}", date, time)
}
pub async fn ssr_get_date_num() -> String {
    let now = chrono::Local::now();
    let date_num: String = now.timestamp_millis().to_string();
    date_num
}