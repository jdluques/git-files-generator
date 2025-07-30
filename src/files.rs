pub mod env_example;
pub mod gitignore;
pub mod license;
pub mod readme;

pub use env_example::generate as generate_env_example;
pub use gitignore::generate_gitignore as generate_gitignore;
pub use license::generate as generate_license;
pub use readme::generate as generate_readme;

use std::fmt::{
    self,
    Display,
    Formatter
};

pub enum FileType {
    EnvExample,
    GitIgnore,
    License,
    Readme,
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            FileType::EnvExample => ".env.example",
            FileType::GitIgnore => ".gitignore",
            FileType::License => "LICENSE",
            FileType::Readme => "README.md",
        };
        write!(f, "{}", s)
    }
}
