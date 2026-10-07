//! Array values compared across languages: a shape and the items, which are
//! either all numbers or all characters.
//!
//! Every language's result is turned into a [`Value`] and printed in one
//! canonical form, so `1 2 3` from J, `[1 2 3]` from Uiua and `⟨1,2,3⟩` from
//! BQN all read `num [3] 1 2 3`.

mod compare;
mod native;
mod serial;
mod value;

pub use compare::{agree, shift_origin};
pub use native::parse_native;
pub use serial::{parse_number, parse_serialized};
pub use value::{Items, Value};
