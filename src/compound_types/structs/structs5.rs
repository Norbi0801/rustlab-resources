// Fill the blank
#[derive(__)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Arfan");
    let age = 28;
    let arfan = build_person(name, age);

    println!("Person 'arfan': {}", arfan);
    println!("Success!");
}

fn build_person(name: String, age: u8) -> __ {
    __
}
