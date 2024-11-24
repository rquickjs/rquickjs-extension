pub use self::definition::{GlobalsOnly, ModuleDefExt, ModuleImpl};
pub use self::loader::{GlobalInitializer, ModuleLoader, ModuleLoaderBuilder, ModuleResolver};

mod definition;
mod loader;
mod macros;
mod wrapper;

#[cfg(test)]
mod tests {

    use rquickjs::{
        async_with, class::Trace, context::EvalOptions, AsyncContext, AsyncRuntime, CatchResultExt,
        JsLifetime, Object, Result, Value,
    };

    use crate::{
        definition::{GlobalsOnly, ModuleImpl},
        globals_only_module, ModuleLoader,
    };

    use super::ModuleDefExt;

    struct Example;
    impl ModuleDefExt for Example {
        type Implementation = GlobalsOnly;

        fn implementation() -> &'static Self::Implementation {
            &GlobalsOnly
        }

        fn options(self) -> () {
            ()
        }
    }

    struct Example2;
    globals_only_module!(Example2, |globals| {
        // Custom globals initialization code here
        Ok(())
    });

    #[derive(Clone, Trace, JsLifetime)]
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

    #[derive(JsLifetime, Debug)]
    struct ConsoleOptions {
        target: String,
        newline: bool,
    }

    struct ConsoleModule {
        options: ConsoleOptions,
    }

    impl ConsoleModule {
        pub fn new<T: Into<String>>(target: T, newline: bool) -> Self {
            Self {
                options: ConsoleOptions {
                    target: target.into(),
                    newline,
                },
            }
        }
    }

    impl ModuleDefExt<ConsoleOptions> for ConsoleModule {
        type Implementation = ModuleImpl<ConsoleOptions>;

        fn implementation() -> &'static Self::Implementation {
            &ModuleImpl {
                declare: |decl| {
                    decl.declare("default")?;
                    Ok(())
                },
                evaluate: |ctx, exports, options| {
                    println!("Options in eval? {:?}", options);
                    exports.export("default", options.target.clone())?;
                    Ok(())
                },
                name: "console",
            }
        }

        fn options(self) -> ConsoleOptions {
            self.options
        }

        fn globals(globals: &Object<'_>, options: &ConsoleOptions) -> Result<()> {
            println!("Options in globals? {:?}", options);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test() {
        let rt = AsyncRuntime::new().unwrap();

        let mut loader = ModuleLoader::builder();
        loader.add_module(ConsoleModule::new("console", true));

        let (loader, resolver, initalizer) = loader.build();

        // let loader = ModuleLoader::default().with_module(
        //     "console",

        //     .as_module(),
        // );

        rt.set_loader(resolver, loader).await;

        let ctx = AsyncContext::full(&rt).await.unwrap();

        async_with!(ctx => |ctx| {

            if let Err(err) = initalizer.init(&ctx).catch(&ctx){
                eprintln!("{:?}", err);
            }

            let mut opts = EvalOptions::default();
            opts.global = false;

            if let Err(err) = ctx.eval_with_options::<Value,_>(r#"

            import console from "console";

            "#, opts).catch(&ctx){
                eprintln!("{:?}", err);
            }


        })
        .await;
    }
}
