use std::{fs::File, io::Read};

use crate::{error::PackageManagerError, model::ConfigFile, wrapper::{command, package_manager::{PackageManager, PackageManagerEnum}}};

pub fn get_packages_str(config_file: &ConfigFile, aur: bool) -> Vec<String> {
    let packages_aur: Vec<String> = config_file.packages.iter().filter_map(|package| {
        if package.is_aur == aur {
            return Option::Some(package.package_name.clone());
        }
        return None;
    }).collect();

    return packages_aur;
}

pub fn get_before_scripts(config_file: &ConfigFile) -> Vec<String> {
    let scripts_path: Vec<String> = config_file.packages.iter().filter_map(|package| {
        package.before_script_path.clone()
    }).collect();

    return scripts_path;
}

pub fn get_post_scripts(config_file: &ConfigFile) -> Vec<String> {
    let scripts_path: Vec<String> = config_file.packages.iter().filter_map(|package| {
        package.post_script_path.clone()
    }).collect();

    return scripts_path;
}

pub fn install_all_packages(config_file: &ConfigFile, update: bool) -> Result<usize, PackageManagerError> {
    let packages_aur: Vec<String> = get_packages_str(&config_file, true);
    let packages: Vec<String> = get_packages_str(&config_file, false);

    let packages_aur_len: usize = packages_aur.len();
    let packages_len: usize = packages.len();

    if packages_aur_len > 0 {
        let paru: PackageManager = PackageManager::new(PackageManagerEnum::get_aur_helper())?;
        paru.install_packages(packages_aur, update)?;
    }

    if packages_len > 0 {
        let pacman: PackageManager = PackageManager::new(PackageManagerEnum::Pacman)?;
        pacman.install_packages(packages, update)?;
    }

    return Ok(packages_aur_len + packages_len);
}

pub fn get_config(path: &str) -> ConfigFile {
    let mut file: File = File::open(path).expect(&format!("Failed to open {}", path));
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let config_json: String = String::from_utf8(buf).unwrap();
    let config_file: ConfigFile = serde_jsonc::from_str(&config_json).unwrap();

    return config_file;
}

pub fn copy_file(src: &str, dest: &str) -> Result<(), std::io::Error> {
    println!("Copying {} to {}", src, dest);
    let filename: &str = src.split("/").last().unwrap();
    if let Err(e) = std::fs::create_dir_all(dest) {
        eprintln!("File already exists. Overwriting: {}", e);
        if let Err(_) = command::ask_continue() {}
    };
    std::fs::copy(src, format!("{dest}/{filename}"))?;

    Ok(())
}
