//! An improved module system for rquickjs
//!
//! This is an extension to [rquickjs](https://github.com/DelSkayn/rquickjs)
//! to allow the ecosystem to create more unified Rust modules.
//!
//! The goal was to create a better version of
//! [`ModuleDef`](rquickjs::module::ModuleDef)
//! that would allow it to have options as input and set global.

pub use self::definition::{GlobalsOnly, ModuleDefExt, ModuleImpl};
pub use self::loader::{GlobalInitializer, ModuleLoader, ModuleLoaderBuilder, ModuleResolver};

mod definition;
mod loader;
mod macros;
mod wrapper;

#[cfg(test)]
mod tests {
    use rquickjs::{
        async_with, class::Trace, context::EvalOptions, AsyncContext, AsyncRuntime, CatchResultExt,
        JsLifetime, Object, Result, Value,
    };

    use super::*;

    struct Example;
    impl ModuleDefExt for Example {
        type Implementation = GlobalsOnly;

        fn implementation() -> &'static Self::Implementation {
            &GlobalsOnly
        }

        fn options(self) {}
    }

    struct Example2;
    globals_only_module!(Example2, |globals| {
        // Custom globals initialization code here
        Ok(())
    });
}
