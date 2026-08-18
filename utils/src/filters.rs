use crate::FilterError;
use std::{
    fs::{self, DirEntry},
    io::{self, ErrorKind},
    path::PathBuf,
    time::UNIX_EPOCH,
};

/// Трейт для фильтрации файлов по расширению.
pub trait FilteredFiles {
    /// Фильтрует файлы по списку расширений.
    ///
    /// Возвращает только файлы (не директории) с одним из указанных расширений.
    /// Расширения сравниваются без учёта регистра.
    fn filtered<T: AsRef<str>>(self, extensions: &[T]) -> Result<Vec<DirEntry>, FilterError>;
}

impl FilteredFiles for &PathBuf {
    fn filtered<T: AsRef<str>>(self, extensions: &[T]) -> Result<Vec<DirEntry>, FilterError> {
        if !self.exists() || !self.is_dir() {
            return Err(FilterError::IoError(io::Error::new(
                ErrorKind::NotFound,
                "Директория не найдена.",
            )));
        }

        if let Ok(entries) = fs::read_dir(self) {
            let filtered_entries = entries
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_file())
                .filter(|file| {
                    if let Some(ext) = file.path().extension() {
                        let ext = ext.to_string_lossy().to_lowercase();
                        return extensions.iter().any(|x| x.as_ref() == ext.as_str());
                    }
                    false
                })
                .collect();
            return Ok(filtered_entries);
        }
        Err(FilterError::NoMatches)
    }
}

impl FilteredFiles for &str {
    fn filtered<T: AsRef<str>>(self, extensions: &[T]) -> Result<Vec<DirEntry>, FilterError> {
        let path_buf: PathBuf = self.into();
        path_buf.filtered(extensions)
    }
}

/// Трейт для получения последнего (самого свежего по времени модификации) элемента.
pub trait Lastest {
    /// Возвращает последний элемент итератора по времени модификации файла.
    ///
    /// Если итератор пуст, возвращает `None`. Элементы без валидных метаданных
    /// считаются имеющими время модификации равное `UNIX_EPOCH`.
    fn lastest(self) -> Option<DirEntry>;
}

impl<I> Lastest for I
where
    I: IntoIterator<Item = DirEntry>,
{
    fn lastest(self) -> Option<DirEntry> {
        if let Some(lastest) = self.into_iter().max_by_key(|entry| {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    return modified;
                }
            }
            UNIX_EPOCH
        }) {
            return Some(lastest);
        }
        None
    }
}
