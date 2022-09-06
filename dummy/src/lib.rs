/// Dummy documentation
pub mod dummy;
pub use crate::dummy::Dummy;
pub mod dummy_impl;

#[cfg(feature = "derive")]
pub use dummy_rs_derive;
