use futures::future::join_all;
use std::{
    fs::OpenOptions,
    io::Write,
    path::Path,
    error::Error,
};

use crate::{
    files::FileType,
    http_client,
    utils,
};
use super::types::GitIgnoreType;

pub async fn generate(git_ignore_types: &[GitIgnoreType], append: bool) -> Result<(), Box<dyn Error>> {
    let fetches = git_ignore_types.iter().map(|git_ignore_type| {
        let git_ignore_type = git_ignore_type.to_string();
        async move {
            let git_ignore_content = http_client::fetch_template(
                &FileType::GitIgnore,
                &git_ignore_type.as_str()
            ).await.unwrap();

            Ok::<_, Box<dyn Error>>((git_ignore_type, git_ignore_content))
        }
    });

    let results = join_all(fetches).await;

    let mut file_content = if append {
        String::from("\n")
    } else {
        String::from(".env\n\n# JetBrains\n.idea/\n*.iml\n*.iws\nout/\n\n# VS Code\n.vscode/\n.vscode-test/\n.history/\n\n")
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
        let path_str = FileType::GitIgnore.to_string();
        let path = Path::new(path_str.as_str());

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        file.write_all(file_content.as_bytes())?;
    } else {
        utils::create_file(
            FileType::GitIgnore.to_string().as_str(),
            &file_content
        )?;
    }
    
    Ok(())
}
