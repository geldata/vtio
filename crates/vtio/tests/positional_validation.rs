//! Compile-time validation tests for positional parameters.
//!
//! This file contains tests that should fail to compile if the positional
//! parameter validation is working correctly.

// This test verifies that required positionals cannot come after optional ones.
// Uncomment to test the validation (it should fail to compile):
/*
use vtio_control_derive::VTControl;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
#[vtctl(osc, number = "999", data = "TEST")]
pub struct InvalidPositionalOrder {
    #[vtctl(positional)]
    pub optional_param: Option<i32>,
    #[vtctl(positional)]
    pub required_param: i32,  // This should trigger a compile error
}
*/

// This test verifies that multiple optional positionals work correctly
#[cfg(test)]
mod valid_cases {
    use vtio_control_base::AnsiEncode;
    use vtio_control_derive::VTControl;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
    #[vtctl(osc, number = "999", data = "MULTI")]
    pub struct MultipleOptionalPositionals {
        #[vtctl(positional)]
        pub first: i32,
        #[vtctl(positional)]
        pub second: Option<i32>,
        #[vtctl(positional)]
        pub third: Option<i32>,
    }

    #[test]
    fn test_multiple_optional_positionals_all_present() {
        let mut cmd = MultipleOptionalPositionals {
            first: 1,
            second: Some(2),
            third: Some(3),
        };
        let mut buf = Vec::new();
        let result = cmd.encode_ansi_into(&mut buf);
        assert!(result.is_ok());
        assert_eq!(
            String::from_utf8(buf).unwrap(),
            "\x1b]999;MULTI;1;2;3\x1b\\"
        );
    }

    #[test]
    fn test_multiple_optional_positionals_partial() {
        let mut cmd = MultipleOptionalPositionals {
            first: 1,
            second: Some(2),
            third: None,
        };
        let mut buf = Vec::new();
        let result = cmd.encode_ansi_into(&mut buf);
        assert!(result.is_ok());
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b]999;MULTI;1;2\x1b\\");
    }

    #[test]
    fn test_multiple_optional_positionals_only_required() {
        let mut cmd = MultipleOptionalPositionals {
            first: 1,
            second: None,
            third: None,
        };
        let mut buf = Vec::new();
        let result = cmd.encode_ansi_into(&mut buf);
        assert!(result.is_ok());
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b]999;MULTI;1\x1b\\");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VTControl)]
    #[vtctl(osc, number = "888", data = "ALL_OPT")]
    pub struct AllOptionalPositionals {
        #[vtctl(positional)]
        pub first: Option<i32>,
        #[vtctl(positional)]
        pub second: Option<i32>,
    }

    #[test]
    fn test_all_optional_positionals() {
        let mut cmd = AllOptionalPositionals {
            first: None,
            second: None,
        };
        let mut buf = Vec::new();
        let result = cmd.encode_ansi_into(&mut buf);
        assert!(result.is_ok());
        assert_eq!(String::from_utf8(buf).unwrap(), "\x1b]888;ALL_OPT\x1b\\");
    }
}
