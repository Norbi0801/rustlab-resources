fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: __ = &arr[1..4];
    assert_eq!(slice, &[__]);

    println!("Success!");
}
