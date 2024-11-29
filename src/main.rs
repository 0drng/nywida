use std::{fs::File, io::Read};

use error::ApplicationError;
use model::{ConfigFile, DotConfig};
use wrapper::package_manager::{PackageManager, PackageManagerEnum};

mod error;
mod model;
mod wrapper;

fn main() -> Result<(), ApplicationError>{    
    let config_file: ConfigFile = get_config();

    let packages: Vec<String> = config_file.packages.iter().filter_map(|s| {
        if !s.is_aur {
            return Option::Some(s.package_name.clone());
        }
        return None;
    }).collect();

    let packages_aur: Vec<String> = config_file.packages.iter().filter_map(|s| {
        if s.is_aur {
            return Option::Some(s.package_name.clone());
        }
        return None;
    }).collect();

    if packages.len() > 0 {
        let pacman: PackageManager = PackageManager::new(PackageManagerEnum::Pacman)?;
        pacman.install_packages(&packages)?;
    }

    if packages_aur.len() > 0 {
        let paru: PackageManager = PackageManager::new(PackageManagerEnum::get_aur_helper())?;
        paru.install_packages(&packages_aur)?;
    }

    let dot_files: Vec<DotConfig> = config_file.packages.iter().filter_map(|f| f.dot_config.clone()).collect();
    
    for dot_file in dot_files {
        copy_file(&dot_file.src, &dot_file.dest).unwrap();
    }
    
    Ok(())
}


pub fn get_config() -> ConfigFile {
    let mut file: File = File::open("/etc/nywida/configuration.jsonc").expect("Failed to open /etc/nywida/configuration.jsonc");
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
        eprintln!("Path already exists. Overwriting: {}", e);
    };
    std::fs::copy(src, format!("{dest}/{filename}"))?;

    Ok(())
}