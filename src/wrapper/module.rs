use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    Ctx, JsLifetime, Result,
};

use super::{AsModule, HasModule};
use crate::{ModuleDefExt, ModuleImpl};

pub(crate) struct ModuleDefWrapper<T, O>
where
    T: ModuleDefExt<O, Implementation = ModuleImpl<O>>,
    for<'js> O: JsLifetime<'js>,
{
    _marker: std::marker::PhantomData<T>,
    _marker2: std::marker::PhantomData<O>,
}

impl<T, O> AsModule<O, ModuleDefWrapper<T, O>> for T
where
    T: ModuleDefExt<O, Implementation = ModuleImpl<O>>,
    for<'js> O: JsLifetime<'js> + 'static,
{
    // fn as_module(&self) -> ModuleDefWrapper<T, O> {
    //     ModuleDefWrapper {
    //         _marker: std::marker::PhantomData::<T>,
    //         _marker2: std::marker::PhantomData::<O>,
    //     }
    // }
}

impl<T, O> HasModule for ModuleDefWrapper<T, O>
where
    T: ModuleDefExt<O, Implementation = ModuleImpl<O>>,
    for<'c> O: JsLifetime<'c> + 'static,
{
    fn name() -> &'static str {
        T::implementation().name
    }

    fn is_module() -> bool {
        true
    }
}

impl<T, O> ModuleDef for ModuleDefWrapper<T, O>
where
    T: ModuleDefExt<O, Implementation = ModuleImpl<O>>,
    for<'js> O: JsLifetime<'js> + 'static,
{
    fn declare(decl: &Declarations) -> Result<()> {
        let module_impl = T::implementation();
        (module_impl.declare)(decl)
    }

    fn evaluate<'a>(ctx: &Ctx<'a>, exports: &Exports<'a>) -> Result<()> {
        let module_impl = T::implementation();
        let options = ctx.userdata::<O>().unwrap();
        (module_impl.evaluate)(ctx, exports, &options)
    }
}
