use clap::Parser;
use config::{Config, FileFormat};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::Display,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub const CONFIG_DIR: &str = "C:\\ProgramData\\dece1ver\\IGS Postexp\\";
pub const CONFIG_PATH: &str = "C:\\ProgramData\\dece1ver\\IGS Postexp\\config.yml";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Print debug messages
    #[arg(short, long)]
    pub debug: bool,

    /// Reset to default settings
    #[arg(short, long)]
    pub reset: bool,

    /// Open config file
    #[arg(short, long)]
    pub config: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AppConfig {
    pub output_path: String,
    pub clear_logs: bool,
    pub autoclose: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            output_path: format!("Z:\\Creo Settings WNC\\{}_6_0\\IGS", whoami::username()),
            clear_logs: true,
            autoclose: Default::default(),
        }
    }
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Параметры:\n\tПуть к папке вывода IGS: {}\n\tОчистка логов: {}",
            self.output_path,
            if self.clear_logs { "да" } else { "нет" }
        )
    }
}

pub fn load_config() -> Result<AppConfig, Box<dyn Error>> {
    if !Path::new(CONFIG_PATH).exists() {
        debug!("file not exists");
        warn!("Создание конфигурации по умолчанию.");
        let dir = fs::create_dir_all(CONFIG_DIR);
        debug!("create_dir_all -> {dir:#?}");
        if let Ok(mut file) = File::create(CONFIG_PATH) {
            let content = serde_yaml::to_string(&AppConfig::default())?;
            let file = file.write_all(content.as_bytes());
            debug!("created new config file:{file:#?}");
        };
    }
    let config = Config::builder()
        .add_source(config::File::new(CONFIG_PATH, FileFormat::Yaml))
        .build()?;
    let settings: AppConfig = match config.clone().try_deserialize() {
        Ok(s) => {
            info!("Прочитана конфигурация.");
            debug!("{s}");
            s
        }
        Err(e) => {
            warn!("Ошибка при чтении файла конфигурации");
            debug!("{e:#?}");
            if fs::remove_file(CONFIG_PATH).is_ok() {
                load_config()?
            } else {
                return Err(e.into());
            }
        }
    };
    Ok(settings)
}
