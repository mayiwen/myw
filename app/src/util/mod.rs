use leptos::web_sys::window;

pub fn open_url(url: &str) {
    if let Some(win) = window() {
        match win.open_with_url_and_target(url, "_blank") {
            Ok(opened_window) => {
                // 成功打开
                if opened_window.is_none() {
                    leptos::logging::warn!("Browser may have blocked popup for URL: {}", url);
                }
            }
            Err(e) => {
                leptos::logging::error!("Failed to open URL {}: {:?}", url, e);
            }
        }
    }
}
