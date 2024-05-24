// Fill in the blank and fix the errors
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs: [__] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(__, __, __),
    ];

    for msg in msgs {
        __
    }
}

fn show_message(msg: Message) {
    println!("{:?}", msg);
}
