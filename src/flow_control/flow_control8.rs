// Fill in the blanks
fn main() {
    let mut n = 0;
    for i in 0..=100 {
        if __ {
            n += 1;
            continue;
        }

        break;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
