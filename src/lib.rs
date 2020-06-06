//! Small wrapper around sentencepiece's esaxx suffix array C++ library.
//! Usage
//!
//! ```rust
//! let string = "abracadabra".to_string();
//! let chars: Vec<_> = string.chars().collect();
//! let n = chars.len();
//! let mut sa = vec![0; n];
//! let mut l = vec![0; n];
//! let mut r = vec![0; n];
//! let mut d = vec![0; n];
//! let mut node_num = 0;
//!
//! let alphabet_size = 0x110000; // All UCS4 range.
//! unsafe {
//!     esaxx_int32(
//!         chars.as_ptr() as *mut u32,
//!         sa.as_mut_ptr(),
//!         l.as_mut_ptr(),
//!         r.as_mut_ptr(),
//!         d.as_mut_ptr(),
//!         n.try_into().unwrap(),
//!         alphabet_size,
//!         &mut node_num,
//!     );
//! }
//! ```
extern "C" {
    pub fn esaxx_int32(
        // This is char32
        T: *const u32,
        SA: *mut i32,
        L: *mut i32,
        R: *mut i32,
        D: *mut i32,
        n: u32,
        k: u32,
        nodeNum: &mut u32,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_esaxx() {
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
        assert_eq!(node_num, 5);
        assert_eq!(sa, vec![10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);
        assert_eq!(l, vec![1, 0, 5, 9, 0, 0, 3, 0, 0, 0, 2]);
        assert_eq!(r, vec![3, 5, 7, 11, 11, 1, 0, 1, 0, 0, 0]);
        assert_eq!(d, vec![4, 1, 3, 2, 0, 0, 0, 0, 0, 0, 0]);
    }
}
