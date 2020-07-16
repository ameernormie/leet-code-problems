mod utils;

use crate::utils::reverse_integers;

fn main() {
    let num1 = 1234;

    println!("Reverse of {} is {}", num1, reverse_integers::reverse(num1));
}
