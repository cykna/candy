///Helpers for managing the default skia renderer for candy

#[cfg(feature = "opengl")]
mod opengl;

#[cfg(feature = "opengl")]
pub use opengl::{create_context, create_surface};
