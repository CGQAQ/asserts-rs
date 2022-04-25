pub mod debug;

///
/// ```rust
/// use asserts_rs::*;
/// asserts_eq!(1, 1); //OK
/// asserts_eq!(1, 1, 1); // OK
/// ```
/// ```should_panic
/// use asserts_rs::*;
/// asserts_eq!(1, 1, 1, 2); // panic 1 not equal to 2
/// ```
///
#[macro_export]
macro_rules! asserts_eq {
    ($expresson:expr, $expected:expr) => {
        assert_eq!($expresson, $expected);
    };

    ($expresson:expr, $expected:expr, $($others:expr),+) => {
        assert_eq!($expresson, $expected);
        asserts_eq!($expresson, $($others),+);
    };
}

///
/// ```rust
/// use asserts_rs::*;
/// asserts_ne!(1, 2); //OK
/// asserts_ne!(1, 2, 3); // OK
/// ```
/// ```should_panic
/// use asserts_rs::*;
/// asserts_ne!(1, 2, 1, 3); // panic 1 equal to 1
/// ```
///
#[macro_export]
macro_rules! asserts_ne {
    ($expresson:expr, $expected:expr) => {
        assert_ne!($expresson, $expected);
    };

    ($expresson:expr, $expected:expr, $($others:expr),+) => {
        assert_ne!($expresson, $expected);
        asserts_ne!($expresson, $($others),+);
    };
}

///
/// ```rust
/// use asserts_rs::*;
/// asserts_eq_one_of!(1, 1); //OK
/// asserts_eq_one_of!(1, 1, 2); // OK
/// ```
/// ```should_panic
/// use asserts_rs::*;
/// asserts_eq_one_of!(1, 2, 3); // panic 1 is not equals to any of numbers in (2, 3)s
/// ```
///
#[macro_export]
macro_rules! asserts_eq_one_of {
    ($expresson:expr, $expected:expr) => {
        assert_eq!($expresson, $expected);
    };

    ($expresson:expr, $expected:expr, $($others:expr),+) => {
        if ($expresson != $expected) {
            asserts_eq_one_of!($expresson, $($others),+);
        } else {
            assert_eq!($expresson, $expected);
        }
    };
}

///
/// ```rust
/// use asserts_rs::*;
/// asserts_ne_one_of!(1, 2); //OK
/// asserts_ne_one_of!(1, 2, 3); // OK
/// ```
/// ```should_panic
/// use asserts_rs::*;
/// asserts_ne_one_of!(1, 1, 1); // panic 1 is equals to all of numbers in (1, 1)
/// ```
///
#[macro_export]
macro_rules! asserts_ne_one_of {
    ($expresson:expr, $expected:expr) => {
        assert_ne!($expresson, $expected);
    };

    ($expresson:expr, $expected:expr, $($others:expr),+) => {
        if ($expresson == $expected) {
            asserts_ne_one_of!($expresson, $($others),+);
        } else {
            assert_ne!($expresson, $expected);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asserts_eq() {
        asserts_eq!(1, 1);
        asserts_eq!(1, 1, 1);
    }

    #[test]
    #[should_panic]
    fn test_asserts_eq_should_fail() {
        asserts_eq!(1, 2);
        asserts_eq!(1, 2, 3);
    }

    #[test]
    fn test_asserts_ne() {
        asserts_ne!(1, 2);
        asserts_ne!(1, 2, 3);
    }

    #[test]
    #[should_panic]
    fn test_asserts_ne_should_fail() {
        asserts_ne!(1, 1);
        asserts_ne!(1, 1, 1);
    }

    #[test]
    fn test_asserts_eq_one_of() {
        asserts_eq_one_of!(1, 1);
        asserts_eq_one_of!(1, 1, 2);
    }

    #[test]
    #[should_panic]
    fn test_asserts_eq_one_of_should_fail() {
        asserts_eq_one_of!(1, 2);
        asserts_eq_one_of!(1, 2, 3);
    }

    #[test]
    fn test_asserts_ne_one_of() {
        asserts_ne_one_of!(1, 2);
        asserts_ne_one_of!(1, 1, 2);
        asserts_ne_one_of!(1, 2, 3);
    }

    #[test]
    #[should_panic]
    fn test_asserts_ne_one_of_should_fail() {
        asserts_ne_one_of!(1, 1);
        asserts_ne_one_of!(1, 1, 1);
    }
}
