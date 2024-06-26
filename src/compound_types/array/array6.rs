// Fix the error
fn main() {
    let names: [String; 2] = [String::from("Sunfei"), "Sunface".to_string()];

    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(2).unwrap();

    // But indexing is not safe
    let _name1 = &names[2];

    println!("Success!");
}
