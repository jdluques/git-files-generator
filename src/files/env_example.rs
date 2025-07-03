use std::{
    fs::File,
    io::{
        BufRead, BufReader, Error, ErrorKind, Result
    },
    path::Path
};

use crate::{
    files,
    utils,
};

pub fn generate() -> Result<()> {
    let env_path = Path::new(".env");
    let env_file = File::open(env_path).map_err(|e| {
        if e.kind() == ErrorKind::NotFound {
            Error::new(ErrorKind::NotFound, "Missing .env file")
        } else {
            e
        }
    })?;
    let reader = BufReader::new(env_file);

    let mut lines = Vec::new();

    for line_env in reader.lines() {
        let line = line_env?;
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() || trimmed_line.starts_with("#") {
            lines.push(line);
        } else if let Some((key, _)) = trimmed_line.split_once("=") {
            lines.push(format!("{}=", key.trim()));
        }
    }

    let file_content = lines.join("\n") + "\n";
    
    utils::create_file(
        files::FileType::EnvExample.to_string().as_str(),
        &file_content
    )
}
