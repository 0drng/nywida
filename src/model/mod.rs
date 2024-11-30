use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigFile {
    pub packages: Vec<Package>,
    pub packages_simple: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Package {
    package_name: String,
    enabled: Option<bool>,
    aur: Option<bool>,
    dot_config: Option<DotConfig>,
    before_script_path: Option<String>,
    post_script_path: Option<String>,
}

impl Package {
    pub fn get_package_name(&self) -> String {
        self.package_name.to_owned()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or(true)
    }

    pub fn is_aur(&self) -> bool {
        self.aur.unwrap_or(false)
    }

    pub fn get_dot_config(&self) -> Option<DotConfig> {
        self.dot_config.to_owned()
    }

    pub fn get_before_scripts_path(&self) -> Option<String> {
        self.before_script_path.to_owned()
    }

    pub fn get_post_scripts_path(&self) -> Option<String> {
        self.post_script_path.to_owned()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DotConfig {
    pub src: String,
    pub dest: String,
}
