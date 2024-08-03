pub use module_def_ext::ModuleDefExt;
pub use module_loader::{ModuleLoader, ModuleLoaderBuilder};

mod module_def_ext;
mod module_loader;

#[cfg(test)]
mod tests {
    use rquickjs::{
        class::Trace,
        function::Constructor,
        module::{self, Declarations, Exports, ModuleDef},
        runtime::UserData,
        AsyncContext, AsyncRuntime, Ctx, Error, Object, Result, Value,
    };

    use super::{module_def, ModuleDefExt};

    #[derive(Clone, Trace)]
    #[rquickjs::class(frozen)]
    struct Console {
        target: String,
        newline: bool,
    }

    impl Console {
        pub fn new(target: String, newline: bool) -> Self {
            Self { target, newline }
        }
    }

    #[rquickjs::methods]
    impl Console {
        fn log(&self, value: Value<'_>) {
            print!(
                "{}: {:?}{}",
                self.target,
                value,
                if self.newline { "\n" } else { "" }
            );
        }
    }

    struct ConsoleOptions {
        target: String,
        newline: bool,
    }

    unsafe impl<'js> UserData<'js> for ConsoleOptions {
        type Static = ConsoleOptions;
    }

    struct ConsoleModule {
        options: ConsoleOptions,
    }

    impl ConsoleModule {
        pub fn new(options: ConsoleOptions) -> Self {
            Self { options }
        }
    }

    impl ModuleDefExt for ConsoleModule {
        const NAME: &'static str = "console";

        type Options<'js> = ConsoleOptions;

        fn declare(decl: &Declarations<'_>) -> Result<()> {
            decl.declare(stringify!(Console))?;
            Ok(())
        }

        fn evaluate<'js>(
            options: &Self::Options<'js>,
            ctx: &Ctx<'js>,
            exports: &Exports<'js>,
        ) -> Result<()> {
            let target = options.target.clone();
            let newline = options.newline;
            exports.export(
                stringify!(Console),
                Constructor::new_class::<Console, _, _>(ctx.clone(), move || {
                    Ok::<_, Error>(Console::new(target.clone(), newline))
                }),
            )?;

            Ok(())
        }

        fn globals<'js>(options: &Self::Options<'js>, globals: &Object<'js>) -> Result<()> {
            globals.set(
                "console",
                Console::new(options.target.clone(), options.newline),
            )?;
            Ok(())
        }

        fn options(self) -> Result<Self::Options<'static>> {
            Ok(self.options)
        }
    }

    module_def!(ConsoleModule);

    #[tokio::test]
    async fn test() {
        let rt = AsyncRuntime::new().unwrap();
        let ctx = AsyncContext::full(&rt).await.unwrap();
        // let loader

        // async_with!(ctx => |ctx| {
        //     func(ctx).await
        // })
        // .await;
    }
}
