use clap::ValueEnum;
use futures::future::join_all;
use std::{
    fmt::{
        self,
        Display,
        Formatter,
    },
    fs::OpenOptions,
    io::Write,
    path::Path,
    error::Error,
};

use crate::{
    files,
    http_client,
    utils,
};

#[derive(Debug, Clone, ValueEnum)]
pub enum GitIgnoreType {
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
    // #[value(name = "javacript", aliases = ["JavaScript", "Javascript", "js"])]
    // JavaScript,
    // #[value(name = "typescript", aliases = ["TypeScript", "Typescript", "ts"])]
    // TypeScript,

    // IDE's and Text Editors
    // #[value(name = "vscode", aliases = ["vs-code", "VSCode", "VS-Code"])]
    // VSCode,
    // #[value(name = "jetbrains", aliases = ["JetBrains", "jet-brains"])]
    // JetBrains,
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
            // GitIgnoreType::JavaScript => "javascript",
            // GitIgnoreType::TypeScript => "typescript",
            // GitIgnoreType::VSCode => "vscode",
            // GitIgnoreType::JetBrains => "jetbrains",
        };
        write!(f, "{}", s)
    }
}

pub async fn generate(git_ignore_types: &[GitIgnoreType], append: bool) -> Result<(), Box<dyn Error>> {
    let fetches = git_ignore_types.iter().map(|git_ignore_type| {
        let git_ignore_type = git_ignore_type.to_string();
        async move {
            let git_ignore_content = http_client::fetch_template(
                &crate::files::FileType::GitIgnore,
                &git_ignore_type.as_str()
            ).await.unwrap();

            Ok::<_, Box<dyn Error>>((git_ignore_type, git_ignore_content))
        }
    });

    let results = join_all(fetches).await;

    let mut file_content = if append {
        String::from("\n")
    } else {
        String::from(".env\n.idea/\n.vscode/\n.vscode-test/\n.history/\n\n")
    };

    for result in results {
        let (git_ignore_type, git_ignore_content) = result?;

        let cleaned_git_ignore_content: String = git_ignore_content
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with('#')
            })
            .collect::<Vec<_>>()
            .join("\n");

        file_content += &format!("# {}\n{}\n\n", git_ignore_type, cleaned_git_ignore_content);
    }

    file_content.pop();
    
    if append {
        let path_str = files::FileType::GitIgnore.to_string();
        let path = Path::new(path_str.as_str());

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        file.write_all(file_content.as_bytes())?;
    } else {
        utils::create_file(
            files::FileType::GitIgnore.to_string().as_str(),
            &file_content
        )?;
    }
    
    Ok(())
}
