use crate::prelude::{*, Error};
use serde_json::to_writer_pretty;
use std::fs::File;


pub fn write_in_output<T: serde::Serialize>(filepath: Option<String>, data: &T) -> Result<()> {
    if let Some(path) = filepath {
        let file = File::create(&path)?;
        to_writer_pretty(file, data)?;
        println!("{:?}", path);
        Ok(())
    } else {
        return Err(Error::PathError(
            "No filepath provided".to_string(),
        ));
    }
}

pub fn write_error_output<T: serde::Serialize>(filepath: Option<String>, error: &T) -> Result<()> {
    if let Some(path) = filepath {
        let file = File::create(&path)?;
        to_writer_pretty(file, &error)?;
        println!("{:?}", path);
        Ok(())
    } else {
        return Err(Error::PathError(
            "No filepath provided".to_string(),
        ));
    }
}