use std::collections::{HashMap, HashSet};

use rquickjs::{
    module::{Module, ModuleDef},
    Ctx, JsLifetime, Object, Result,
};

use super::{GlobalInitializer, GlobalLoadFn, ModuleLoadFn, ModuleLoader, ModuleResolver};
use crate::wrapper::{IntoModule, ModuleMeta};

fn load_module_func<D: ModuleDef>(ctx: Ctx<'_>, name: Vec<u8>) -> Result<Module<'_>> {
    Module::declare_def::<D, _>(ctx, name)
}

/// Builder to create a [`ModuleLoader`], [`ModuleResolver`] and [`GlobalInitializer`]
///
/// # Example
/// ```rust
/// use rquickjs_module::{ModuleLoader, ModuleDefExt, ModuleImpl};
///
/// struct MyModule;
///
/// impl ModuleDefExt for MyModule {
///     type Implementation = ModuleImpl<()>;
///
///     fn implementation() -> &'static Self::Implementation {
///         &ModuleImpl {
///             declare: |decl| {
///                 decl.declare("hello")?;
///                 Ok(())
///             },
///             evaluate: |ctx, exports, options| {
///                 exports.export("hello", "world".to_string())?;
///                 Ok(())
///             },
///             name: "my-module",
///         }
///     }
///
///     fn options(self) -> () {}
/// }
///
/// ```
#[derive(Default)]
pub struct ModuleLoaderBuilder {
    modules: HashMap<&'static str, ModuleLoadFn>,
    globals: Vec<GlobalLoadFn>,
    names: HashSet<&'static str>,
}

impl ModuleLoaderBuilder {
    #[must_use]
    pub fn with_module<O, M, R>(mut self, module: M) -> Self
    where
        for<'js> O: JsLifetime<'js> + Send + Sync + 'static,
        R: ModuleDef + ModuleMeta,
        M: IntoModule<O, R>,
    {
        self.process_module(module, None);
        self
    }

    #[must_use]
    pub fn with_module_named<O, M, R>(mut self, module: M, name: &'static str) -> Self
    where
        for<'js> O: JsLifetime<'js> + Send + Sync + 'static,
        R: ModuleDef + ModuleMeta,
        M: IntoModule<O, R>,
    {
        self.process_module(module, Some(name));
        self
    }

    pub fn add_module<O, M, R>(&mut self, module: M) -> &mut Self
    where
        for<'js> O: JsLifetime<'js> + Send + Sync + 'static,
        R: ModuleDef + ModuleMeta,
        M: IntoModule<O, R>,
    {
        self.process_module(module, None)
    }

    pub fn add_module_named<O, M, R>(&mut self, module: M, name: &'static str) -> &mut Self
    where
        for<'js> O: JsLifetime<'js> + Send + Sync + 'static,
        R: ModuleDef + ModuleMeta,
        M: IntoModule<O, R>,
    {
        self.process_module(module, Some(name))
    }

    fn process_module<O, M, R>(&mut self, module: M, name: Option<&'static str>) -> &mut Self
    where
        for<'js> O: JsLifetime<'js> + Send + Sync + 'static,
        R: ModuleDef + ModuleMeta,
        M: IntoModule<O, R>,
    {
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
            let name = name.unwrap_or(R::name());
            self.names.insert(name);
            self.modules.insert(name, load_module_func::<R>);
        }

        self.globals.push(boxed_globals);
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
