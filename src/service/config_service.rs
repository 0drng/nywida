use std::{fs::File, io::Read};

use crate::{
    error::PackageManagerError,
    model::ConfigFile,
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
    let packages_aur: Vec<String> = config_file
        .packages
        .iter()
        .filter_map(|package| {
            if let Some(aur) = aur {
                if package.is_aur != aur {
                    return None;
                }
            }
            if let Some(enabled) = enabled {
                if package.enabled.unwrap_or(true) != enabled {
                    return None;
                }
            }
            return Option::Some(package.package_name.clone());
        })
        .collect();

    return packages_aur;
}

pub fn get_before_scripts(config_file: &ConfigFile) -> Vec<String> {
    let scripts_path: Vec<String> = config_file
        .packages
        .iter()
        .filter_map(|package| {
            if package.enabled.unwrap_or(false) {
                return None;
            }
            package.before_script_path.clone()
        })
        .collect();

    return scripts_path;
}

pub fn get_post_scripts(config_file: &ConfigFile) -> Vec<String> {
    let scripts_path: Vec<String> = config_file
        .packages
        .iter()
        .filter_map(|package| {
            if package.enabled.unwrap_or(false) {
                return None;
            }
            package.post_script_path.clone()
        })
        .collect();

    return scripts_path;
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

    if packages_aur_len > 0 {
        let paru: PackageManager = PackageManager::new(PackageManagerEnum::get_aur_helper()?)?;
        paru.install_packages(packages_aur, update)?;
    }

    if packages_len > 0 {
        pacman.install_packages(packages, update)?;
    }

    return Ok(packages_aur_len + packages_len);
}

pub fn remove_all_packages(
    config_file: &ConfigFile,
    with_dependencies: bool,
) -> Result<usize, PackageManagerError> {
    let pacman: PackageManager = PackageManager::new(PackageManagerEnum::Pacman)?;

    let mut packages: Vec<String> = get_packages_str(&config_file, None, Some(false));
    let installed_packages: Vec<String> = pacman.get_installed();

    packages = packages
        .into_iter()
        .filter(|package| installed_packages.contains(&package))
        .collect();

    let packages_len: usize = packages.len();
    if packages_len > 0 {
        pacman.uninstall_packages(packages, with_dependencies)?;
    }

    return Ok(packages_len);
}

pub fn get_config(path: &str) -> ConfigFile {
    let mut file: File = File::open(path).expect(&format!("{}", t(Labels::Error_FileOpenFailed, Some(vec![path.to_owned()]))));
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let config_json: String = String::from_utf8(buf).unwrap();
    let config_file: ConfigFile = serde_jsonc::from_str(&config_json).unwrap();

    return config_file;
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
        eprintln!("{}", t(Labels::Error_FileAlreadyExists, Some(vec![dest.to_owned()])));
        if let Err(_) = command::ask_continue() {
            return Ok(());
        }
    };
    std::fs::copy(src, format!("{dest}/{filename}"))?;

    Ok(())
}
