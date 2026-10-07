//! The idioms to test, read in place from X_eTaL's Rosetta data
//! (`demos/rosetta/data.toml`): each idiom's input bindings and expected
//! result as values, and the expression X_eTaL's table gives each language.

mod binding;
mod rosetta;

pub use binding::{parse_bindings, parse_literal};
pub use rosetta::{Idiom, load};
