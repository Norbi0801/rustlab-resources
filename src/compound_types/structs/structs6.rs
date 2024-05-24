// Fill the blank to make the code work
struct User {
    active: __,
    username: __,
    email: __,
    sign_in_count: u64,
}
fn main() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(__);

    println!("Success!");
}

fn set_email(u: User) -> User {
    User {
        email: String::from("hellorust@rust.cargo"),
        __
    }
}
