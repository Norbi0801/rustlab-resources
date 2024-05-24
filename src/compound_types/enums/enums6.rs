use crate::compound_types::enums::enums6::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => __,
            Nil => __,
        }
    }

    fn stringify(__self) -> String {
        match __self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.__)
            }
            Nil => format!(__),
        }
    }
}

fn main() {
    let mut list = List::new(); // Nil

    list = list.prepend(1); // Cons(1, Nil)
    list = list.prepend(2); // Cons(2, Cons(1, Nil))
    list = list.prepend(3); // Cons(3, Cons(2, Cons(1, Nil)))

    println!("linked list has length: {}", __); // 3
    println!("{}", __); // 3, 2, 1, Nil
}
