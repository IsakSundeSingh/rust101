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
