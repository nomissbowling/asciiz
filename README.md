asciiz
======

asciiz Rust crate create buffer and copy bytes ends with 0u8


Examples
--------

```rust
use asciiz::u8z::U8zBuf;

let mut m = U8zBuf::from_u8array(b"bytesarray"); // to keep lifetime
let r = unsafe { c_function_requires_asciiz_const(m.as_i8p()) };
let s = unsafe { c_function_requires_asciiz_not_const(m.as_i8p_mut()) };
```

- [ https://crates.io/crates/asciiz ]( https://crates.io/crates/asciiz )


Samples
-------

see also

- [ https://github.com/nomissbowling/oyk ]( https://github.com/nomissbowling/oyk )
- [ https://github.com/nomissbowling/ode-rs-0000 ]( https://github.com/nomissbowling/ode-rs-0000 )


License
-------

MIT License
