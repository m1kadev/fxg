mod data;
pub use data::{Project, ProjectMeta};

mod new;
#[cfg(feature = "developer")]
pub use new::{new, TEMPLATE_FXG};