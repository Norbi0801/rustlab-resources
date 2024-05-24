fn main() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];

    // Fill the blank
    assert!(std::mem::size_of_val(&arr) == __);

    println!("Success!");
}
