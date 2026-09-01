/// Assert that a string contains a substring.
#[macro_export]
macro_rules! assert_contains {
    ($haystack:expr, $needle:expr) => {
        assert!(
            $haystack.contains($needle),
            "Expected '{}' to contain '{}'",
            $haystack,
            $needle
        );
    };
}

/// Assert that a result is an error.
#[macro_export]
macro_rules! assert_error {
    ($result:expr) => {
        assert!($result.is_err(), "Expected error, got Ok({:?})", $result.unwrap());
    };
}

/// Assert that a result is Ok with a specific value.
#[macro_export]
macro_rules! assert_ok_eq {
    ($result:expr, $expected:expr) => {
        assert_eq!($result.unwrap(), $expected);
    };
}
