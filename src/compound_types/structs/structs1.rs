// Fix the error
struct Person {
    name: &str,
    age: u128,
    hobby: String,
}
fn main() {
    let age = '30';
    let p = Person {
        name: String::from("sunface"),
        hobby: String::from("coding"),
        age,
    };

    println!("Success!");
}
