use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigFileLock {
    pub packages_installed: Vec<String>,
    pub packages_removed: Vec<String>,
    pub pre_scripts_runned: Vec<Script>,
    pub post_scripts_runned: Vec<Script>
}

impl ConfigFileLock {
    pub fn empty() -> Self {
        ConfigFileLock { packages_installed: Vec::new(), packages_removed: Vec::new(), pre_scripts_runned: Vec::new(), post_scripts_runned: Vec::new() }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigFile {
    pub packages: Vec<Package>,
    pub packages_simple: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Script {
    pub bin: String
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    package_name: String,
    enable: Option<bool>,
    aur: Option<bool>,
    config: Option<DotConfig>,
    pre_script: Option<Vec<Script>>,
    post_script: Option<Vec<Script>>,
}

impl Package {
    pub fn get_package_name(&self) -> String {
        self.package_name.to_owned()
    }

    pub fn is_enabled(&self) -> bool {
        self.enable.unwrap_or(true)
    }

    pub fn is_aur(&self) -> bool {
        self.aur.unwrap_or(false)
    }

    pub fn get_dot_config(&self) -> Option<DotConfig> {
        self.config.to_owned()
    }

    pub fn get_pre_scripts(&self) -> Option<Vec<Script>> {
        self.pre_script.to_owned()
    }

    pub fn get_post_scripts(&self) -> Option<Vec<Script>> {
        self.post_script.to_owned()
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DotConfig {
    pub content: Option<String>,
    pub src: Option<String>,
    pub dest: String,
}
