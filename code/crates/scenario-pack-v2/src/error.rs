use std::fmt;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ValidationIssue {
    pub path: String,
    pub code: String,
    pub message: String,
}

impl ValidationIssue {
    pub fn new(
        path: impl Into<String>,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            code: code.into(),
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationError {
    pub issues: Vec<ValidationIssue>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "scenario pack validation failed with {} issue(s)",
            self.issues.len()
        )?;
        for issue in &self.issues {
            writeln!(f, "{} [{}]: {}", issue.path, issue.code, issue.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug)]
pub enum PackLoadError {
    Parse(String),
    Validation(ValidationError),
}

impl fmt::Display for PackLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(message) => write!(f, "scenario pack JSON parse failed: {message}"),
            Self::Validation(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for PackLoadError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InitializeError {
    Registry(sim_kernel_v2::RegistryError),
    Replay(sim_kernel_v2::ReplayError),
}

impl From<sim_kernel_v2::RegistryError> for InitializeError {
    fn from(value: sim_kernel_v2::RegistryError) -> Self {
        Self::Registry(value)
    }
}

impl From<sim_kernel_v2::ReplayError> for InitializeError {
    fn from(value: sim_kernel_v2::ReplayError) -> Self {
        Self::Replay(value)
    }
}

impl fmt::Display for InitializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Registry(error) => write!(f, "scenario registry compilation failed: {error:?}"),
            Self::Replay(error) => write!(f, "scenario world initialization failed: {error:?}"),
        }
    }
}

impl std::error::Error for InitializeError {}
