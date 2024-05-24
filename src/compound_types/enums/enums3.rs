// Fill in the blank and fix the error
enum Message {
    Quit,
    Move {__},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = __ { x: 2, y: 2 };

    if let Message::Move { x: a, y: b } = msg {
        assert_eq!(a, b);
    } else {
        panic!(__);
    }

    println!("Success!");
}
