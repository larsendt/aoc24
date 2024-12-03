use std::fs::read_to_string;
use std::io;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("input not found")]
    InputNotFound(#[from] io::Error),
    #[error("parse error")]
    ParseError(String),
}

pub struct AocData {
    pub input: String,
}

impl AocData {
    pub fn new(filename: &str) -> Result<Self, AocError> {
        Ok(Self {
            input: read_to_string(filename)?,
        })
    }

    pub fn lines(&self) -> Result<impl Iterator<Item = &str>, AocError> {
        Ok(self.input.lines())
    }

    pub fn rows(&self) -> Result<impl Iterator<Item = impl Iterator<Item = &str>>, AocError> {
        Ok(self.lines()?.map(|line| line.split_whitespace()))
    }

    pub fn parsed_rows<T>(&self) -> Result<Vec<Vec<T>>, AocError>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        self.rows()?
            .map(|row| {
                row.map(|col| {
                    col.parse()
                        .map_err(|e: T::Err| AocError::ParseError(e.to_string()))
                })
                .collect()
            })
            .collect()
    }
}
