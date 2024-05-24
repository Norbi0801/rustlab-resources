// Fix the error and fill the blanks
struct Color(i32, i32, i32);
struct Point;
fn main() {
    let v = Point(0, 127, 255);
    check_color(v);

    println!("Success!");
}

fn check_color(c: Color) {
    let Color(__) = c;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}
