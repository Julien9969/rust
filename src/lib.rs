#![warn(clippy::all, rust_2018_idioms)]

mod vue;
mod app;
pub use app::ActivityTrackerApp;
pub use vue::histogram;
