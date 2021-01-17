pub fn echo(buf: &[u8]) -> &[u8] {
    return buf;
}

mod tests {
    #[test]
    fn echo() {
        let str = "abc";
        assert_eq!(super::echo(str.as_bytes()), str.as_bytes());
    }
}
