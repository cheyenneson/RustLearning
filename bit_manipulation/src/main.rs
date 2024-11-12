/*
1. **Check if a Number is Power of Two**
   Write a function to determine if a given integer `n` is a power of two. A number is a power of two if it has exactly one `1` bit in its binary representation.

   Example:
   - `n = 16` → `True`
   - `n = 18` → `False`
*/
fn pow_of_two(n: i32) -> bool {
    return n & (n - 1) == 0;
}

/*
2. **Count the Number of 1's (Hamming Weight)**
   Write a function to count the number of `1` bits in the binary representation of an integer `n` (Hamming weight). Try to do it in O(log n) time complexity.

   Example:
   - `n = 29` (binary `11101`) → Output: `4`
   - `n = 15` (binary `1111`) → Output: `4`
*/
fn num_of_ones(n: i32) -> i32 {
    let mut n = n;
    let mut count: i32 = 0;
    while n != 0 {
        n &= n - 1;
        count += 1;
    }
    return count;
}

/*
3. **Reverse the Bits of an Integer**
   Write a function that takes an integer `n` and reverses its binary representation. Assume that the integer is represented using 32 bits.

   Example:
   - `n = 43261596` (binary `00000010100101000001111010011100`) → Output: `964176192` (binary `00111001011110000010100101000000`)
*/
fn reverse_bits(n: i32) -> i32 {
    let mut n = n;
    let mut result: i32 = 0;
    for _ in 0..=31 {
        result <<= 1;
        result |= n & 1;
        n >>= 1;
    }
    return result;
}

/*
4. **Find the Single Non-Duplicate Element**
   Given an array of integers where every element appears twice except for one element that appears only once, write a function to find that element. Do this using **bitwise XOR**.

   Example:
   - `arr = [4, 1, 2, 1, 2]` → Output: `4`
   - `arr = [2, 2, 3, 3, 7, 7, 9]` → Output: `9`
*/
fn find_non_dup(arr: &[i32]) -> i32 {
    let mut result: i32 = 0;
    for val in arr {
        result ^= val;
    }
    return result;
}

/*
5. **Set the k-th Bit**
   Write a function to set the `k`-th bit (0-indexed) of a given integer `n` to `1`.

   Example:
   - `n = 5` (binary `101`), `k = 1` → Output: `7` (binary `111`)
   - `n = 10` (binary `1010`), `k = 2` → Output: `14` (binary `1110`)
*/
fn set_kth_bit(n: i32, k: i32) -> i32 {
    let mask: i32 = 1 << k;
    return n ^ mask;
}

/*
6. **Clear the k-th Bit**
   Write a function to clear the `k`-th bit (0-indexed) of a given integer `n`, setting it to `0`.

   Example:
   - `n = 15` (binary `1111`), `k = 2` → Output: `11` (binary `1011`)
   - `n = 8` (binary `1000`), `k = 3` → Output: `0` (binary `0000`)
*/
fn clear_kth_bit(n: i32, k: i32) -> i32 {
    let mask: i32 = !(1 << k);
    return n & mask;
}

/*
7. **Swap Two Numbers Using Bitwise XOR**
   Write a function to swap two numbers without using a temporary variable, utilizing bitwise XOR.

   Example:
   - `a = 5`, `b = 7` → Output: `a = 7`, `b = 5`
*/
fn swap_two_numbers(mut a: i32, mut b: i32) {
    let a_original = a;
    let b_original = b;

    a = a ^ b; // a is now a ^ b
    b = b ^ a; // b is now b ^ a ^ b = a
    a = a ^ b; // a is now a ^ b ^ a = b

    println!("{}", grade(a == b_original && b == a_original));
}

/*
8. **Find the Two Non-Duplicate Elements**
   Given an array where every element appears twice except for two elements that appear only once, find the two elements that appear only once. Solve it using bit manipulation.

   Example:
   - `arr = [1, 2, 3, 2, 1, 4]` → Output: `[3, 4]`
*/
fn find_non_dups(n: &[i32]) -> &[i32] {
    return &[1, 2, 3, 4];
}

/*
9. **Detect Even or Odd Using Bitwise Operators**
   Write a function to check if a given number `n` is even or odd without using the modulus operator `%`. Use bitwise operators.

   Example:
   - `n = 4` → `Even`
   - `n = 7` → `Odd`
*/
fn even_or_odd(n: i32) -> String {
    if n & 1 == 1 {
        "Odd".to_string()
    } else {
        "Even".to_string()
    }
}

/*
10. **Determine if a Number is Palindrome Using Bits**
   Write a function to check if the binary representation of a number is a palindrome (it reads the same forwards and backwards).

   Example:
   - `n = 9` (binary `1001`) → Output: `True`
   - `n = 10` (binary `1010`) → Output: `False`
*/
fn palindrome(n: i32) -> bool {
    println!("n = {:04b} and reverse = {:04b}", n, reverse_bits(n));
    return reverse_bits(n) == n;
}

fn grade(condition: bool) -> String {
    if condition {
        "Passed :)".to_string()
    } else {
        "FAILED!".to_string()
    }
}

fn main() {
    println!("\n#1");
    println!("{}", grade(pow_of_two(16) == true));
    println!("{}", grade(pow_of_two(18) == false));

    println!("\n#2");
    println!("{}", grade(num_of_ones(29) == 4));
    println!("{}", grade(num_of_ones(15) == 4));

    println!("\n#3");
    println!("{}", grade(reverse_bits(43261596) == 964176192));

    println!("\n#4");
    println!("{}", grade(find_non_dup(&[4, 1, 2, 1, 2]) == 4));
    println!("{}", grade(find_non_dup(&[2, 2, 3, 3, 7, 7, 9]) == 9));

    println!("\n#5");
    println!("{}", grade(set_kth_bit(5, 1) == 7));
    println!("{}", grade(set_kth_bit(10, 2) == 14));

    println!("\n#6");
    println!("{}", grade(clear_kth_bit(15, 2) == 11));
    println!("{}", grade(clear_kth_bit(8, 3) == 0));

    println!("\n#7");
    swap_two_numbers(5, 7);

    println!("\n#9");
    println!("{}", grade(even_or_odd(4) == "Even"));
    println!("{}", grade(even_or_odd(7) == "Odd"));

    println!("\n#10");
    println!("{}", grade(palindrome(9) == true));
    println!("{}", grade(palindrome(10) == false));
}
