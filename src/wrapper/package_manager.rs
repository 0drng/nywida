use crate::{error::PackageManagerError, wrapper::command};
use std::process::{Command, ExitStatus, Output};

pub enum PackageManagerEnum {
    Pacman,
    Paru,
    Yay,
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

    pub fn get_package_param(&self, update: bool) -> String {
        let param = match self {
            PackageManagerEnum::Pacman => "-S".to_owned(),
            PackageManagerEnum::Paru => "-S".to_owned(),
            PackageManagerEnum::Yay => "-S".to_owned(),
        };

        if update {
            return format!("{}yu", param);
        }

        return param;
    }
}

pub struct PackageManager {
    package_manager: PackageManagerEnum,
}

impl PackageManager {
    pub fn new(package_manager: PackageManagerEnum) -> Result<Self, PackageManagerError> {
        Self::check_installed(&package_manager)?;
        Ok(PackageManager { package_manager })
    }

    pub fn install_packages(
        &self,
        mut packages: Vec<String>,
        update: bool,
    ) -> Result<(), PackageManagerError> {
        let program: &str = &self.package_manager.get_command();
        let mut args: Vec<String> = vec![self.package_manager.get_package_param(update)];
        args.append(&mut packages);
        let command: Result<ExitStatus, std::io::Error> = command::run_command(program, args);

        let exit_status: ExitStatus = command.expect("Failed to read exit status");

        if !exit_status.success() {
            return Err(PackageManagerError::InstallFailed(
                "package.error.installFailed".to_owned(),
                Vec::new(),
            ));
        }

        Ok(())
    }

    pub fn check_installed(
        package_manager: &PackageManagerEnum,
    ) -> Result<(), PackageManagerError> {
        let output_result: Result<Output, std::io::Error> = Command::new("which")
            .arg(package_manager.get_command())
            .output();

        let output = match output_result {
            Ok(output) => output,
            Err(_) => {
                return Err(PackageManagerError::WhichIsNotInstalled(
                    "package.error.whichIsNotInstalled".to_owned(),
                    Vec::new(),
                ))
            }
        };

        if !output.status.success() {
            return Err(PackageManagerError::NotInstalled(
                "package.error.notInstalled".to_owned(),
                Vec::new(),
            ));
        }

        Ok(())
    }
}
