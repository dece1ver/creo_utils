mod errors;
mod filters;
mod tests;

/// Типы ошибок, используемые при фильтрации файлов.
pub use errors::FilterError;
/// Трейты для фильтрации файлов и получения последнего элемента.
pub use filters::{FilteredFiles, Lastest};
