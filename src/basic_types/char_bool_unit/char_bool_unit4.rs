// Make it work
fn main() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}
