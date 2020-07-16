pub fn is_palindrome(x: i32) -> bool {
    if x == 0 {
        return true;
    }
    if x < 0 {
        return false;
    }
    let mut num = x as i64;
    let mut result: i64 = 0;
    while num >= 10 {
        let remainder = num % 10;
        num = num / 10;
        result = result * 10 + remainder;
    }
    result = result * 10 + num;
    if result == x as i64 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_two_one_is_palindrome() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    fn zero_is_palindrome() {
        assert_eq!(is_palindrome(0), true);
    }

    #[test]
    fn negative_number_is_not_palindrome() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    fn one_two_is_not_palindrome() {
        assert_eq!(is_palindrome(12), false);
    }
}
