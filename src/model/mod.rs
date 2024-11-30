use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigFile {
    pub packages: Vec<Package>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Package {
    pub package_name: String,
    pub enabled: Option<bool>,
    pub is_aur: bool,
    pub dot_config: Option<DotConfig>,
    pub before_script_path: Option<String>,
    pub post_script_path: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DotConfig {
    pub src: String,
    pub dest: String,
}
