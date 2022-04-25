# asserts-rs
Assert utillities


## Utility macros
### asserts_eq/assertsd_eq
```rust
use asserts_rs::*;
asserts_eq!(1, 1); //OK
asserts_eq!(1, 1, 1); // OK
asserts_eq!(1, 1, 1, 2); // panic 1 not equal to 2
assertsd_eq!(1, 1, 1, 2); // panic 1 not equal to 2(only in debug mode)
```

### asserts_ne/assertsd_ne
```rust
use asserts_rs::*;
asserts_ne!(1, 2); //OK
asserts_ne!(1, 2, 3); // OK
asserts_ne!(1, 2, 1, 3); // panic 1 equal to 1
assertsd_ne!(1, 2, 1, 3); // panic 1 equal to 1(only in debug mode)
```

### asserts_eq_one_of/assertsd_eq_one_of
```rust
use asserts_rs::*;
asserts_eq_one_of!(1, 1); //OK
asserts_eq_one_of!(1, 1, 2); // OK
asserts_eq_one_of!(1, 2, 3); // panic 1 is not equals to any of numbers in (2, 3)s
assertsd_eq_one_of!(1, 2, 3); // panic 1 is not equals to any of numbers in (2, 3)s(only in debug mode)
```

### asserts_ne_one_of/assertsd_ne_one_of
```rust
use asserts_rs::*;
asserts_ne_one_of!(1, 2); //OK
asserts_ne_one_of!(1, 2, 3); // OK
asserts_ne_one_of!(1, 1, 1); // panic 1 is equals to all of numbers in (1, 1)
assertsd_ne_one_of!(1, 1, 1); // panic 1 is equals to all of numbers in (1, 1)(only in debug mode)
```
