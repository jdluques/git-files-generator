use clap::ValueEnum;
use std::fmt::{
    self,
    Display,
    Formatter,
};

#[derive(Debug, Clone, ValueEnum)]
pub enum GitIgnoreType {
    #[value(name = "rust", alias = "Rust")]
    Rust,
    #[value(name = "go", aliases = ["Go", "golang", "Golang", "GoLang"])]
    Go,
    #[value(name = "python", alias = "Python")]
    Python,
    #[value(name = "c++", aliases = ["C++", "cpp"])]
    Cpp,
    #[value(name = "c", alias = "C")]
    C,
    #[value(name = "java", alias = "Java")]
    Java,
    #[value(name = "node", aliases = ["Node", "NodeJS", "Node.js", "nodejs", "node.js",
                                      "javascript", "JavaScript", "Javascript", "js",
                                      "typescript", "TypeScript", "Typescript", "ts"])]
    NodeJS,
    #[value(name = "next", aliases = ["Next", "NextJs", "Next.js", "nextjs", "next.js"])]
    NextJS,
    #[value(name = "angular", aliases = ["Angular"])]
    Angular,
    #[value(name = "nest", aliases = ["Nest", "NestJS", "nestjs"])]
    NestJS,
}

impl Display for GitIgnoreType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            GitIgnoreType::Rust => "Rust",
            GitIgnoreType::Go => "Go",
            GitIgnoreType::Python => "Python",
            GitIgnoreType::Cpp => "C++",
            GitIgnoreType::C => "C",
            GitIgnoreType::Java => "Java",
            GitIgnoreType::NodeJS => "Node",
            GitIgnoreType::NextJS => "Nextjs",
            GitIgnoreType::Angular => "Angular",
            GitIgnoreType::NestJS => "Nestjs",
        };
        write!(f, "{}", s)
    }
}