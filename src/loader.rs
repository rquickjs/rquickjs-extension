use std::collections::HashMap;

use rquickjs::{
    loader::Loader,
    module::{Module, ModuleDef},
    Ctx, Error, JsLifetime, Object, Result,
};

use crate::definition::ModuleDefExt;
use crate::wrapper::{AsModule, HasModule};

type LoadFn = for<'js> fn(Ctx<'js>, Vec<u8>) -> Result<Module<'js>>;

fn load_func<D: ModuleDef>(ctx: Ctx<'_>, name: Vec<u8>) -> Result<Module<'_>> {
    Module::declare_def::<D, _>(ctx, name)
}

pub struct ModuleLoader {
    modules: HashMap<&'static str, LoadFn>,
}

impl ModuleLoader {
    pub fn builder() -> ModuleLoaderBuilder {
        ModuleLoaderBuilder::default()
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

#[derive(Default)]
pub struct ModuleLoaderBuilder {
    modules: HashMap<&'static str, LoadFn>,
    globals: Vec<Box<dyn for<'js> FnOnce(&Ctx<'js>, &Object<'js>) -> Result<()>>>,
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
        let boxed_globals: Box<dyn for<'js> FnOnce(&Ctx<'js>, &Object<'js>) -> Result<()>> =
            Box::new(globals_fn);

        if R::is_module() {
            self.insert_module(name, m);
        }

        self.globals.push(boxed_globals);
        self
    }

    fn insert_module<M: ModuleDef>(&mut self, name: &'static str, _module: M) -> &mut Self {
        self.modules.insert(name, load_func::<M>);
        self
    }

    pub fn build(self) -> (ModuleLoader, GlobalInitializer) {
        let globals = self.globals;
        let modules = self.modules;
        (ModuleLoader { modules }, GlobalInitializer { globals })
    }
}

pub struct GlobalInitializer {
    globals: Vec<Box<dyn for<'js> FnOnce(&Ctx<'js>, &Object<'js>) -> Result<()>>>,
}

unsafe impl Send for GlobalInitializer {}
unsafe impl Sync for GlobalInitializer {}

impl GlobalInitializer {
    pub fn init(self, ctx: &Ctx) -> Result<()> {
        let globals_obj = ctx.globals();
        for globals_fn in self.globals {
            globals_fn(ctx, &globals_obj)?;
        }
        Ok(())
    }
}
