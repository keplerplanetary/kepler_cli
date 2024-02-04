#[derive(Debug)]
pub enum ApplicationError {
    IoError(std::io::Error),
    TomlError(toml::de::Error),
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::IoError(e) => write!(f, "{e}"),
            ApplicationError::TomlError(e) => write!(f, "{e}"),
        }
    }
}

impl From<std::io::Error> for ApplicationError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<toml::de::Error> for ApplicationError {
    fn from(value: toml::de::Error) -> Self {
        Self::TomlError(value)
    }
}
