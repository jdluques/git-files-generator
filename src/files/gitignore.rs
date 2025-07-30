pub mod generate;
pub mod types;

pub use types::GitIgnoreType as GitIgnoreType;
pub use generate::generate as generate_gitignore;