use std::collections::HashMap;

use rquickjs::{loader::Loader, Ctx, Error, Module, Result};

use super::ModuleLoadFn;

/// Rquickjs [`Loader`](rquickjs::loader::Loader) for modules
/// defined using [`Extension`](crate::Extension).
pub struct ModuleLoader {
    modules: HashMap<&'static str, ModuleLoadFn>,
}

impl ModuleLoader {
    pub(crate) fn new(modules: HashMap<&'static str, ModuleLoadFn>) -> Self {
        Self { modules }
    }
}

impl Loader for ModuleLoader {
    fn load<'js>(&mut self, ctx: &Ctx<'js>, path: &str) -> Result<Module<'js>> {
        let load = self
            .modules
            .remove(path)
            .ok_or_else(|| Error::new_loading(path))?;

        (load)(ctx.clone(), Vec::from(path))
    }
}
