use clap::Parser;
use clipboard_win::{raw::set_file_list, Clipboard};
use env_logger::{Builder, Target};
use igs_postexp::{AppConfig, Args};
use log::{debug, info, warn, LevelFilter};
use std::{
    error::Error,
    fs::{self},
    io::{self, Read, Write},
    process::Command,
};
use utils::{self, FilterError, FilteredFiles, Lastest};

const IGS_EXT: &[&str; 1] = &["igs"];
const LOGS_EXT: &[&str; 3] = &["1", "2", "3"];

fn main() -> Result<(), Box<dyn Error>> {
    let config = init()?;
    match config.output_path.filtered(IGS_EXT) {
        Ok(files) => {
            if let Some(file) = files.lastest() {
                info!("Последний файл: {}", file.file_name().to_string_lossy());
                if let Some(file_path) = file.path().to_str() {
                    let file_path: [&str; 1] = [file_path];
                    match Clipboard::new_attempts(10) {
                        Ok(_) => {
                            if set_file_list(&file_path).is_ok() {
                                info!("Успешно добавлен в буфер обмена.");
                            } else {
                                warn!(
                                    "Не удалось добавить файл {} добавлен в буфер обмена.",
                                    file.file_name().to_string_lossy()
                                );
                                debug!("Не удалось получить доступ к буферу обмена.");
                            }
                        }
                        Err(err) => {
                            warn!(
                                "Не удалось добавить файл {} добавлен в буфер обмена.",
                                file.file_name().to_string_lossy()
                            );
                            debug!("{}", err);
                        }
                    }
                }
            } else {
                warn!("Подходящих файлов не найдено.");
            };
        }
        Err(err) => match err {
            FilterError::IoError(io_err) => match io_err.kind() {
                io::ErrorKind::NotFound => warn!("Директория не найдена."),
                _ => {
                    warn!("Не удалось прочитать директорию.");
                    debug!("{io_err}");
                }
            },
            FilterError::NoMatches => warn!("Файлов не найдено."),
        },
    }

    if config.clear_logs {
        config
            .output_path
            .filtered(LOGS_EXT)
            .into_iter()
            .flatten()
            .filter(|f| f.file_name().to_string_lossy().contains("log"))
            .for_each(|f| match fs::remove_file(f.path()) {
                Ok(_) => info!("Очищен лог: {}", f.file_name().to_string_lossy()),
                Err(_) => warn!(
                    "Не удалось удалить файл: {}",
                    f.file_name().to_string_lossy()
                ),
            });
    }
    if !config.autoclose {
        print!("\nДанное окно можно закрыть.");
        let _ = io::stdout().flush();
        let _ = io::stdin().read(&mut [0]);
    }
    Ok(())
}

fn init() -> Result<AppConfig, Box<dyn Error>> {
    let args = Args::parse();
    let level = if args.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    Builder::new()
        .filter(None, level)
        .format_timestamp(None)
        .target(Target::Stdout)
        .init();

    if args.reset {
        debug!("Сброс конфигурации.");
        match fs::remove_file(igs_postexp::CONFIG_PATH) {
            Ok(_) => {
                info!("Cброс конфигурации.")
            }
            Err(e) => {
                warn!("Не удалось сбросить конфигурацию.");
                debug!("{e}");
            }
        }
    }
    let config = igs_postexp::load_config();
    if args.config {
        debug!("Открытие файла конфигурации.");
        match Command::new("notepad")
            .arg(igs_postexp::CONFIG_PATH)
            .spawn()
        {
            Ok(output) => {
                debug!("{output:#?}");
            }
            Err(e) => {
                warn!("Не удалось открыть файл конфигурации.");
                debug!("{e}");
            }
        }
    }
    config
}
