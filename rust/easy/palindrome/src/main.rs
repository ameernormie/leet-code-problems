mod utils;

use crate::utils::palindrome;

fn main() {
    let num1 = 121;
    println!(
        "The number {} is a palindrome: {}",
        num1,
        palindrome::is_palindrome(num1)
    );

    let num2 = 1233;
    println!(
        "The number {} is a palindrome: {}",
        num2,
        palindrome::is_palindrome(num2)
    );
}
