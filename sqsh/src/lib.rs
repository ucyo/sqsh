//! # Squash Compression Library
//!
//! A library for compression software in Rust with focus on scientific data.
//! Currently the library is under private development. It will be released Q4 2022.
//!
//! 🤿

pub mod core;
pub mod processors;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
