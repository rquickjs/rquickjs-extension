use rquickjs::{
    async_with, AsyncContext, AsyncRuntime, CatchResultExt, Function, Module, Object, Result,
};
use rquickjs_extension::{Extension, ExtensionBuilder, ModuleImpl};

use self::common::{Printer, PrinterOptions};

mod common;

struct PrinterExtension {
    options: PrinterOptions,
}

impl PrinterExtension {
    pub fn new<T: Into<String>>(target: T) -> Self {
        Self {
            options: PrinterOptions {
                target: target.into(),
            },
        }
    }
}

impl Extension<PrinterOptions> for PrinterExtension {
    type Implementation = ModuleImpl<PrinterOptions>;

    fn implementation() -> &'static Self::Implementation {
        &ModuleImpl {
            declare: |decl| {
                decl.declare("default")?;
                Ok(())
            },
            evaluate: |_ctx, exports, options| {
                exports.export("default", Printer::new(options.target.clone()))?;
                Ok(())
            },
            name: "printer",
        }
    }

    fn options(self) -> PrinterOptions {
        self.options
    }

    fn globals(globals: &Object<'_>, options: &PrinterOptions) -> Result<()> {
        globals.set("global_printer", Printer::new(options.target.clone()))?;
        Ok(())
    }
}

#[tokio::test]
async fn test_extension() {
    let rt = AsyncRuntime::new().unwrap();

    let (loader, resolver, initalizer) = ExtensionBuilder::new()
        .with_extension(PrinterExtension::new("john"))
        .build();

    rt.set_loader(resolver, loader).await;

    let ctx = AsyncContext::full(&rt).await.unwrap();

    async_with!(ctx => |ctx| {
        initalizer.init(&ctx).unwrap();

        let (module, module_eval) = Module::declare(ctx.clone(), "test", r#"
            import printer from "printer";
            export function testFunc() {
                return printer.print();
            }
        "#).unwrap().eval().unwrap();
        module_eval.into_future::<()>().await.unwrap();
        let result = module.get::<_, Function>("testFunc").unwrap().call::<_, String>(()).unwrap();
        assert_eq!(result, "hello john");
    })
    .await;
}

#[tokio::test]
async fn test_extension_named() {
    let rt = AsyncRuntime::new().unwrap();

    let (loader, resolver, initalizer) = ExtensionBuilder::new()
        .with_extension_named(PrinterExtension::new("arnold"), "custom_printer")
        .build();

    rt.set_loader(resolver, loader).await;

    let ctx = AsyncContext::full(&rt).await.unwrap();

    async_with!(ctx => |ctx| {
        initalizer.init(&ctx).unwrap();

        let (module, module_eval) = Module::declare(ctx.clone(), "test", r#"
            import printer from "custom_printer";
            export function testFunc() {
                return printer.print();
            }
        "#).unwrap().eval().unwrap();
        module_eval.into_future::<()>().await.unwrap();
        let result = module.get::<_, Function>("testFunc").unwrap().call::<_, String>(()).unwrap();
        assert_eq!(result, "hello arnold");
    })
    .await;
}

#[tokio::test]
async fn test_extension_global() {
    let rt = AsyncRuntime::new().unwrap();

    let (loader, resolver, initalizer) = ExtensionBuilder::new()
        .with_extension(PrinterExtension::new("david"))
        .build();

    rt.set_loader(resolver, loader).await;

    let ctx = AsyncContext::full(&rt).await.unwrap();

    async_with!(ctx => |ctx| {
        initalizer.init(&ctx).unwrap();

        let result = ctx.eval::<String,_>(r#"
            global_printer.print()
        "#).catch(&ctx).unwrap();
        assert_eq!(result, "hello david");
    })
    .await;
}
