pub fn main() {

    let data = vec![1,2,3,4,5];
    let closure1 = || println!("{data:?}");
    let closure2 = move || println!("{data:?}");

    let mut string1 = "a1b2c3d4e5".to_string();
    string1.retain(|c: char| c.is_alphabetic());

    println!("{string1:?}");

}
