use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    Ctx, JsLifetime, Result,
};

use super::{AsModule, HasModule};
use crate::{GlobalsOnly, ModuleDefExt};

pub struct GlobalDefWrapper<T, O>
where
    T: ModuleDefExt<O, Implementation = GlobalsOnly>,
    for<'js> O: JsLifetime<'js>,
{
    _marker: std::marker::PhantomData<T>,
    _marker2: std::marker::PhantomData<O>,
}

impl<T, O> AsModule<O, GlobalDefWrapper<T, O>> for T
where
    T: ModuleDefExt<O, Implementation = GlobalsOnly>,
    for<'js> O: JsLifetime<'js>,
{
    fn as_module(&self) -> GlobalDefWrapper<T, O> {
        GlobalDefWrapper {
            _marker: std::marker::PhantomData::<T>,
            _marker2: std::marker::PhantomData::<O>,
        }
    }
}

impl<T, O> HasModule for GlobalDefWrapper<T, O>
where
    T: ModuleDefExt<O, Implementation = GlobalsOnly>,
    for<'js> O: JsLifetime<'js>,
{
    fn name() -> &'static str {
        unimplemented!("Global modules do not have a name")
    }

    fn is_module() -> bool {
        // This ensures the loader won't try to use this as a module
        // even though the wrapper implements ModuleDef.
        false
    }
}

impl<T, O> ModuleDef for GlobalDefWrapper<T, O>
where
    T: ModuleDefExt<O, Implementation = GlobalsOnly>,
    for<'js> O: JsLifetime<'js>,
{
    //unused
    fn declare(_decl: &Declarations) -> Result<()> {
        unimplemented!("Global modules do not declare anything")
    }

    //unused
    fn evaluate<'a>(_ctx: &Ctx<'a>, _exports: &Exports<'a>) -> Result<()> {
        unimplemented!("Global modules do not evaluate anything")
    }
}
