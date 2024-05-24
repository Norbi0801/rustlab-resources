// Fix the errors
enum Number {
    Two,
    One,
    Zero,
}

enum Number1 {
    Zero = 0,
    Two,
    One,
}

// C-like enum
enum Number2 {
    Two = 2,
    Zero = 0,
    One = 1,
}

fn main() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);

    println!("Success!");
}
