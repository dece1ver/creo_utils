use clap::{command, Parser};
use config::{Config, FileFormat};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::Display,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub const CONFIG_DIR: &str = "C:\\ProgramData\\dece1ver\\CNC Postout\\";
pub const CONFIG_PATH: &str = "C:\\ProgramData\\dece1ver\\CNC Postout\\config.yml";

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
    pub cimco_path: String,
    pub fallback_program: String,
    pub output_path: String,
    pub support_extensions: Vec<String>,
    pub autoclose: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            cimco_path: "C:\\CIMCO\\CIMCOEdit8\\CIMCOEdit.exe".to_owned(),
            fallback_program: "notepad".to_owned(),
            output_path: format!("Z:\\Creo Settings WNC\\{}_6_0\\NC_OUT", whoami::username()),
            support_extensions: vec![
                "nc".to_string(),
                "eia".to_string(),
                "mpf".to_string(),
                "spf".to_string(),
                "tap".to_string(),
                "pbg".to_string(),
                "h".to_string(),
            ],
            autoclose: Default::default(),
        }
    }
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Параметры:\n\tПуть к Cimco Edit: {}\n\tРезервная программа: {}\n\tПуть к папке вывода УП: {}\n\tОбрабатываемые расширения: {}\n\t",
            self.cimco_path, self.fallback_program, self.output_path, self.support_extensions.iter().map(|num| format!("\"{}\"", num))
            .collect::<Vec<String>>()
            .join(" "))
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
        Ok(s) => s,
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
