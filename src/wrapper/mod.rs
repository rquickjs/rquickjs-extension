use rquickjs::{module::ModuleDef, JsLifetime};

use crate::Extension;

mod globals;
mod module;

/// Module metadata
///
/// We use this trait to still access metadata once we have
/// converted it from an [`Extension`] to a [`ModuleDef`].
///
/// This is necessary for the [`ExtensionBuilder`](crate::ExtensionBuilder) to work.
pub trait ModuleMeta {
    fn name() -> &'static str;
    fn is_module() -> bool;
}

/// Semantically convert an [`Extension`] to a [`ModuleDef`] and [`ModuleMeta`]
pub trait IntoModule<O, R>
where
    Self: Extension<O>,
    R: ModuleDef + ModuleMeta,
    for<'js> O: JsLifetime<'js>,
{
}
