use rquickjs::{Ctx, Result};

use super::GlobalLoadFn;

pub struct GlobalInitializer {
    globals: Vec<GlobalLoadFn>,
}

unsafe impl Send for GlobalInitializer {}
unsafe impl Sync for GlobalInitializer {}

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
