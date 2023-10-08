use std::io::Write;
use std::{fs::OpenOptions, path::PathBuf};

use crate::model::{Error, Model};

fn get_storage_path() -> PathBuf {
    directories::BaseDirs::new()
        .data_dir()
        .join("todotui-data.json")
}

pub fn read_model() -> Result<Model, Error> {
    let path = get_storage_path();
    if path.exists() {
        let json = std::fs::read_to_string(path).map_err(|_| Error::CannotReadDataFile)?;
        let model = serde_json::from_str::<Model>(&json).map_err(|_| Error::InvalidDataFile)?;

        Ok(model)
    } else {
        Ok(Model::default())
    }
}

pub fn write_model(model: &Model) -> Result<(), Error> {
    let json = serde_json::to_string_pretty(model).map_err(|_| Error::SerializationError)?;
    let path = get_storage_path();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .map_err(|_| Error::CannotWriteOpenDataFile)?;
    write!(file, "{}", json).map_err(|_| Error::CannotWriteDataFile)?;

    Ok(())
}
