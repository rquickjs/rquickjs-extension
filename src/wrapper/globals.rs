use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    Ctx, JsLifetime, Result,
};

use super::{IntoModule, ModuleMeta};
use crate::{GlobalsOnly, Extension};

pub struct GlobalDefWrapper<T, O>
where
    T: Extension<O, Implementation = GlobalsOnly>,
    for<'js> O: JsLifetime<'js>,
{
    _marker: std::marker::PhantomData<T>,
    _marker2: std::marker::PhantomData<O>,
}

impl<T, O> IntoModule<O, GlobalDefWrapper<T, O>> for T
where
    T: Extension<O, Implementation = GlobalsOnly>,
    for<'js> O: JsLifetime<'js>,
{
}

impl<T, O> ModuleMeta for GlobalDefWrapper<T, O>
where
    T: Extension<O, Implementation = GlobalsOnly>,
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
    T: Extension<O, Implementation = GlobalsOnly>,
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
