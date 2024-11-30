//! An extension system for rquickjs
//!
//! This is a complement to [rquickjs](https://github.com/DelSkayn/rquickjs)
//! to allow the ecosystem to create more unified Rust extensions.
//!
//! The goal was to create a more generic version of
//! [`ModuleDef`](rquickjs::module::ModuleDef)
//! that would allow it to have options and/or set global values.

pub use self::definition::{Extension, GlobalsOnly, ModuleImpl};
pub use self::loader::{ExtensionBuilder, GlobalInitializer, ModuleLoader, ModuleResolver};

mod definition;
mod loader;
mod macros;
mod wrapper;
