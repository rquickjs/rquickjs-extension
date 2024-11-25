use rquickjs::{Ctx, Result};

use super::GlobalLoadFn;

/// Global initializer that MUST be called before any user code is run.
pub struct GlobalInitializer {
    globals: Vec<GlobalLoadFn>,
}

impl GlobalInitializer {
    pub(crate) fn new(globals: Vec<GlobalLoadFn>) -> Self {
        Self { globals }
    }

    pub fn init(self, ctx: &Ctx) -> Result<()> {
        let globals_obj = ctx.globals();
        for globals_fn in self.globals {
            globals_fn(ctx, &globals_obj)?;
        }
        Ok(())
    }
}
