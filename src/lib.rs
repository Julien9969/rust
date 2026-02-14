#![warn(clippy::all, rust_2018_idioms)]

mod vue;
mod app;
mod core;
pub use app::ActivityTrackerApp;
pub use vue::histogram;
pub use core::collector;
