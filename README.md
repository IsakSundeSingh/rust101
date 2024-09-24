---
marp: true
theme: gaia
transition: melt
---

<!-- _class: lead -->
<!-- _color: orange -->

# Rust 101

---

# Agenda

- Variables
- Functions
- Data types
- Pattern matching

---

# Variables

Or really, _bindings_. There are no variables, only bindings to values. That binding may be immutable or mutable

```rust
let a = 123;
a = 321; // <- Produces compilation error
```

---

# ~~Variables~~ Bindings

```
error[E0384]: cannot assign twice to immutable variable `a`
  --> src/lib.rs:1:5
   |
 1 |     let a = 123;
   |         - first assignment to `a`
 2 |     a = 321;
   |     ^^^^^^^ cannot assign twice to immutable variable
   |
help: consider making this binding mutable
   |
 1 |     let mut a = 123;
   |         +++
```

---

# Mutable bindings

If you want to mutate something, slap `mut` in front of it!

```rust
let mut a = 123;
a = 321; // No compilation error!
```

---

## Expressions vs statements

Rust is inspired by functional languages so _almost_ everything is an expression:

```rust
1
2 + 3 / 4
false.to_string()
```

Statements always return the unit-type, and always end with a semicolon:

```rust
let a = 3;
3;
return 2 * 4;
```

---

# Functions

Rust has type inference, but requires explicit function signatures

```rust
fn returns_unit() -> () {}
```

`()` is the empty tuple, commonly called the unit-type
Statements and empty functions return it

```rust
fn also_returns_unit() {}
```

The unit-type is the only type that can be omitted as a return-type.

---

# Function returns

Functions can have explicit or implicit returns, but implicit returns is the idiomatic way. Implicit returns always returns the last expression.

```rust
fn math(x: i32) -> i32 {
    let a = x * 2;
    return a * x + a
}
fn idiomatic_math(x: i32) -> i32 {
    let a = x * 2;
    a * x + a
}
```

---

# Function calls

Function calls in Rust are like most languages:

```rust
let a = idiomatic_math(123);
let b = idiomatic_math(321);
let result = max(a, b);

// You also have method call syntax
let result = a.max(b);
```

---

## Modules

<style scoped>
section {
    font-size: 1.9rem;
}
</style>

Curly braces define a new scope, as do modules. Access using `outside::inside`

```rust
mod module {
    pub fn answer() -> i32 {
        let result = {
            let a = 40;
            a + 2
        };
        result
    }
}

fn question() -> i32 {
    // answer() would compile fail because it is not in scope
    module::answer()
}
```

---

# Data types

- Most commonly-used languages (C#, Java, Kotlin, Python, JS, etc.) only have product types
- Rust (and almost _all_ functional languages) also has sum types
- Together these are called ADTs: Algebraic Data Types
- Product types can be thought of as **_and_**-types, e.g. this _and_ that
- Sum types can be thought of as **_or_**-types, e.g. this _or_ that

---

# Product types

<style scoped>
section {
    font-size: 1.9rem;
}
</style>

A class C# is a product-type. It has this _and_ that.

```csharp
class Access {
    public bool Read { get; set; }
    public bool Write { get; set; }
}
```

Four states can be true at any time, technically:

- Read: false, write: false -> no access
- Read: true, write: false -> read-only access
- Read: true, write: true -> read and write access
- Read: false, write: true -> ... write-only access? Makes no sense

---

# Sum types

A Rust-enum or a C#-enum is a sum-type. Is is this either one or the other:

```rust
enum Access {
    ReadOnly,
    ReadAndWrite,
    None
}
```

(Both Rust and C# here have the same syntax)
We no longer have have write-only access! Only three states!

#### Make illegal states unrepresentable

---

# Representing either-or-data in C#

How do you sometimes carry data and other times not?
In class-like languages: nullability 🤢

```csharp
class Address {
    public StreetAddress? Address { get; set; }
    public PostBox? PostBox { get; set; }
}
```

Again, four states, but now only two should be possible!
You can't have an address _and_ a post box. Probably. Idk
It is possible, but very difficult (many lines of code) to work around

---

# Algebraic data types

Combining sums and products produce an _algebra_, thus the name.
Rust doesn't have `null`, it uses ADTs

In Rust an address would be:

```rust
enum Address {
    StreetAddress(StreetAddressData),
    PostBox(PostBoxData),
}
```

Only two states possible! Rust prohibits invalid access

---

# No null? What is the other _option_?

Absence of data is represented using `Option<T>` (generic over the value of type `T`):

```rust
enum Option<T> {
    Some(T),
    None
}
```

Either you have `Some(value)` or you have `None`
Where you would use `string?` before, you now use `Option<String>`

---

# Working with ADTs

How do you extract data? You `match` on what the data is!

```rust
fn send_mail(address: Address, mail: Mail) {
    match address {
        StreetAddress(street_address) => {
            // Use street address data here
        }
        PostBox(post_box) => {
            // Use post box data here
        }
    }
}
```

---

### Pattern matching is powerful

<style scoped>
section {
    font-size: 1.7rem;
}
</style>

```rust
fn adults_only(age: u8) -> bool {
    match age {
        0..18 => false,
        _ => true,
    }
}
```

However Rust also has if-else

```rust
fn adults_only_boring_way(age: u8) -> bool {
    if age < 18 {
        false
    } else {
        true
    }
}
```

Takes one more line, so objectively worse ;)

---

### Pattern matching is _really_ powerful

```rust
let restaurant_lookup_result = Some(Restaurant {
    name: String::from("Roze Gastro"),
    rating: 5,
    address: Address {
        street: String::from("Thereses gate 20"),
        zip_code: 0168,
    },
});
match restaurant_lookup_result {
    Some(Restaurant {
        name,
        rating: 5,
        address:
            Address {
                street,
                zip_code: 0..=0999,
            },
    }) => {
        println!("Try out {name} at {street} in Oslo!");
    }
    _ => { /* Do nothing */ }
}
```

---

# Error-handling

<style scoped>
section {
    font-size: 1.9rem;
}
</style>

Since we can clearly know we have an exclusive either or (XOR) situation, we use ADTs for things that can fail

```rust
enum Result<T, E> {
    Ok(T),
    Err(E)
}
struct NumberParsingError;
fn try_parse_123(number: String) -> Result<i32, NumberParsingError> {
    if number == "123" {
        Ok(123)
    } else {
        Err(NumberParsingError)
    }
}
```
