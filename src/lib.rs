#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_arbitrary_self_type)]
mod null;
mod owners;

// A comment!

// A function!
fn empty_function() {}

fn lets_talk_about_variables() {
    let x; // declare "x"
    x = 123; // assign 123 to "x"

    // Can also be written as
    let x = 123;

    // The type is inferred, but can also be written explicitly
    let x: i32 = 123; // `i32` is a signed 32-bit integer

    // Variables can be shadowed and integers can fit into multiple sizes
    let x: i64 = 123;

    // there's i8, i16, i32, i64, i128
    //    also u8, u16, u32, u64, u128 for unsigned

    // But integer literals are checked at compile-time to see if they are too big to fit in their type
    // let x: i8 = 256;
    // Produces this compiler output:
    // error: literal out of range for `i8`
    //   --> src/lib.rs:25:17
    //    |
    // 25 |     let x: i8 = 256;
    //    |                 ^^^
    //    |
    //    = note: the literal `256` does not fit into the type `i8` whose range is `-128..=127`
    //    = help: consider using the type `i16` instead
    //    = note: `#[deny(overflowing_literals)]` on by default

    // Variable names are actually bindings, where underscore (`_`) means to throw away the value
    let _ = 123;

    // Starting with an underscore is convention for unused values
    let _going_to_use_this_later_maybe = 321;
}

fn lets_move_on_to_other_types() {
    // Rust supports tuples
    let pair: (char, i32) = ('a', 17);

    // Can be extracted like this
    let left = pair.0;
    let right = pair.1;

    // Or just destructured directly
    let (left, right) = some_func();

    // And you can discard items easily
    let (_, i_want_this_one) = some_func();
}

// Function arguments and return values are written with colons and a returning arrow
fn add(first: i32, second: i32) -> i32 {
    return first + second;
}

fn simpler_add(first: i32, second: i32) -> i32 {
    first + second
    // Note the missing semicolon 👆
    // A semicolon marks a statement, returning nothing,
    // while a missing semicolon means an expression, which is implicitly returned
    // This is the idiomatic way to write it
}

fn add_and_square(thing: i32) -> i32 {
    let added = thing + 1;
    added * added
}

fn returns_nothing() {
    let x = 0;
    // Note  👆 the semicolon, marking a statement that returns nothing
    // There is also no return type on the function
    // And no return statement or expression without semicolon 👇
}

fn printer() {
    println!("Hello from Rust!");
}

// This prints "in", then "out"
fn blocks() {
    let x = "out";
    {
        // this is a different `x`
        let x = "in";
        println!("{x}");
    }
    println!("{x}");
}

fn blocks_are_expressions() {
    // This
    let x = 123;

    // is equivalent to this:
    let x = { 123 };

    // And blocks can also have statements, like a function, returning the last expression
    // So this is equivalent to the above:
    let x = {
        let hundred = 100;
        let twenty = 20;
        let three = 3;
        hundred + twenty + three
    };
}

fn what_if_else_can_do(this_is_true: bool) -> i32 {
    if this_is_true {
        100
    } else {
        -100
    }
}

fn match_can_do_better(what_is_this: bool) -> i32 {
    // Match is like a switch-case from other languages
    match what_is_this {
        true => 100,
        false => -100,
    }
}

fn methods_and_imports() {
    // Methods are accessed by a dot
    let biggest = 100.max(-100); // This is 100

    // The double-colon is similar, but works on namespaces
    // `std` is a crate (a library), `cmp` is a module (a file), and `max` is a function
    let biggest = std::cmp::max(100, -100); // This is 100

    // You can use `use` to import things into scope instead of writing the whole thing
    use std::cmp::max;
    let biggest = max(100, -100); // This is the same function call as the two above and is 100

    // A glob can be used to import everything (this is often messy and not idiomatic)
    use std::cmp::*;

    // Or better, just use curly brackets to import just what you need
    // use std::cmp::{max, min};

    // Imports are scoped, so the block below could import a max from another namespace and not clash
    let x = {
        use std::cmp::max;
        max(1223, 123232)
    };

    let string = "hello";

    // These are equivalent
    let string_len = string.len();
    let string_len = str::len(string);
    // Types like    👆 which are used often are in the prelude and don't need importing
    // And since types are namespaces, we can use `::` to access methods on them
    let biggest = i32::max(-100, 100); // Also 100, also equivalent to the comparison at the top
}

// Let's talk about types baby
struct SomeData {
    my_int: i32,
    is_mine: bool,
}

fn creating_values() {
    // This is how we create a value of the type
    let some_data = SomeData {
        my_int: 123,
        is_mine: true,
    };
    // The order of the fields does not matter
    let some_data = SomeData {
        is_mine: true,
        my_int: 123,
    };

    // You can create a new struct from the fields of another
    let not_my_data = SomeData {
        is_mine: false,
        ..some_data
    };

    let copy_of_some_data = SomeData { ..some_data };

    // Structs can also be deconstructed
    let SomeData { my_int, is_mine } = some_data;
    // Values are: 👆 123  👆 true
}

fn print_if_mine(data: SomeData) {
    if data.is_mine {
        println!("My data is: {}", data.my_int);
    }

    if let SomeData {
        is_mine: true,
        my_int,
    } = data
    {
        println!("My data is: {my_int}");
    }
}

// Functions can also be written as methods, using self as a standin for the name
impl SomeData {
    fn print_if_mine(self: SomeData) {
        if self.is_mine {
            println!("My data is: {}", self.my_int);
        }
    }

    // Function signature could also be written as:
    //
    // fn print_if_mine(self: Self)
    //
    // or just the most common and idiomatic version:
    //
    // fn print_if_mine(self)
}

fn some_func() -> (i32, i32) {
    todo!()
}
