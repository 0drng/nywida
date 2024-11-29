use crate::error::PackageManagerError;
use std::{io::Write, process::{Command, Output, Stdio}};

pub enum PackageManagerEnum {
    Pacman,
    Paru,
    Yay
}

impl PackageManagerEnum {

    pub fn get_aur_helper() -> PackageManagerEnum {
        if PackageManager::check_installed(&PackageManagerEnum::Paru).is_ok() {
            return PackageManagerEnum::Paru;
        }
        if PackageManager::check_installed(&PackageManagerEnum::Yay).is_ok() {
            return PackageManagerEnum::Yay;
        }
        panic!("No AUR helper found");
    }

    pub fn get_command(&self) -> String {
        match self {
            PackageManagerEnum::Pacman => "pacman".to_owned(),
            PackageManagerEnum::Paru => "paru".to_owned(),
            PackageManagerEnum::Yay => "yay".to_owned(),
        }
    }
}

pub struct PackageManager {
    package_manager: PackageManagerEnum
}

impl PackageManager {
    pub fn new(package_manager: PackageManagerEnum) -> Result<Self, PackageManagerError> {
        Self::check_installed(&package_manager)?;
        Ok(PackageManager { package_manager })
    }

    pub fn install_packages(&self, packages: &[String]) -> Result<(), PackageManagerError> {
        let packages_str: String = packages.join("\n");

        println!("Following packages will be installed\n{}", packages_str);
        let _ = &self.ask_continue()?;

        let output_result: Result<Output, std::io::Error> = Command::new(&self.package_manager.get_command())
            .arg("-S")
            .arg(packages.join(" "))
            .arg("--noconfirm")
            .stdout(Stdio::piped())
            .output();

        let output = match output_result {
            Ok(output) => output,
            Err(_) => return Err(PackageManagerError::InstallFailed("package.error.horribleWrong".to_owned(), Vec::new())),
        };

        if !output.status.success() {
            let stdout = String::from_utf8(output.stdout).unwrap();
            println!("{}", stdout);
            return Err(PackageManagerError::InstallFailed("package.error.installFailed".to_owned(), Vec::new()));
        }

        Ok(())
    }

    pub fn check_installed(package_manager: &PackageManagerEnum) -> Result<(), PackageManagerError> {
        let output_result: Result<Output, std::io::Error> =
            Command::new("which").arg(package_manager.get_command()).output();

        let output = match output_result {
            Ok(output) => output,
            Err(_) => return Err(PackageManagerError::WhichIsNotInstalled("package.error.whichIsNotInstalled".to_owned(), Vec::new())),
        };

        if !output.status.success() {
            return Err(PackageManagerError::NotInstalled("package.error.notInstalled".to_owned(), Vec::new()));
        }

        Ok(())
    }

    fn ask_continue(&self) -> Result<(), PackageManagerError> {
        print!("Do you want to continue? y/N: ");
        std::io::stdout().flush().unwrap();
        let mut val: String = String::new();
        std::io::stdin().read_line(&mut val).unwrap();
        let val = val.trim();
        if val.eq("yes") | val.eq("y") {
            return Ok(())
        }
        Err(PackageManagerError::UserAbort("error.userAbort".to_owned(), Vec::new()))
    }
}
