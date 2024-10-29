# Data Types

Rust is statically typed, meaning it must know the types of all variables at compile time.
Example of delcaring a type:
```let guess: u32 = "42".parse().expect("Not a number!");```

### const vs let 
let can only be used in the context of a function, const can be used in a global context

### shadowing
Shadowing declares a value with the same name as a previous value. The second value "shadows" the first.

Note: If you use `mut` and try to shadow with a new value type, you'll have compiler error.

example:
```rust
fn main() {
    // doesn't need to be mut since we're using 'let' in the shadow instance
    let x = 5;

    let x = x + 1;

    {
        // we're effectively creating a new var but reusing the name
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // prints 12
    }

    println!("The value of x is: {x}"); // prints 6
}
```
    


    