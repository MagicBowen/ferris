#[cfg(test)]
mod unittests {
    use crate::ffi::*;

    #[test]
    fn test_init_sdk() {
        let device = Device;
        let chips = device.init().unwrap();
        assert_eq!(chips.len(), 0);
    }
}
