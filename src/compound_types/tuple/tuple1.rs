fn main() {
    let _t0: (__, __) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (__, (__, __)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (__, __, __, __, __) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}
