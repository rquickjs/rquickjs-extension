use rquickjs::{async_with, AsyncContext, AsyncRuntime, CatchResultExt, Object, Result};
use rquickjs_extension::{globals_only_module, Extension, ExtensionBuilder, GlobalsOnly};

use self::common::{Printer, PrinterOptions};

mod common;

struct PrinterModule {
    options: PrinterOptions,
}

impl PrinterModule {
    pub fn new<T: Into<String>>(target: T) -> Self {
        Self {
            options: PrinterOptions {
                target: target.into(),
            },
        }
    }
}

impl Extension<PrinterOptions> for PrinterModule {
    type Implementation = GlobalsOnly;

    fn implementation() -> &'static Self::Implementation {
        &GlobalsOnly
    }

    fn options(self) -> PrinterOptions {
        self.options
    }

    fn globals(globals: &Object<'_>, options: &PrinterOptions) -> Result<()> {
        globals.set("global_printer", Printer::new(options.target.clone()))?;
        Ok(())
    }
}

struct PrinterModule2;
globals_only_module!(PrinterModule2, |globals| {
    globals.set("global_printer", Printer::new("emile".to_string()))?;
    Ok(())
});

#[tokio::test]
async fn test_global() {
    let rt = AsyncRuntime::new().unwrap();

    let (loader, resolver, initalizer) = ExtensionBuilder::new()
        .with_extension(PrinterModule::new("world"))
        .build();

    rt.set_loader(resolver, loader).await;

    let ctx = AsyncContext::full(&rt).await.unwrap();

    async_with!(ctx => |ctx| {
        initalizer.init(&ctx).unwrap();

        let result = ctx.eval::<String,_>(r#"
            global_printer.print()
        "#).catch(&ctx).unwrap();
        assert_eq!(result, "hello world");
    })
    .await;
}

// Enable once https://github.com/DelSkayn/rquickjs/pull/395 is merged
// #[tokio::test]
// async fn test_global_macro() {
//     let rt = AsyncRuntime::new().unwrap();

//     let (loader, resolver, initalizer) =
//         ModuleLoader::builder().with_module(PrinterModule2).build();

//     rt.set_loader(resolver, loader).await;

//     let ctx = AsyncContext::full(&rt).await.unwrap();

//     async_with!(ctx => |ctx| {
//         initalizer.init(&ctx).unwrap();

//         let result = ctx.eval::<String,_>(r#"
//             global_printer.print()
//         "#).catch(&ctx).unwrap();
//         assert_eq!(result, "hello emile");
//     })
//     .await;
// }
