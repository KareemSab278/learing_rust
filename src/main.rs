// mod memory_types_fn_vars;
// use memory_types_fn_vars::one; // :: means path access in rust

fn main() {
    // one();
    let mut demo = Example {
        text: String::from("i am text"), // if you get "expected `String`, found `&str`" error
        number: 28,
    };
    demo.increment_num();
    demo.read_num();
    demo.return_text();
}
// structs in rust
struct Example {
    text: String,
    number: i32,
}

impl Example {
    // impl = implementation for the struct you created. kinda like adding medthods to a class in JS
    // so just an object methods but outside the obj
    fn return_text(&self) {
        // cannot have a non mutable reference if you plan to mutate it. We're only reading so it is fine.
        println!("text: {}", self.text)
    }

    fn increment_num(&mut self) {
        self.number += 1;
    }

    fn read_num(&self) {
        println!("num: {}", self.number);
    }
}
