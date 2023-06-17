#![allow(dead_code)]
use web_view::*;
// Challenge: add decrement and reset buttons;
pub fn main() {
    web_view::builder()
        .title("My WebView")
        .content(Content::Html(r#"
        <html>
            <body>
                <h1>Hello, World!</h1>
                <div id='counter'>0</div>
                <button onclick="incrementCounter()">Increment</button>

                <script>
                    function incrementCounter() {
                        let counterElement = document.getElementById('counter');
                        counterElement.textContent = parseInt(counterElement.textContent) + 1;
                        external.invoke('counterUpdated');
                    }
                </script>
            </body>
        </html>
        "#))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, arg| {
            match arg {
                "counterUpdated" => {
                    println!("Counter was incremented in JavaScript");
                },
                _ => (),
            }
            Ok(())
        })
        .run()
        .unwrap();
}
