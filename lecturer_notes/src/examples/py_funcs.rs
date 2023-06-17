#![allow(dead_code)]
use std::fs::File;
use std::io::Write;

macro_rules! python_function {
    ($fn_name:ident, $arg:ident -> $body:expr) => {
        format!("def {}({}):\n    return {}\n", stringify!($fn_name), stringify!($arg), $body)
    };
}

pub fn main() {
    let square = python_function!(square, x -> "x**2");
    let add_one = python_function!(add_one, x -> "x + 1");
    let content = format!("{}\n{}", square, add_one);

    let mut file = File::create("functions.py").expect("Cannot create file");
    file.write_all(content.as_bytes()).expect("Cannot write to file");

    println!("Python functions written to functions.py");
}
