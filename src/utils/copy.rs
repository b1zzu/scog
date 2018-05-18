use std::fs;
use utils::error::Error;
use std::path::Path;
use std::path::PathBuf;

pub fn copy(source: &Path, destination: &Path) -> Result<Vec<PathBuf>, Error> {
    if source.exists() {
        if source.is_file() {
            copy_file(source, destination)
        } else if source.is_dir() {
            copy_dir(source, destination)
        } else {
            Err(format!("source '{}' is neither a file nor dir", source.to_string_lossy()))?
        }
    } else {
        Ok(vec![])
    }
}

fn copy_dir(source: &Path, destination: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut copied: Vec<PathBuf> = vec![];
    for _source in fs::read_dir(&source)? {
        let _source = _source?.path();
        let _destination = destination.join(_source.strip_prefix(&source)?);

        // Recall copy to handle dir or file
        let mut _copied = copy(_source.as_path(), _destination.as_path())?;

        copied.append(&mut _copied);
    }
    Ok(copied)
}

fn copy_file(source: &Path, destination: &Path) -> Result<Vec<PathBuf>, Error> {
    if destination.is_dir() {
        Err(format!("source '{}' is a file but destination '{}' is a dir", source.to_string_lossy(), destination.to_string_lossy()))?
    } else {
        // Create the destination dir if it does not exists
        match destination.parent() {
            Some(parent) => {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            None => {}
        }

        // Copy the file from source to destination
        fs::copy(&source, &destination).unwrap();

        // Call each for each copied files
        Ok(vec![destination.to_path_buf()])
    }
}
