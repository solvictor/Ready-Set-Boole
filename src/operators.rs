pub fn adder(mut a: u32, mut b: u32) -> u32 {
    while b > 0 {
        let carry = (a & b) << 1;
        a ^= b;
        b = carry;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(0, 9), 9);
        assert_eq!(adder(53, 87), 140);
        assert_eq!(adder(47, 18), 65);
        assert_eq!(adder(18, 47), 65);
        assert_eq!(adder(62, 80), 142);
    }

    #[test]
    fn test_add_overflows() {
        assert_eq!(adder(u32::MAX, 1), 0);
        assert_eq!(adder(u32::MAX, u32::MAX), u32::MAX - 1);
    }
}
