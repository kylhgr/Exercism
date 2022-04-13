# Functions - Rustlan

Functions: bodies of code with one or more statements or expression
* functions may return a value

## Rust functions
Rust uses **snake case** for function names
* prefaced with `fn` keyword

```rust
fn do_nothing() {

}
```

### Rust parameters
* the parentheses surrond ==parameters==, parameters create a binding to a value of a paticular type --> that value is passed into the function
* if there are no parameters the parenthesis are still required
* the value passedd through a parameter is called an **arguement**

#### Function signature
Combining parameters and return values = function signature
* the signature requires that each parameter must have its type annotated

#### Function statements
* the curl braces `{}` enclose the body of the function --> creates a statement or expression

## Example

```rust
//function that takes in on parameter type  `i32`, binds it to the name `value`, and prints it
fn print_int(value: i32) {
    println("{:?}", value);
}

//function parameter `i32` and returns its double

//`-> i32` = function will return an i32

fn double_int(value: i32) -> i32 {
    value * 2
}

//exit from a function early using return keyword

fn long_funct() -> i32 {
    let some_action = false;
    //code snipped
    if (some_action) {
        return 0;
    }
}

//constant functions, evaluated at compile time
const fn compute_data_checksum() -> u128 {
  const DATA: &[u8] = include_bytes!("my_big_data_file");
  // checksum implementation is left as an exercise for the reader
}

/// This checksum is used to validate that the user has not tampered with proprietary configuration.
pub const DATA_CHECKSUM: u128 = compute_data_checksum();

//`const fn` can only call other functions marked with `const`, otherwise will resullt in a compile error
const fn multiply_integer(value: i32) -> i32 {
    use std::time::SystemTime;
    if SystemTime::now().elapsed().unwrap().as_nanos() == 0 { // this line errors
        value * 2
    } else {
        value * 3
    }
}



```
