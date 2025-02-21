pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_code() {
        for (n, &expected) in [0, 1, 3, 2, 6, 7, 5, 4, 12].iter().enumerate() {
            // println!("{:08b} {:08b}", n, expected);
            // println!(
            //     "gray_code({}) = {}, should be {}",
            //     n,
            //     gray_code(n as u32),
            //     expected
            // );
            assert_eq!(gray_code(n as u32), expected);
        }
    }
}
