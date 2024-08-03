use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    runtime::UserData,
    Ctx, Object, Result,
};

pub trait ModuleDefExt: ModuleDef {
    const NAME: &'static str;

    type Options<'js>: UserData<'js>;

    fn declare(decl: &Declarations<'_>) -> Result<()>
    where
        Self: Sized,
    {
        let _ = decl;
        Ok(())
    }

    fn evaluate<'js>(
        options: &Self::Options<'js>,
        ctx: &Ctx<'js>,
        exports: &Exports<'js>,
    ) -> Result<()>
    where
        Self: Sized,
    {
        let _ = (options, exports, ctx);
        Ok(())
    }

    fn globals<'js>(&self, globals: &Object<'js>) -> Result<()> {
        let _ = (globals);
        Ok(())
    }

    fn options(self) -> Result<Self::Options<'static>>;
}

#[macro_export]
macro_rules! module_def {
    ( $x:ident ) => {
        impl rquickjs::module::ModuleDef for $x {
            fn declare(decl: &rquickjs::module::Declarations<'_>) -> rquickjs::Result<()> {
                <Self as $crate::ModuleDefExt>::declare(decl)
            }

            fn evaluate<'js>(
                ctx: &rquickjs::Ctx<'js>,
                exports: &rquickjs::module::Exports<'js>,
            ) -> rquickjs::Result<()> {
                let options = ctx
                    .userdata::<<Self as $crate::ModuleDefExt>::Options<'js>>()
                    .ok_or(rquickjs::Exception::throw_message(
                        ctx,
                        &format!(
                            "Module {} options not found",
                            <Self as $crate::ModuleDefExt>::NAME
                        ),
                    ))?;
                <Self as $crate::ModuleDefExt>::evaluate(&options, ctx, exports)
            }
        }
    };
}
