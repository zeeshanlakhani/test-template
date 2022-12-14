#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![deny(unreachable_pub, private_in_public)]

//! test-template

/// Add two integers together.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two integers together.
pub fn mult(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mult() {
        assert_eq!(mult(3, 2), 6);
    }
}
