// Fill in the blank
fn main() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= __ {
                break __;
            }

            continue 'outer;
        }
    }

    assert!(count == 30);

    println!("Success!");
}
