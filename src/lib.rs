pub fn sinus8(data: &str) -> u8 {
    0x00 // Implement here
}

#[cfg(test)]
mod tests {
    use crate::sinus8;

    #[test]
    fn testcase1() {
        let result = sinus8("12345678");
        assert_eq!(result, 0x38)
    }

    #[test]
    fn testcase2() {
        let result = sinus8("123456781234567812345678");
        assert_eq!(result, 0x35)
    }

    #[test]
    fn testcase3() {
        let result = sinus8("12345678ABCDEFGH12345678");
        assert_eq!(result, 0xAA)
    }

    #[test]
    fn testcase4() {
        let result = sinus8("");
        assert_eq!(result, 0x00)
    }

    #[test]
    fn testcase5() {
        let data = (33..127).map(|c| c as u8 as char).collect::<String>();
        let result = sinus8(&data);
        assert_eq!(result, 0x57)
    }
}
