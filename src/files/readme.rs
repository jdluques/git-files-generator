use std::{
    env,
    error::Error
};

use crate::{
    files,
    utils,
};

pub fn generate() -> Result<(), Box<dyn Error>> {
    let current_path = env::current_dir()?;

    let dir_name = current_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");

    let file_content = format!("# {}\n", dir_name);

    utils::create_file(
        files::FileType::Readme.to_string().as_str(),
        &file_content
    )?;

    Ok(())
}
