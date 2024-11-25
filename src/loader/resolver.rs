use std::collections::HashSet;

use rquickjs::{
    loader::{BuiltinResolver, Resolver},
    Ctx, Result,
};

/// Rquickjs [`Resolver`](rquickjs::loader::Resolver) for modules
/// defined using [`ModuleDefExt`](crate::ModuleDefExt).
pub struct ModuleResolver {
    inner: BuiltinResolver,
}

impl ModuleResolver {
    pub(crate) fn new(names: HashSet<&'static str>) -> Self {
        let mut resolver = BuiltinResolver::default();
        for name in names {
            resolver.add_module(name);
        }

        Self { inner: resolver }
    }
}

impl Resolver for ModuleResolver {
    fn resolve(&mut self, ctx: &Ctx<'_>, base: &str, name: &str) -> Result<String> {
        self.inner.resolve(ctx, base, name)
    }
}
