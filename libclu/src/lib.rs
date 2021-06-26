pub mod core;
pub mod error;
pub mod model;

pub use crate::error::{CluError, CluResult};

/// All essential symbols in a simple consumable form
///
/// ### Examples
/// ```
/// use libclu::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{core::*, error::*, model::*};

    // Re-exports
    pub use fungus::prelude::*;
}
