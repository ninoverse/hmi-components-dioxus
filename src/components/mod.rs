//! Component module. Re-export each component here so call sites can write
//! `use components::Name`. Keep the `pub use` lines alphabetical.

mod hero;

pub use hero::Hero;
