use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum IgnoreType {
    // Languages
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
    #[value(name = "javacript", aliases = ["JavaScript", "js"])]
    Javascript,
    #[value(name = "typescript", aliases = ["TypeScript", "ts"])]
    Typescript,

    // IDE's and Text Editors
    #[value(name = "vscode", aliases = ["vs-code", "VSCode", "VS-Code"])]
    VSCode,
    #[value(name = "jetbrains", aliases = ["JetBrains", "jet-brains"])]
    JetBrains,
}

pub fn generate(ignore_types: &Vec<IgnoreType>) {
    println!("Generate .gitignore file");
    dbg!(ignore_types);
}
