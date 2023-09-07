// A type alias can be written as such
type SomeType = i32;
// The type here 👆 doesn't matter for now,
// we just use it to focus on the ownership

fn function_owning_the(libs: SomeType) {
    // This function now owns the incoming parameter
    // and can do anything they want with it,
    // including not using it
}

fn borrowing(thing: &SomeType) {
    // This function is only borrowing the thing and
    // can only read it, not consume it/drop it/destroy it
    // and can not give ownership to someone else

    // This will not compile:
    // function_owning_the(thing)

    // An example of what this function can do is to print the value
    println!("{thing}");

    // It cannot modify it so this will not compile
    // thing.value = 23;

    // Also: the caller cannot modify the value while
    // this function borrows it
    // (e.g. using multiple threads)
}

fn exclusive_ownership(thing: &mut SomeType) {
    // This function has an exclusive reference,
    // often called a mutable reference,
    // because this function can mutate the value.

    // Mutating an exclusive reference is done by
    // assigning a value to the thing it references (points to).
    // * dereferences the reference
    *thing = 5;

    // our `thing` now references a value that is 5.

    // No one can read or modify the value thing points
    // to while this function runs.
    // This is checked compile time.
}

fn lifetimes() {
    // `x` doesn't exist yet
    {
        let x = 42; // `x` starts existing
        println!("x = {x}");
        // `x` stops existing
    }
    // `x` no longer exists
}

fn reference_lifetimes() {
    // `x` doesn't exist yet
    {
        let x = 42; // `x` starts existing
        let x_ref = &x; // `x_ref` starts existing - it borrows `x`
        println!("x_ref = {x_ref}");
        // `x_ref` stops existing
        // `x` stops existing
    }
    // `x` no longer exists
}

fn lifetimes_cannot_be_too_short() {
    let x_ref = {
        let x = 42;
        &x
    };
    println!("x_ref = {x_ref}");
    // error: `x` does not live long enough
}

fn exclusive_references_cannot_be_changed_by_others() {
    let mut x = 42;
    let x_ref = &x;
    x = 13;
    println!("x_ref = {x_ref}");
    // error: cannot assign to `x` because it is borrowed
}

fn difference_between_owned_and_borrowed_types() {
    let owned_string: String = String::from("hello");

    let borrowed_string: &str = "there";
    let borrowed_from_static: &'static str = borrowed_string;
    // String literals live for as long as the program does
    // as they are embedded in the binary. The two above have the
    // same lifetime, but the last example has an explicit lifetime.
}

fn lifetimes_are_type_parameters<'this_is_a_lifetime_type_name_and_every_identifier_can_be_used>(
    some_string: &'this_is_a_lifetime_type_name_and_every_identifier_can_be_used str,
) {
    // Any name can be used for lifetimes, but most of
    // the times you don't have to write them explicitly
    // Because of this, a lot of people just write 'a, 'b, and so on

    // 'static is a special lifetime that lives the longest of them all.
    // It is currently the only special lifetime
}

fn lifetimes_can_live_longer_than_others<'b, 'a: 'b>() {
    // The above means that we have to generic lifetimes
    // where 'a outlives 'b.
    // This is rarely required.
}

struct StoredReference {
    some_str: &str,
}
