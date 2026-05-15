# Activity: Terminal Business Card Generator

Build a program that prints a nicely formatted developer business card.

### Requirements

**1. Comments**

Use:
- single-line comments
- multi-line comments

Explain sections of your code.

**2. Basic println!**

Print:
- your name
- role
- favorite language
- learning goals

**3. Debug formatting**

Create a struct:

```rust
struct Developer {
    name: &'static str,
    years_coding: u8,
}
```

Print it using:

```rust
println!("{:?}", dev);
```

You must derive `Debug`.

**4. Display formatting**

Implement `Display` manually for `Person`.

Example output:

```
Alice (3 years experience)
```

This tests:
- `std::fmt`
- formatter traits
- custom formatting

**5. Formatting specifiers**

Use:
- positional arguments
- named arguments
- binary formatting
- width/alignment
- padding

Examples to include:

```rust
println!("{:08}", 42);
println!("{:b}", 15);
println!("{:>10}", "rust");
```

**Stretch Goal**

Create a menu like:

```
==== DEV CARD ====
Name     : Aryan
Role     : Student
Rust XP  : Beginner
Binary ID: 101010
==================
```
