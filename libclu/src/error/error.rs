use crate::error::ComponentError;
use fungus::errors::*;
use std::{error::Error as StdError, fmt, io};

/// `Result<T>` provides a simplified result type with a common error type
pub type CluResult<T> = std::result::Result<T, CluError>;

// An error indicating that something went wrong with an arch linux operation
#[derive(Debug)]
pub enum CluError {
    // An error from the component module
    Component(ComponentError),

    // std::io::Error from lower down
    Io(io::Error),

    /// An error from the fungus crate
    Fungus(FuError),

    // An error from the serde_yaml crate
    SerdeYaml(serde_yaml::Error),
}
impl CluError {
    /// Implemented directly on the `Error` type to reduce casting required
    pub fn is<T: StdError + 'static>(&self) -> bool {
        self.as_ref().is::<T>()
    }

    /// Implemented directly on the `Error` type to reduce casting required
    pub fn downcast_ref<T: StdError + 'static>(&self) -> Option<&T> {
        self.as_ref().downcast_ref::<T>()
    }

    /// Implemented directly on the `Error` type to reduce casting required
    pub fn downcast_mut<T: StdError + 'static>(&mut self) -> Option<&mut T> {
        self.as_mut().downcast_mut::<T>()
    }

    /// Implemented directly on the `Error` type to reduce casting required
    /// which allows for using as_ref to get the correct pass through.
    pub fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.as_ref().source()
    }
}
impl StdError for CluError {}

impl fmt::Display for CluError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CluError::Component(ref err) => write!(f, "{}", err),
            CluError::Io(ref err) => write!(f, "{}", err),
            CluError::Fungus(ref err) => write!(f, "{}", err),
            CluError::SerdeYaml(ref err) => write!(f, "{}", err),
        }
    }
}

impl AsRef<dyn StdError> for CluError {
    fn as_ref(&self) -> &(dyn StdError + 'static) {
        match *self {
            CluError::Component(ref err) => err,
            CluError::Io(ref err) => err,
            // Call as_ref on inner to make transparent
            CluError::Fungus(ref err) => err.as_ref(),
            CluError::SerdeYaml(ref err) => err as &(dyn StdError + 'static),
        }
    }
}

impl AsMut<dyn StdError> for CluError {
    fn as_mut(&mut self) -> &mut (dyn StdError + 'static) {
        match *self {
            CluError::Component(ref mut err) => err,
            CluError::Io(ref mut err) => err,
            // Call as_ref on inner to make transparent
            CluError::Fungus(ref mut err) => err.as_mut(),
            CluError::SerdeYaml(ref mut err) => err as &mut (dyn StdError + 'static),
        }
    }
}

impl From<ComponentError> for CluError {
    fn from(err: ComponentError) -> CluError {
        CluError::Component(err)
    }
}

impl From<io::Error> for CluError {
    fn from(err: io::Error) -> CluError {
        CluError::Io(err)
    }
}

impl From<FuError> for CluError {
    fn from(err: FuError) -> CluError {
        CluError::Fungus(err)
    }
}

impl From<serde_yaml::Error> for CluError {
    fn from(err: serde_yaml::Error) -> CluError {
        CluError::SerdeYaml(err)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_errors() {

        // Fungus(FuError),
        let mut err = CluError::from(FuError::from(FileError::FailedToExtractString));
        assert_eq!("failed to extract string from file", err.to_string());
        assert_eq!(
            "failed to extract string from file",
            err.as_ref().to_string()
        );
        assert_eq!(
            "failed to extract string from file",
            err.as_mut().to_string()
        );
        assert!(err.is::<FileError>());
        assert!(err.downcast_ref::<FileError>().is_some());
        assert!(err.downcast_mut::<FileError>().is_some());
        assert!(err.source().is_none());
    }
}
