fn main() {
    let v = (let x = 3);

    assert!(v == 3);

    println!("Success!");
}
