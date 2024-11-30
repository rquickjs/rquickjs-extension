# rquickjs extension

[![github](https://img.shields.io/badge/github-rquickjs/rquickjs-extension.svg?style=for-the-badge&logo=github)](https://github.com/rquickjs/rquickjs-extension)
[![crates](https://img.shields.io/crates/v/rquickjs-extension.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/rquickjs-extension)

This is a complement to [rquickjs](https://github.com/DelSkayn/rquickjs) to allow the ecosystem to create more unified Rust extensions.

The goal was to create a more generic version of [`ModuleDef`](https://docs.rs/rquickjs/latest/rquickjs/module/trait.ModuleDef.html) that would allow it to have options and/or set global values.

For example, a `fetch` extension could set a global `fetch` function and have as options an allowlist of domains.

## Using extensions

If you are a consumer of extensions created using that crate, here is how you can import them in your runtime.

```rust
use rquickjs::AsyncRuntime;
use rquickjs_extension::ExtensionBuilder;

#[tokio::main]
async fn main() {
    let rt = AsyncRuntime::new().unwrap();

    let (loader, resolver, initalizer) = ExtensionBuilder::new().with_module(MyExtension).build();

    rt.set_loader(resolver, loader).await;

    let ctx = AsyncContext::full(&rt).await.unwrap();

    async_with!(ctx => |ctx| {
        if let Err(err) = initalizer.init(&ctx).catch(&ctx){
            eprintln!("{:?}", err);
        }
    })
    .await;
}
```

## Creating extensions

For the base case, replace the `ModuleDef` by an implementation of `Extension`.

```rust
use rquickjs::{Ctx, JsLifetime, Object, Result};
use rquickjs_extension::{Extension, ModuleImpl};

#[derive(JsLifetime, Debug)]
struct MyExtensionOptions {
    user: String,
}

struct MyExtension {
    options: MyExtensionOptions,
}

impl MyExtension {
    pub fn new<T: Into<String>>(user: T) -> Self {
        Self {
            options: MyExtensionOptions {
                user: user.into(),
            },
        }
    }
}

impl Extension<MyExtensionOptions> for MyExtension {
    // Use `ModuleImpl` when you want a Javascript module.
    // The options generic is not required.
    type Implementation = ModuleImpl<MyExtensionOptions>;

    fn implementation() -> &'static Self::Implementation {
        // This is the same as the implementation of `ModuleDef`
        &ModuleImpl {
            declare: |decl| {
                decl.declare("user")?;
                Ok(())
            },
            evaluate: |ctx, exports, options| {
                exports.export("user", options.user.clone())?;
                Ok(())
            },
            name: "my-module",
        }
    }

    fn options(self) -> MyExtensionOptions {
        self.options
    }

    fn globals(globals: &Object<'_>, options: &MyExtensionOptions) -> Result<()> {
        // Set your globals here
        globals.set("global_user", options.user.clone())?;
        Ok(())
    }
}
```

At runtime, this extension results in:

- A global variable named `global_user`
- An importable module `import { user } from "my-module"`

### Globals only

If you only need to set globals and do **NOT** want a Javascript module, use `GlobalsOnly` for the implementation.

```rust
use rquickjs::{Object, Result};
use rquickjs_extension::{Extension, GlobalsOnly};

struct MyExtension;

impl Extension for MyExtension {
    type Implementation = GlobalsOnly;

    fn implementation() -> &'static Self::Implementation {
        &GlobalsOnly
    }

    fn options(self) {}

    fn globals(globals: &Object<'_>, options: &()) -> Result<()> {
        // Set your globals here
        globals.set("hello", "world".to_string())?;
        Ok(())
    }
}
```

You can also use the macro for a simpler experience:

```rust
use rquickjs_module::globals_only;

struct MyExtension;
globals_only!(MyExtension, |globals| {
    // Set your globals here
    globals.set("hello", "world".to_string())?;
    Ok(())
});
```

At runtime, both result in a global variable named `hello`.
