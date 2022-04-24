# asserts-rs
Assert utillities


## Utility macros
### asserts_eq
```rust
use asserts_rs::*;
asserts_eq!(1, 1); //OK
asserts_eq!(1, 1, 1); // OK
asserts_eq!(1, 1, 1, 2); // panic 1 not equal to 2
```

### asserts_ne
```rust
use asserts_rs::*;
asserts_ne!(1, 2); //OK
asserts_ne!(1, 2, 3); // OK
asserts_ne!(1, 2, 1, 3); // panic 1 equal to 1
```

### asserts_eq_one_of
```rust
use asserts_rs::*;
asserts_eq_one_of!(1, 1); //OK
asserts_eq_one_of!(1, 1, 2); // OK
asserts_eq_one_of!(1, 2, 3); // panic 1 is not equals to any of numbers in (2, 3)s
```

### asserts_ne_one_of
```rust
use asserts_rs::*;
asserts_ne_one_of!(1, 2); //OK
asserts_ne_one_of!(1, 2, 3); // OK
asserts_ne_one_of!(1, 1, 1); // panic 1 is equals to all of numbers in (1, 1)
```
