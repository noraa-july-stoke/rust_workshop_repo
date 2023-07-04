// ownership, borrowing, lifetimes, references, borrow checker, and pointers

fn bf1(s: &String) {
    println!("{s}");
}


fn bf2(s: String) {
    println!("{s}");
}

fn borrow_func() {
    let string1 = "Hello".to_string();
    bf1(&string1);
    println!("==============================");
    bf2(string1.clone());
}


fn move_func() {
    let str1 = "hello".to_string();
    let str2 = str1;
}


fn expr_func() {
    let x: i32 = 1;
    loop {
        break 1;
    };
}



pub fn main() {
    borrow_func();
    move_func();
}
