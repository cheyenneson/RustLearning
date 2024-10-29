# Data Types

Rust is statically typed, meaning it must know the types of all variables at compile time.

Example of delcaring a type:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```
Wihtout `:u32`, compiler will complain because it doesn't know what type to parse the text into.

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

### Scalar Types
Single value - includes itegers, floating-point numbers, Booleans, and characters.

##### Integer
Numbers can be stored from $-2^{n - 1}$ to $2^{n - 1} - 1$ inclusive. Rust int type defaults to i32. `isize` or `usize` is best for if you're indexing a collection.

| Length        | Signed        | Unsigned  |
| ------------- |:-------------:| ---------:|
| 8-bit         | i8            | u8        |
| 16-bit        | i16           | u16       |
| 32-bit        | i32           | u32       |
| 64-bit        | i64           | u64       |
| 128-bit       | i128          | u128      |
| arch          | isize         | usize     |

arch is dependent on the computer's archtecture (64 or 32 usually)

*Fun note: vars are signed using Two's Complement*

Literals:
| Number Literal| Example
| ------------- |:----------:|
| Decimal       | 98_222     |
| Hex           | 0xff       |
| Octal         | 0o77       |
| Binary        | 0b1111_0000|
| Byte (u8 only)| b'A'       |

Rust will cause your program to `panic` at runtime if an overflow issue occurs (only in debug mode, otherwise will wrap). To error check for overflow, use standard library:
* Wrap in all modes with the wrapping_* methods, such as wrapping_add.
* Return the None value if there is overflow with the checked_* methods.
* Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
* Saturate at the value’s minimum or maximum values with the saturating_* methods.

#### Floating-Point
* `f32` or `f64` (single and double precision)
* always signed
* default is i64

#### Boolean
* one byte size
* `bool` keyword

#### Character
* `char`
* can be special characters like a greek letter or emoji (uses Unicode)
* specify with single quotes
* 4 bytes in size
* range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive