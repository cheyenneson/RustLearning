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
Single value, there are 4 native ones - integers, floating-point numbers, Booleans, and characters.

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

### Compound Types
* two primitive: tuples and arrays

#### Tuples
* fixed length
* **Unit**: the name for an empty tuple (used as an empty value or return type and written as `()` -> this is implicitly returned in any function without an expression)
* add `mut` keyword to make individual values mutable

e.g.
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // this is called destructuring => x = 500, y = 6.4, z = 1

    // can also access through .index
    let five_hundred = tup.0; // 500
}
```
#### Arrays
* Every element must have same type
* Fixed length (use a Vector if you want mutable size)
* `let a = [1,2,3,4];`
* allocated on the stack, not heap
* Explicit type declaration: `let a: [i32; 5] = [1, 2, 3, 4, 5];` using `[type, number of elemenets]`
⋅⋅⋅can also put a value here to make a fixed list of all same values: `let a = [3;5]; // same as [3,3,3,3,3]`
* access with indexing: `a[4]`
* memory-safe: rust will throw a runtime error if you access an index that doesn't exist (say if a user inputted a bad index during runtime)


# Functions
* convention to use snake_case for function names
* *fun fact: technically parameters that are concrete are called arguments, but we usually use them interchangably to mean either*
* Rust requires type annotations in the parameter list: `fn addition(a: i32, b: i32)`
* **Statement**: instructions that perform an action and don't return anything (therefore, you cannot do `x = y = 6` to assign the same value to two variables)
* **Expression**: evaluate to a resultant value; do not include ending semicolons (which would make it a statement instead), don't need the `return` keyword
* when returning a value, must add `-> value_type` after the function declaration

# Control Flow
#### Conditional
* `if` conditions must be a bool value (unlike JS)
* if, if else, else, block sometimes called `arms` like in a `match`
* if/else can be used inline: `let number = if a == 6 { 5 } else { 6 }; // types must not be mismatched here`

#### Loops
* `loop`, `while`, and `for`
* uses `break` and `continue` keywords
* can return a value by adding an expression after `break` (could also use `return`, but note this will return out of the current function)

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // technically a ; is not needed as this works like return and returns an expression
        }
    };

    println!("The result is {result}"); // 20
}
```

You can add a `loop label` in order to specify a certain loop to break out of (instead of the innermost). e.g.

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop { // must put a single quote at beginning of name
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}"); // ends with printing 2
}
```

Can use a for/each type loop like so (this is safe bc it will always just look at the elements of an array and nothing more):
```rust
for element in a {
    // do stuff
}
```

For loops can also specify a range of numbers:
```rust
for number in (1..4) { // this is called a Range and is provided by the standard library
    // do stuff
}
```

# Ownership
* foundational goal of rust is to never allow undefined behavior (unsafe, esp when you have direct access to memory) and catch undefined behavior at *compile-time*
* **safety**: absence of undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
* Rust does not allow you to interpret memory as an array of bytes, like C
* Variables live in **frames**, a mapping of variables to values within a single scope, such as a function
* frames are organized into a stack of currently-caleld functions
* after a function returns, Rust deallocates the function's frame
* **heap**: a separate region of memory where data can live indefinitely
⋅⋅⋅not tied to a specific stack frame
···you can put data on the heap by using the `Box` construct (`let a = Box::new([0; 1_000_000]);`)

```rust
// this will duplicate the data and fill up memory faster
fn main() {
    let a = [0; 1_000_000];
    let b = a;
}

// this makes a and b pointers to the same thing, with the pointee being [0,0,0...], which is on the heap
fn main() {
    let a = Box::new([0; 1_000_000]);
    let b = a;
}
```

Rust does not allow manual memory management (allocation/deallocation). When a function is called, Rust allocated a stack frame. When it completes, Rust deallocates the stack frame. 

**Box Deallcoation Principle**: If a variable owns a box, when Rust deallocates the variables frame, then Rust deallocates the box's heap memory.
* When you assign a variable to another pointer, ownership of that section of the heap is transferred from the first to the second. Therefore, Rust doesn't accidentally deallocate the same box twice

Boxes are used by data structures (Vec, String, HashMap, etc) - these don't use the literal Box type at the base, but they use Box-like types

**Moved heap data principle**: if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.
```rust
fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first); // first cannot be used anymore, ownership was given to name, and then full
    println!("{full}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
```
We can work around this using `first.clone()`, which creates a separate copy on the heap (deep copy).

To use a Box as an argument, you can use this syntax: `fn foo(b: Box<i32>) { ... }`

Summary: 
Ownership is primarily a discipline of heap management:
* All heap data must be owned by exactly one variable.
* Rust deallocates heap data once its owner goes out of scope.
* Ownership can be transferred by moves, which happen on assignments and function calls.
* Heap data can only be accessed through its current owner, not a previous owner.


## References and Borrowing
**Reference**: A non-owning pointer.
* Uses the & operator
* Creates a reference to or 'borrows' another pointer
* dereference with *

```rust
fn main() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value,
                            //     so x points to the value 2

    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value

    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;    // so only one dereference is needed to read it
}
```

Rust *implicitly* references and dereferences sometimes:
```rust
let x: Box<i32> = Box::new(-1);
let x_abs1 = i32::abs(*x); // explicit dereference
let x_abs2 = x.abs();      // implicit dereference
assert_eq!(x_abs1, x_abs2); // true

let r: &Box<i32> = &x;
let r_abs1 = i32::abs(**r); // explicit dereference (twice)
let r_abs2 = r.abs();       // implicit dereference (twice)
assert_eq!(r_abs1, r_abs2); // true

let s = String::from("Hello");
let s_len1 = str::len(&s); // explicit reference
let s_len2 = s.len();      // implicit reference
assert_eq!(s_len1, s_len2); // true
```

**Aliasing**: Accessing the same data through different variables
* When combined with *mutation*, this can be dangerous (a var can deallocate/mutate the accessed data, or concurrently mutate accessed data, creating a race condition)
* A variable is *aliased* when another variable references it or something in it (if it's, say, a vector of values)

**Pointer Safety Principle**: Data should never be aliased and mutated at the same time (this can cause undefined behavior)
* e.g. Boxes aren't allowed to be aliased
* This is enforced for *References* using the **Borrow Checker**

**Borrow Checker**
* vars have 3 types of permissions on their data (exists only within the compiler, not at runtime): read, write, own
* RW is default, O is given if `mut` is used
* once a var has been used for the last time, it loses all permissions
* when a var references another, the original loses O/W permissions which are then given to the new until it is no longer being used, when it returns those permissions to the original
* permissions can be given to **places**: includes variables (a), dereferences of places (\*a), array accesses of places (a[0]), fields of places (a.0 or a.field), or any combination of places (\*((\*a)[0].1))
