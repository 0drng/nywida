use crate::{error::PackageManagerError, service::translation_service::Labels, wrapper::command};
use std::process::{Command, ExitStatus, Output};

pub enum PackageManagerEnum {
    Pacman,
    Paru,
    Yay,
}

impl PackageManagerEnum {
    pub fn get_aur_helper() -> Result<PackageManagerEnum, PackageManagerError> {
        if PackageManager::check_installed(&PackageManagerEnum::Paru).is_ok() {
            return Ok(PackageManagerEnum::Paru);
        }
        if PackageManager::check_installed(&PackageManagerEnum::Yay).is_ok() {
            return Ok(PackageManagerEnum::Yay);
        }
        Err(PackageManagerError::NotInstalled(
            Labels::Error_NoAURHelper,
            None,
        ))
    }

    pub fn get_command(&self) -> String {
        match self {
            PackageManagerEnum::Pacman => "pacman".to_owned(),
            PackageManagerEnum::Paru => "paru".to_owned(),
            PackageManagerEnum::Yay => "yay".to_owned(),
        }
    }

    pub fn get_install_param(&self, update: bool) -> String {
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

    pub fn get_uninstall_param(&self, with_dependencies: bool) -> String {
        let param = match self {
            PackageManagerEnum::Pacman => "-R".to_owned(),
            PackageManagerEnum::Paru => "-R".to_owned(),
            PackageManagerEnum::Yay => "-R".to_owned(),
        };

        if with_dependencies {
            return format!("{}ncs", param);
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
        let mut args: Vec<String> = vec![self.package_manager.get_install_param(update)];
        args.append(&mut packages);
        let exit_status: ExitStatus = command::run_command(program, args)?;

        if !exit_status.success() {
            return Err(PackageManagerError::InstallFailed(
                Labels::Error_InstallationFailed,
                None,
            ));
        }

        Ok(())
    }

    pub fn uninstall_packages(
        &self,
        mut packages: Vec<String>,
        with_dependencies: bool,
    ) -> Result<(), PackageManagerError> {
        let program: &str = &self.package_manager.get_command();
        let mut args: Vec<String> =
            vec![self.package_manager.get_uninstall_param(with_dependencies)];
        args.append(&mut packages);
        let exit_status: ExitStatus = command::run_command(program, args)?;

        if !exit_status.success() {
            return Err(PackageManagerError::UninstallFailed(
                Labels::Error_UninstallFailed,
                None,
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
                    Labels::Error_Which_NotInstalled,
                    None,
                ))
            }
        };

        if !output.status.success() {
            return Err(PackageManagerError::NotInstalled(
                Labels::Error_PackageManager_NotInstalled,
                None,
            ));
        }

        Ok(())
    }

    pub fn get_installed(&self) -> Vec<String> {
        let output: Output = Command::new(&self.package_manager.get_command())
            .arg("-Qqe")
            .output()
            .unwrap();

        return String::from_utf8(output.stdout)
            .unwrap()
            .split("\n")
            .map(|f| f.to_owned())
            .collect();
    }
}
