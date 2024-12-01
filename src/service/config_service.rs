use std::{fs::File, io::Read};

use serde_jsonc::json;

use crate::{
    error::PackageManagerError,
    model::{ConfigFile, ConfigFileLock, Package, Script},
    service::translation_service::{t, Labels},
    wrapper::{
        command,
        package_manager::{PackageManager, PackageManagerEnum},
    },
};

pub fn get_packages_str(
    config_file: &ConfigFile,
    aur: Option<bool>,
    enabled: Option<bool>,
) -> Vec<String> {
    let mut packages: Vec<String> = config_file
        .packages
        .iter()
        .filter_map(|package| {
            if let Some(aur) = aur {
                if package.is_aur() != aur {
                    return None;
                }
            }
            if let Some(enabled) = enabled {
                if package.is_enabled() != enabled {
                    return None;
                }
            }
            if package.get_package_name().starts_with("!") {
                return None;
            }
            return Option::Some(package.get_package_name());
        })
        .collect();

    if !aur.unwrap_or(false) && enabled.unwrap_or(true) {
        packages.append(&mut config_file.packages_simple.clone());
    }

    return packages;
}

pub fn filter_packages_by_enable(packages: Vec<Package>, enable: bool) -> Vec<Package> {
    packages
        .into_iter()
        .filter_map(|package| {
            if package.is_enabled() != enable {
                return None;
            }
            return Option::Some(package);
        })
        .collect()
}

pub fn filter_packages_by_aur(packages: Vec<Package>, aur: bool) -> Vec<Package> {
    packages
        .into_iter()
        .filter_map(|package| {
            if package.is_aur() != aur {
                return None;
            }
            return Option::Some(package);
        })
        .collect()
}

pub fn get_pre_scripts(config_file: &ConfigFile) -> Vec<Script> {
    let packages = filter_packages_by_enable(config_file.packages.clone(), true);
    let packages_with_pre_scripts: Vec<Package> = packages
        .into_iter()
        .filter_map(|package| {
            let scripts: Vec<Script> = package.get_pre_scripts().unwrap_or(Vec::new());
            if scripts.len() == 0 {
                return None;
            }
            return Some(package);
        })
        .collect();
    let scripts: Vec<Script> = packages_with_pre_scripts
        .iter()
        .flat_map(|package| package.get_pre_scripts().unwrap())
        .collect();

    return scripts;
}

pub fn get_post_scripts(config_file: &ConfigFile) -> Vec<Script> {
    let packages = filter_packages_by_enable(config_file.packages.clone(), true);
    let packages_with_pre_scripts: Vec<Package> = packages
        .into_iter()
        .filter_map(|package| {
            let scripts: Vec<Script> = package.get_post_scripts().unwrap_or(Vec::new());
            if scripts.len() == 0 {
                return None;
            }
            return Some(package);
        })
        .collect();
    let scripts: Vec<Script> = packages_with_pre_scripts
        .iter()
        .flat_map(|package| package.get_post_scripts().unwrap())
        .collect();

    return scripts;
}

pub fn install_all_packages(
    config_file: &ConfigFile,
    update: bool,
) -> Result<usize, PackageManagerError> {
    let pacman: PackageManager = PackageManager::new(PackageManagerEnum::Pacman)?;
    let installed_packages: Vec<String> = pacman.get_installed();

    let mut packages_aur: Vec<String> = get_packages_str(&config_file, Some(true), Some(true));
    packages_aur = packages_aur
        .into_iter()
        .filter(|package| !installed_packages.contains(&package))
        .collect();

    let mut packages: Vec<String> = get_packages_str(&config_file, Some(false), Some(true));

    packages = packages
        .into_iter()
        .filter(|package| !installed_packages.contains(&package))
        .collect();

    let packages_aur_len: usize = packages_aur.len();
    let packages_len: usize = packages.len();

    if packages_aur_len > 0 || update {
        let paru: PackageManager = PackageManager::new(PackageManagerEnum::get_aur_helper()?)?;
        paru.install_packages(packages_aur, update)?;
    }

    if packages_len > 0 {
        pacman.install_packages(packages, update)?;
    }

    return Ok(packages_aur_len + packages_len);
}

pub fn remove_all_packages(
    config_file_lock: &ConfigFileLock,
    config_file: &ConfigFile,
    with_dependencies: bool,
) -> Result<usize, PackageManagerError> {
    let pacman: PackageManager = PackageManager::new(PackageManagerEnum::Pacman)?;

    let mut packages_to_remove: Vec<String> = get_packages_str(&config_file, None, Some(false));
    let packages_to_install: Vec<String> = get_packages_str(&config_file, None, Some(true));

    println!("{:?}", config_file_lock);
    println!("{:?}", packages_to_remove);
    let installed_packages: Vec<String> = pacman.get_installed();

    packages_to_remove = packages_to_remove
        .into_iter()
        .filter(|package| installed_packages.contains(&package))
        .collect();
    println!("{:?}", packages_to_remove);
    for package_prev_installed in config_file_lock.packages_installed.clone() {
        if !packages_to_install.contains(&package_prev_installed) {
            packages_to_remove.push(package_prev_installed);
        }
    }
    println!("{:?}", packages_to_remove);
    let packages_len: usize = packages_to_remove.len();
    if packages_len > 0 {
        pacman.uninstall_packages(packages_to_remove, with_dependencies)?;
    }

    return Ok(packages_len);
}

pub fn get_config(path: &str) -> ConfigFile {
    let mut file: File = File::open(path).expect(&format!(
        "{}",
        t(Labels::Error_FileOpenFailed, Some(vec![path.to_owned()]))
    ));
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let config_json: String = String::from_utf8(buf).unwrap();
    let config_file: ConfigFile = serde_jsonc::from_str(&config_json).unwrap();

    return config_file;
}

pub fn get_config_file_lock() -> ConfigFileLock {
    let path = "/etc/nywida/nywida_lock.json";
    let mut file: File = match File::open(path) {
        Ok(file) => file,
        Err(_) => return ConfigFileLock::empty(),
    };
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let config_file_lock_json: String = String::from_utf8(buf).unwrap();
    let config_file_lock: ConfigFileLock = serde_jsonc::from_str(&config_file_lock_json).unwrap();

    return config_file_lock;
}

pub fn save_lock(config_file_lock: &ConfigFileLock) {
    let path = "/etc/nywida/nywida_lock.json";
    let data = json!(config_file_lock).to_string();
    std::fs::write(path, data).expect("Unable to write file");
} 

pub fn copy_file(src: &str, dest: &str) -> Result<(), std::io::Error> {
    println!(
        "{}",
        t(
            Labels::Info_CopyingFile,
            Some(vec![src.to_owned(), dest.to_owned()])
        )
    );
    let filename: &str = src.split("/").last().unwrap();
    if let Err(_) = std::fs::create_dir_all(dest) {
        eprintln!(
            "{}",
            t(Labels::Error_FileAlreadyExists, Some(vec![dest.to_owned()]))
        );
        if let Err(_) = command::ask_continue() {
            return Ok(());
        }
    };
    std::fs::copy(src, format!("{dest}/{filename}"))?;

    Ok(())
}
