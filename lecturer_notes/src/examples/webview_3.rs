#![allow(dead_code)]
use web_view::*;

pub fn main() {
    web_view::builder()
        .title("My WebView")
        .content(Content::Url("https://www.github.com/"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
