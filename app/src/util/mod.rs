use leptos::web_sys::window;
pub fn open_url(url: &str) {
    if let Some(win) = window() {
        // 打开新窗口
        let _ = win.open_with_url(url);
    }
}
