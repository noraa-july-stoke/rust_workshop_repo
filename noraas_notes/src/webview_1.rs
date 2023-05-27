#![allow(dead_code)]

use web_view::*;
pub fn main() {
    web_view::builder()
        .title("My Webview")
        .content(Content::Html(
            "<html><body><h1>Hello, World!</h1></body></html>",
        ))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
