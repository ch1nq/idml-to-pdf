extern crate zip_extract;

use std::path::{PathBuf};
use std::fs::{File, remove_file, copy};
use std::env;
use zip_extract::ZipExtractError;

pub fn decompress_idml(idml_path_str:&str) -> Result<PathBuf, ZipExtractError> {
    
    // Duplicate .idml file and change extesnsion to .zip
    let idml_path = PathBuf::from(idml_path_str);
    let mut zip_path = idml_path.clone();
    zip_path.set_extension("zip");
    copy(idml_path, &zip_path)?;

    // Unzip into new directory in tmp 
    let mut target_dir = env::temp_dir();
    target_dir.push("idml_extracted");     
    zip_extract::extract(File::open(&zip_path).unwrap(), &target_dir, true)?;
    
    // Delete .zip 
    remove_file(zip_path)?;

    Ok(target_dir)
}
