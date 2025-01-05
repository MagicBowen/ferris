#[cfg(test)]
mod unittests {
    use crate::ffi;

    #[test]
    fn test_init_sdk() {
        let chips = ffi::init_sdk().unwrap();
        assert_eq!(chips.len(), 0);
    }
}
