#![warn(clippy::pedantic)]

pub mod event;
mod traits;

pub use crate::traits::TerseDisplay;
pub use vtio_control_derive::VTControl;
