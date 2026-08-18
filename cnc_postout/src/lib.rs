use clap::Parser;
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

/// Путь к директории конфигурации CNC Postout.
pub const CONFIG_DIR: &str = "C:\\ProgramData\\dece1ver\\CNC Postout\\";
/// Путь к файлу конфигурации CNC Postout.
pub const CONFIG_PATH: &str = "C:\\ProgramData\\dece1ver\\CNC Postout\\config.yml";

/// Аргументы командной строки.
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

/// Конфигурация приложения CNC Postout.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AppConfig {
    /// Использовать программу по умолчанию (ассоциацию системы) для открытия УП.
    pub use_default_program: bool,
    /// Список программ для попытки открытия УП (при `use_default_program == false`).
    pub fallback_programs: Vec<String>,
    /// Путь к папке вывода NC-файлов.
    pub output_path: String,
    /// Поддерживаемые расширения файлов УП.
    pub support_extensions: Vec<String>,
    /// Автозакрытие окна после выполнения.
    pub autoclose: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            use_default_program: true,
            fallback_programs: vec!["notepad".to_string()],
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
        let program_info = if self.use_default_program {
            "по умолчанию".to_string()
        } else {
            self.fallback_programs
                .iter()
                .map(|p| format!("\"{}\"", p))
                .collect::<Vec<_>>()
                .join(", ")
        };
        write!(
            f,
            "Параметры:\n\tПрограмма открытия: {}\n\tПуть к папке вывода УП: {}\n\tОбрабатываемые расширения: {}\n\t",
            program_info,
            self.output_path,
            self.support_extensions.iter().map(|num| format!("\"{}\"", num))
            .collect::<Vec<String>>()
            .join(" "))
    }
}

/// Загружает конфигурацию из YAML-файла.
///
/// Если файл конфигурации не существует, создаёт его с настройками по умолчанию.
/// При ошибке чтения удаляет повреждённый файл и создаёт заново.
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
