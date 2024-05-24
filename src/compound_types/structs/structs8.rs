// Fix errors to make it work
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Something".to_string(),
    };

    let _name = f.name;

    // ONLY modify this line
    println!("{}", __);
}
