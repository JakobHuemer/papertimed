use std::path::PathBuf;

use thiserror::Error;

const BACKUP_FILE_EXTENSION: &'static str = "pptmd-bkp";

use crate::adapter::{
    custom::{CustomAdapter, CustomAdpaterError},
    wpaperd::{WpaperdAdapter, WpaperdError},
};

pub mod custom;
pub mod wpaperd;

pub trait WallpaperAdapter: Default {
    type Input;

    async fn update(&mut self, input: Self::Input) -> Result<(), AdapterError>;
}

#[derive(Clone, Debug)]
pub enum AdapterDispatcher {
    Wpaperd(WpaperdAdapter),
    Custom(CustomAdapter),
}

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("wpaperd: {0}")]
    Wpaperd(#[from] WpaperdError),

    #[error("custom: {0}")]
    Custom(#[from] CustomAdpaterError),

    #[error("background utility '{0}' not installed")]
    UtilityNotInstalled(String),

    #[error("{utility} failed with ({status_code}): {error_out}")]
    UtilityFailedWith {
        utility: String,
        status_code: i32,
        error_out: String,
    },

    #[error("write_file_save: {0}")]
    WriteFile(#[from] WriteFileSaveError),
}

#[derive(Debug, Clone, Error)]
pub enum WriteFileSaveError {
    #[error("backup file already exists")]
    BackupFileExist,
    #[error("could not write backup file")]
    CouldNotWriteBackupFile,
    #[error("could not write file")]
    CouldNotWriteFile,
    #[error("could not create directories")]
    CouldNotCreateDirectories,
}

/// Takes a filepath and some content to write to a file. If the target file
/// already exist, copy the original file with a `.pptmd-bkp` extension.
fn write_file_save(path: &PathBuf, content: String) -> Result<(), WriteFileSaveError> {
    use std::fs;

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|_| WriteFileSaveError::CouldNotCreateDirectories)?;
        }
    }

    if path.exists() {
        let mut backup_path = path.clone();
        backup_path.set_extension(BACKUP_FILE_EXTENSION);
        if backup_path.exists() {
            return Err(WriteFileSaveError::BackupFileExist);
        }
        fs::copy(&path, &backup_path).map_err(|_| WriteFileSaveError::CouldNotWriteBackupFile)?;
    }
    fs::write(&path, content).map_err(|_| WriteFileSaveError::CouldNotWriteFile)?;
    Ok(())
}
