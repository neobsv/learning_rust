//! # Crates.io Libraries
//!
//! `cratesio_libraries` is a crate which describes how to publish your own crate online
//! to crates.io, and also various ways to document the crate.
//! 


/// Adds one to a given number
/// 
/// # Example:
/// 
/// ```
/// let arg = 5;
/// let answer = cratesio_libraries::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
/// 
pub fn add_one(x: i32) -> i32 {
    x + 1
}