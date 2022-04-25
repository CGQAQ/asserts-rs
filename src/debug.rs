///
/// ```rust
/// use asserts_rs::*;
/// assertsd_eq!(1, 1); //OK
/// assertsd_eq!(1, 1, 1); // OK
/// ```
/// ```should_panic
/// use asserts_rs::*;
/// assertsd_eq!(1, 1, 1, 2); // panic 1 not equal to 2
/// ```
///
#[macro_export]
macro_rules! assertsd_eq {
    ($expresson:expr, $expected:expr) => {
        debug_assert_eq!($expresson, $expected);
        assert_eq!($expresson, $expected);
    };

    ($expresson:expr, $expected:expr, $($others:expr),+) => {
        debug_assert_eq!($expresson, $expected);
        assertsd_eq!($expresson, $($others),+);
    };
}

///
/// ```rust
/// use asserts_rs::*;
/// assertsd_ne!(1, 2); //OK
/// assertsd_ne!(1, 2, 3); // OK
/// ```
/// ```should_panic
/// use asserts_rs::*;
/// assertsd_ne!(1, 2, 1, 3); // panic 1 equal to 1
/// ```
///
#[macro_export]
macro_rules! assertsd_ne {
    ($expresson:expr, $expected:expr) => {
        debug_assert_ne!($expresson, $expected);
    };

    ($expresson:expr, $expected:expr, $($others:expr),+) => {
        debug_assert_ne!($expresson, $expected);
        assertsd_ne!($expresson, $($others),+);
    };
}

///
/// ```rust
/// use asserts_rs::*;
/// assertsd_eq_one_of!(1, 1); //OK
/// assertsd_eq_one_of!(1, 1, 2); // OK
/// ```
/// ```should_panic
/// use asserts_rs::*;
/// assertsd_eq_one_of!(1, 2, 3); // panic 1 is not equals to any of numbers in (2, 3)s
/// ```
///
#[macro_export]
macro_rules! assertsd_eq_one_of {
    ($expresson:expr, $expected:expr) => {
        debug_assert_eq!($expresson, $expected);
    };

    ($expresson:expr, $expected:expr, $($others:expr),+) => {
        if ($expresson != $expected) {
            assertsd_eq_one_of!($expresson, $($others),+);
        } else {
            debug_assert_eq!($expresson, $expected);
        }
    };
}

///
/// ```rust
/// use asserts_rs::*;
/// assertsd_ne_one_of!(1, 2); //OK
/// assertsd_ne_one_of!(1, 2, 3); // OK
/// ```
/// ```should_panic
/// use asserts_rs::*;
/// assertsd_ne_one_of!(1, 1, 1); // panic 1 is equals to all of numbers in (1, 1)
/// ```
///
#[macro_export]
macro_rules! assertsd_ne_one_of {
    ($expresson:expr, $expected:expr) => {
        debug_assert_ne!($expresson, $expected);
    };

    ($expresson:expr, $expected:expr, $($others:expr),+) => {
        if ($expresson == $expected) {
            assertsd_ne_one_of!($expresson, $($others),+);
        } else {
            debug_assert_ne!($expresson, $expected);
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_assertsd_eq() {
        assertsd_eq!(1, 1);
        assertsd_eq!(1, 1, 1);
    }

    #[test]
    #[should_panic]
    fn test_assertsd_eq_should_fail() {
        assertsd_eq!(1, 2);
        assertsd_eq!(1, 2, 3);
    }

    #[test]
    fn test_assertsd_ne() {
        assertsd_ne!(1, 2);
        assertsd_ne!(1, 2, 3);
    }

    #[test]
    #[should_panic]
    fn test_assertsd_ne_should_fail() {
        assertsd_ne!(1, 1);
        assertsd_ne!(1, 1, 1);
    }

    #[test]
    fn test_assertsd_eq_one_of() {
        assertsd_eq_one_of!(1, 1);
        assertsd_eq_one_of!(1, 1, 2);
    }

    #[test]
    #[should_panic]
    fn test_assertsd_eq_one_of_should_fail() {
        assertsd_eq_one_of!(1, 2);
        assertsd_eq_one_of!(1, 2, 3);
    }

    #[test]
    fn test_assertsd_ne_one_of() {
        assertsd_ne_one_of!(1, 2);
        assertsd_ne_one_of!(1, 1, 2);
        assertsd_ne_one_of!(1, 2, 3);
    }

    #[test]
    #[should_panic]
    fn test_assertsd_ne_one_of_should_fail() {
        assertsd_ne_one_of!(1, 1);
        assertsd_ne_one_of!(1, 1, 1);
    }
}
