//! One adapter per language column: how to write an input value in that
//! language, the program that runs an idiom's expression and prints its
//! result, and how to read that result back as a [`ci_value::Value`].

mod column;
mod decode;
mod literal;
mod program;

pub use column::Column;
pub use decode::{Fault, decode};
pub use literal::literal;
pub use program::program;
