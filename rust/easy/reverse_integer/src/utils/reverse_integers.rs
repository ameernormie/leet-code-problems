pub fn reverse(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    let mut num = x as i64;
    let mut result: i64 = 0;
    while num <= -10 || num >= 10 {
        let remainder = num % 10;
        num = num / 10;
        result = result * 10 + remainder;
    }
    result = result * 10 + num;
    if result < i32::min_value() as i64 || result > i32::max_value() as i64 {
        return 0;
    }
    return result as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_positive_integer() {
        assert_eq!(reverse(1234), 4321);
    }

    #[test]
    fn reverse_negative_integer() {
        assert_eq!(reverse(-1234), -4321);
    }

    #[test]
    fn reverse_zero() {
        assert_eq!(reverse(0), 0);
    }

    #[test]
    fn reverse_a_number_with_insignificant_zero() {
        assert_eq!(reverse(0123), 321);
    }

    #[test]
    fn reverse_a_out_of_bound_integer() {
        assert_eq!(reverse(2147483647), 0);
    }
}
