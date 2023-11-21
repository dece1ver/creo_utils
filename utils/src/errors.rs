use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FilterError {
    #[error("Ошибка ввода/вывода: {0}")]
    IoError(io::Error),
    #[error("Подходящих файлов не найдено.")]
    NoMatches,
}

impl From<io::Error> for FilterError {
    fn from(error: io::Error) -> Self {
        FilterError::IoError(error)
    }
}
