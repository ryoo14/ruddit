use serde::Deserialize;

// settings
#[derive(Deserialize)]
pub struct Setting {
    pub user_agent: String,
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
}

impl Setting {
    pub fn load_toml(setting_file: &str) -> Setting {
        let setting_str: String = std::fs::read_to_string(&setting_file).unwrap();
        let setting = toml::from_str(&setting_str).unwrap();
        setting
    }
}
