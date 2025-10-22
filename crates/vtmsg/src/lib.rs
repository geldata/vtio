//! Terminal command sequences implementing common VT escape codes.
//!
//! This crate provides types that implement the `Encode` trait to render
//! terminal control sequences for cursor movement, screen manipulation,
//! and terminal state queries.

#![warn(clippy::pedantic)]

pub mod charset;
pub mod cursor;
pub mod dsr;
pub mod iterm;
pub mod keyboard;
pub mod mouse;
pub mod screen;
pub mod scroll;
pub mod shell;
pub mod terminal;
pub mod traits;
pub mod window;

pub use crate::traits::TerseDisplay;
