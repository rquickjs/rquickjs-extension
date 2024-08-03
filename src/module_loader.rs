use std::collections::HashMap;

use rquickjs::{
    loader::Loader,
    module::{Module, ModuleDef},
    AsyncContext, Ctx, Error, Result,
};

use crate::ModuleDefExt;

type LoadFn = for<'js> fn(Ctx<'js>, Vec<u8>) -> Result<Module<'js>>;

fn load_func<'js, D: ModuleDef>(ctx: Ctx<'js>, name: Vec<u8>) -> Result<Module<'js>> {
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

#[derive(Debug, Default)]
pub struct ModuleLoaderBuilder {
    definitions: HashMap<&'static str, LoadFn>,
    //instances: HashMap<&'static str, Box<dyn ModuleDefExt>>,
    globals: HashMap<&'static str, bool>,
}

impl ModuleLoaderBuilder {
    pub async fn add_module<M: ModuleDefExt>(&mut self, module: M, global: bool) -> &mut Self {
        self.definitions.insert(M::NAME, load_func::<M>);
        //self.instances.insert(M::NAME, Box::new(module));
        self.globals.insert(M::NAME, global);
        self
    }

    pub async fn build(&mut self, ctx: &AsyncContext) -> Result<ModuleLoader> {
        // ctx.with(|ctx| {
        //     let globals = ctx.globals();

        //     for (name, global) in self.globals {
        //         if global {
        //             let module = self
        //                 .globals
        //                 .get(name)
        //                 .expect("Module should be instanciated");
        //             module.globals(globals)?;
        //         }
        //     }
        // });

        // Ok(ModuleLoader {
        //     modules: self.definitions,
        // })
        todo!()
    }
}
