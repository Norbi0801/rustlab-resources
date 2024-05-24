// Fill the blank and fix the error without adding/removing new line
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let age = 18;
    let mut p = Person {
        __: String::from("sunface"),
        __,
    };

    p.__ = 30;

    // Fill the blank
    p.__ = String::from("sunfei");

    println!("Success!");
}
