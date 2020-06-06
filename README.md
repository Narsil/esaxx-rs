# esaxx-rs

Small wrapper around sentencepiece's esaxx suffix array C++ library.
Usage

```rust
let string = "abracadabra".to_string();
let chars: Vec<_> = string.chars().collect();
let n = chars.len();
let mut sa = vec![0; n];
let mut l = vec![0; n];
let mut r = vec![0; n];
let mut d = vec![0; n];
let mut node_num = 0;

let alphabet_size = 0x110000; // All UCS4 range.
unsafe {
    esaxx_int32(
        chars.as_ptr() as *mut u32,
        sa.as_mut_ptr(),
        l.as_mut_ptr(),
        r.as_mut_ptr(),
        d.as_mut_ptr(),
        n.try_into().unwrap(),
        alphabet_size,
        &mut node_num,
    );
}
```

Current version: 0.1.0

License: Apache
