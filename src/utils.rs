use std::{
    fs::File,
    io::{ Write, Result },
    path::Path,
};

pub fn create_file(path: &str, content: &str) -> Result<()> {
    let path = Path::new(path);
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
