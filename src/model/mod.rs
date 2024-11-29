use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigFile {
    pub packages: Vec<Config>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub package_name: String,
    pub is_aur: bool,
    pub extra_config: Option<String>,
    pub dot_config: Option<DotConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DotConfig {
    pub src: String,
    pub dest: String,
}