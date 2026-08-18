use std::io;
use thiserror::Error;

/// Ошибки, возникающие при фильтрации файлов в директории.
#[derive(Error, Debug)]
pub enum FilterError {
    /// Ошибка ввода/вывода при чтении директории.
    #[error("Ошибка ввода/вывода: {0}")]
    IoError(io::Error),

    /// Подходящих файлов не найдено.
    #[error("Подходящих файлов не найдено.")]
    NoMatches,
}

impl From<io::Error> for FilterError {
    fn from(error: io::Error) -> Self {
        FilterError::IoError(error)
    }
}
