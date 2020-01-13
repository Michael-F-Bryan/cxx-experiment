#[cfg(test)]
mod tests {
    use super::*;
    use std::{ffi::CStr, os::raw::c_char};

    extern "C" {
        fn BZ2_bzlibVersion() -> *const c_char;
    }

    #[test]
    fn smoke_test() {
        unsafe {
            let got = BZ2_bzlibVersion();

            assert!(!got.is_null());
            let version_number = CStr::from_ptr(got).to_str().unwrap();
            assert_eq!(version_number, "1.0.8, 13-Jul-2019");
        }
    }
}
