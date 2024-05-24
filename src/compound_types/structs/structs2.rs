struct Unit;

trait SomeTrait {
    fn describe(&self) -> String;
    fn perform_action(&self);
}

impl SomeTrait for Unit {
    fn describe(&self) -> String {
        String::from("I am a Unit struct.")
    }

    fn perform_action(&self) {
        println!("Unit is performing an action!");
    }
}

fn main() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

fn do_something_with_unit(__) {
    println!("{}", __.describe());
    __.perform_action();
}
