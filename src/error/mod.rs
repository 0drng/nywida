use crate::service::translation_service::{t, Labels};

#[derive(Debug)]
pub struct ApplicationError;

impl ApplicationError {
    pub fn new(label: Labels, params: Option<Vec<String>>) -> Self {
        eprintln!("{}", t(label, params));
        std::process::exit(1);
    }
}

pub enum PackageManagerError {
    NotInstalled(Labels, Option<Vec<String>>),
    WhichIsNotInstalled(Labels, Option<Vec<String>>),
    InstallFailed(Labels, Option<Vec<String>>),
    UninstallFailed(Labels, Option<Vec<String>>),
}

impl From<PackageManagerError> for ApplicationError {
    fn from(value: PackageManagerError) -> Self {
        match value {
            PackageManagerError::NotInstalled(label, params) => {
                ApplicationError::new(label, params)
            },
            PackageManagerError::WhichIsNotInstalled(label, params) => {
                ApplicationError::new(label, params)
            },
            PackageManagerError::InstallFailed(label, params) => {
                ApplicationError::new(label, params)
            },
            PackageManagerError::UninstallFailed(label, params) => {
                ApplicationError::new(label, params)
            }
         }
    }
}

pub enum CommandError {
    UserAbort(Labels, Option<Vec<String>>),
    CommandFailed(Labels, Option<Vec<String>>),
}

impl From<CommandError> for PackageManagerError {
    fn from(value: CommandError) -> Self {
        match value {
            CommandError::UserAbort(labels, params) => {
                PackageManagerError::InstallFailed(labels, params)
            }
            CommandError::CommandFailed(labels, params) => {
                PackageManagerError::InstallFailed(labels, params)
            }
        }
    }
}

impl From<CommandError> for ApplicationError {
    fn from(value: CommandError) -> Self {
        match value {
            CommandError::UserAbort(labels, params) => ApplicationError::new(labels, params),
            CommandError::CommandFailed(labels, params) => ApplicationError::new(labels, params),
        }
    }
}
