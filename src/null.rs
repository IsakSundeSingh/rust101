// Rust does not have nulls, so how does it
// represent values that does not exist?

// In addition to structs (product types),
// Rust has enumerations (sum types).
// A value of Abc can only be A, B or C.
enum Abc {
    A,
    B,
    C,
}

// Enum variants in Rust can also carry data,
// such as Custom here.
// So a value of type Age can be `Underage`
// or `Custom(0)`, `Custom(1)`, ... or `Custom(255)`
enum Age {
    Underage,
    Custom(u8),
}

#[repr(C)]
enum CStyleEnum {
    A = 0,
    B,
    C,
}

// So you could represent a missing value as such,
// but this requires a lot of duplication for all types
enum MyInt {
    Null,
    Thing(i32),
}

// Let's make it generic!
enum MyOption<T> {
    None,
    Some(T),
}

// This is actually built-in and is called Option<T>, so let's use that instead.
