use rquickjs::{module::ModuleDef, JsLifetime};

use crate::ModuleDefExt;

mod globals;
mod module;

/// Module metadata
///
/// We use this trait to still access metadata once we have
/// converted it from a [`ModuleDefExt`] to a [`ModuleDef`].
///
/// This is necessary for the loader to work.
pub trait HasModule {
    fn name() -> &'static str;
    fn is_module() -> bool;
}

/// Semantically convert a [`ModuleDefExt`] to a [`ModuleDef`] and [`HasModule`]
pub trait AsModule<O, R>
where
    Self: ModuleDefExt<O>,
    R: ModuleDef + HasModule,
    for<'js> O: JsLifetime<'js>,
{
}
