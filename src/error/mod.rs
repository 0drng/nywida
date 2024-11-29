#[derive(Debug)]
pub struct ApplicationError {
    label: String,
    params: Vec<String>,
}

impl ApplicationError {
    pub fn new(label: &str, params: Vec<String>) -> Self {
        ApplicationError { label: label.to_owned(), params }
    }
}

pub enum PackageManagerError {
    NotInstalled(String, Vec<String>),
    WhichIsNotInstalled(String, Vec<String>),
    InstallFailed(String, Vec<String>),
    UserAbort(String, Vec<String>),
}

impl From<PackageManagerError> for ApplicationError {
    fn from(value: PackageManagerError) -> Self {
        match value {
            PackageManagerError::NotInstalled(label, params) => ApplicationError::new(&label, params),
            PackageManagerError::WhichIsNotInstalled(label, params) => ApplicationError::new(&label, params),
            PackageManagerError::InstallFailed(label, params) => ApplicationError::new(&label, params),
            PackageManagerError::UserAbort(label, params) => ApplicationError::new(&label, params),
        }
    }
}