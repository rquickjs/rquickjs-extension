//! An improved module system for rquickjs
//!
//! This is an extension to [rquickjs](https://github.com/DelSkayn/rquickjs)
//! to allow the ecosystem to create more unified Rust modules.
//!
//! The goal was to create a better version of
//! [`ModuleDef`](rquickjs::module::ModuleDef)
//! that would allow it to have options as input and set global.

pub use self::definition::{Extension, GlobalsOnly, ModuleImpl};
pub use self::loader::{GlobalInitializer, ModuleLoader, ModuleLoaderBuilder, ModuleResolver};

mod definition;
mod loader;
mod macros;
mod wrapper;
