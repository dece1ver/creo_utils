use clap::Parser;
use cnc_postout::{AppConfig, Args};
use env_logger::{Builder, Target};
use log::{debug, info, warn, LevelFilter};
use std::{
    error::Error,
    fs::{self},
    io::{self, Read, Write},
    process::Command,
};
use utils::{self, FilterError, FilteredFiles, Lastest};

fn main() -> Result<(), Box<dyn Error>> {
    let config = init()?;

    match config.output_path.filtered(&config.support_extensions) {
        Ok(files) => {
            if let Some(lastest) = files.lastest() {
                info!("Последний файл УП: {:?}", lastest.file_name());
                info!("Открытие УП в Cimco Edit");
                match Command::new(&config.cimco_path).arg(lastest.path()).spawn() {
                    Ok(output) => debug!("{output:#?}"),
                    Err(e) => {
                        warn!("Не удалось открыть УП в Cimco Edit. Открытие резервной программой.");
                        debug!("{e}");
                        match Command::new(&config.fallback_program)
                            .arg(lastest.path())
                            .spawn()
                        {
                            Ok(output) => debug!("{output:#?}"),
                            Err(e) => {
                                warn!("Не удалось запустить резервной программой.");
                                debug!("{e:#?}")
                            }
                        }
                    }
                }
            } else {
                warn!("Подходящих УП не найдено.");
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

    let config = cnc_postout::load_config();

    if args.reset {
        debug!("Сброс конфигурации.");
        match fs::remove_file(cnc_postout::CONFIG_PATH) {
            Ok(_) => {
                info!("Cброс конфигурации.")
            }
            Err(e) => {
                warn!("Не удалось сбросить конфигурацию.");
                debug!("{e}");
            }
        }
    }
    if args.config {
        debug!("Открытие файла конфигурации.");
        match Command::new("notepad")
            .arg(cnc_postout::CONFIG_PATH)
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
