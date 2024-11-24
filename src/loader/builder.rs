use std::collections::{HashMap, HashSet};

use rquickjs::{
    module::{Module, ModuleDef},
    Ctx, JsLifetime, Object, Result,
};

use super::{GlobalInitializer, GlobalLoadFn, ModuleLoadFn, ModuleLoader, ModuleResolver};
use crate::wrapper::{AsModule, HasModule};

fn load_module_func<D: ModuleDef>(ctx: Ctx<'_>, name: Vec<u8>) -> Result<Module<'_>> {
    Module::declare_def::<D, _>(ctx, name)
}

#[derive(Default)]
pub struct ModuleLoaderBuilder {
    modules: HashMap<&'static str, ModuleLoadFn>,
    globals: Vec<GlobalLoadFn>,
    names: HashSet<&'static str>,
}

impl ModuleLoaderBuilder {
    pub fn add_module<O, M, R>(&mut self, module: M) -> &mut Self
    where
        for<'js> O: JsLifetime<'js> + 'static,
        R: ModuleDef + HasModule,
        M: AsModule<O, R>,
    {
        let name = R::name();
        let m = module.as_module();
        let o = module.options();

        // Create a new closure that explicitly captures 'js lifetime
        let globals_fn = move |ctx: &Ctx<'_>, globals: &Object<'_>| {
            let globals_fn = M::globals;
            globals_fn(globals, &o)?;
            let _ = ctx.store_userdata(o);
            Ok(())
        };

        // Box the closure with explicit lifetime bounds
        let boxed_globals: GlobalLoadFn = Box::new(globals_fn);

        if R::is_module() {
            self.insert_module(name, m);
        }

        self.globals.push(boxed_globals);
        self
    }

    fn insert_module<M: ModuleDef>(&mut self, name: &'static str, _module: M) -> &mut Self {
        self.names.insert(name);
        self.modules.insert(name, load_module_func::<M>);
        self
    }

    pub fn build(self) -> (ModuleLoader, ModuleResolver, GlobalInitializer) {
        (
            ModuleLoader::new(self.modules),
            ModuleResolver::new(self.names),
            GlobalInitializer::new(self.globals),
        )
    }
}
