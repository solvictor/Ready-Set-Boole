pub fn adder(mut a: u32, mut b: u32) -> u32 {
    while b > 0 {
        let carry = (a & b) << 1;
        a ^= b;
        b = carry;
    }
    a
}

pub fn multiplier(mut a: u32, mut b: u32) -> u32 {
    let mut res = 0;
    while b > 0 {
        if b & 1 != 0 {
            res = adder(res, a);
        }
        a <<= 1;
        b >>= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(0, 9), 9);
        assert_eq!(adder(53, 87), 53 + 87);
        assert_eq!(adder(47, 18), 47 + 18);
        assert_eq!(adder(18, 47), 18 + 47);
        assert_eq!(adder(62, 80), 62 + 80);
    }

    #[test]
    fn test_add_overflows() {
        assert_eq!(adder(u32::MAX, 1), 0);
        assert_eq!(adder(u32::MAX, u32::MAX), u32::MAX - 1);
    }

    #[test]
    fn test_mul() {
        assert_eq!(multiplier(0, 0), 0);
        assert_eq!(multiplier(0, 9), 0);
        assert_eq!(multiplier(1, 1), 1);
        assert_eq!(multiplier(53, 87), 53 * 87);
        assert_eq!(multiplier(47, 18), 47 * 18);
        assert_eq!(multiplier(18, 47), 18 * 47);
        assert_eq!(multiplier(62, 80), 62 * 80);
    }

    #[test]
    #[allow(arithmetic_overflow)]
    fn test_mul_overflows() {
        assert_eq!(multiplier(u32::MAX, 1), u32::MAX);
        assert_eq!(multiplier(u32::MAX, u32::MAX), u32::MAX * u32::MAX);
    }
}
