// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;

    println!("v = {}", v);
    assert!(v == 0);

    println!("Success!");
}
