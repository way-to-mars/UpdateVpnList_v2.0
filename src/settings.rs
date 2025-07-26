use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub(crate) enum ProtoTypes {
    UDP,
    TCP,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub(crate) struct AppSettings {
    pub(crate) url: String,
    pub(crate) proto_types: Vec<ProtoTypes>,
}

impl AppSettings {
    fn default_settings() -> AppSettings {
        AppSettings {
            url: "https://vpnobratno.info/russia_server_list.html".to_string(),
            proto_types: [ProtoTypes::UDP].to_vec(),
        }
    }

    fn null_settings() -> AppSettings {
        AppSettings {
            url: "".to_string(),
            proto_types: [].to_vec(),
        }
    }

    fn example_settings() -> AppSettings {
        AppSettings {
            url: "https://domain.app.html".to_string(),
            proto_types: [ProtoTypes::UDP, ProtoTypes::TCP].to_vec(),
        }
    }

    pub(crate) fn to_string(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
}

pub(crate) fn load_setting() -> (AppSettings, String) {
    const FILE_NAME: &str = "settings.yaml";
    let file_exists = fs::exists(&FILE_NAME).unwrap_or(false);

    if file_exists {
        match fs::read_to_string(FILE_NAME) {
            Ok(data) => {
                let settings = serde_yaml::from_str(&data).unwrap_or(AppSettings::null_settings());
                if settings == AppSettings::null_settings() {
                    (
                        AppSettings::default_settings(),
                        "Ошибка в формате файла".to_string(),
                    )
                } else {
                    (
                        settings,
                        format!("Настройки успешно загружены из {FILE_NAME}"),
                    )
                }
            }
            Err(_) => (
                AppSettings::default_settings(),
                format!("Ошибка чтения файла {FILE_NAME}"),
            ),
        }
    } else {
        if create_settings_file(FILE_NAME, AppSettings::default_settings()) {
            (
                AppSettings::default_settings(),
                format!("Создан новый файл настроек {FILE_NAME}"),
            )
        } else {
            (
                AppSettings::default_settings(),
                "Не удалось создать файл настроек".to_string(),
            )
        }
    }
}

fn create_settings_file(file_name: &str, setting: AppSettings) -> bool {
    let yaml = serde_yaml::to_string(&setting).unwrap();
    match fs::write(&file_name, yaml) {
        Ok(_) => true,
        Err(_) => false,
    }
}
