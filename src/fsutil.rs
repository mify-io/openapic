use std::io;
use std::io::prelude::*;
use std::fs::File;

use anyhow::Result;

pub fn read_input(file_path: Option<String>) -> Result<String> {
    use std::path::Path;

    let mut content = String::new();

    match file_path {
        Some(path) => {
            let path = Path::new(&path);
            File::open(&path)?.read_to_string(&mut content)
        }
        None => io::stdin().read_to_string(&mut content)
    }?;

    Ok(content)
}

pub fn write_string_to_file(file_path: &str, content: &str) -> Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
