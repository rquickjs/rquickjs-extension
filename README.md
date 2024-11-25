# rquickjs module

[![github](https://img.shields.io/badge/github-rquickjs/rquickjs-module.svg?style=for-the-badge&logo=github)](https://github.com/rquickjs/rquickjs-module)
[![crates](https://img.shields.io/crates/v/rquickjs-module.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/rquickjs-module)

This is an extension to [rquickjs](https://github.com/DelSkayn/rquickjs) to allow the ecosystem to create more unified Rust modules.

The goal was to create a better version of [`ModuleDef`](https://docs.rs/rquickjs/latest/rquickjs/module/trait.ModuleDef.html) that would allow it to have options as input and set global.

For example, a `fetch` module using `ModuleDefExt` could set a global `fetch` function and have as options an allowlist of domains.

## Using modules

If you are a consumer of modules create using that crate, here is how you can import them in your runtime.

```rust
use rquickjs::AsyncRuntime;
use rquickjs_module::ModuleLoader;

#[tokio::main]
async fn main() {
    let rt = AsyncRuntime::new().unwrap();

    let (loader, resolver, initalizer) = ModuleLoader::builder().with_module(MyModule).build();

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

## Creating modules

For the base case, replace the `ModuleDef` by an implementation of `ModuleDefExt`.

```rust
use rquickjs::{Ctx, JsLifetime, Object, Result};
use rquickjs_module::{ModuleDefExt, ModuleImpl};

#[derive(JsLifetime, Debug)]
struct MyModuleOptions {
    user: String,
}

struct MyModule {
    options: MyModuleOptions,
}

impl MyModule {
    pub fn new<T: Into<String>>(user: T) -> Self {
        Self {
            options: MyModuleOptions {
                user: user.into(),
            },
        }
    }
}

impl ModuleDefExt<MyModuleOptions> for MyModule {
    // Use `ModuleImpl` when you want a Javascript module.
    // The options generic is not required.
    type Implementation = ModuleImpl<MyModuleOptions>;

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

    fn options(self) -> MyModuleOptions {
        self.options
    }

    fn globals(globals: &Object<'_>, options: &MyModuleOptions) -> Result<()> {
        // Set your globals here
        globals.set("global_user", options.user.clone())?;
        Ok(())
    }
}
```

At runtime, this module results in:

- A global `global_user` variable
- An importable module `import { user } from "my-module"`

### Globals only

If you only need to set globals and do **NOT** want an actual module that the Javascript can import, use `GlobalsOnly` for the implementation.

```rust
use rquickjs::{Object, Result};
use rquickjs_module::{ModuleDefExt, GlobalsOnly};

struct MyModule;

impl ModuleDefExt for MyModule {
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
use rquickjs_module::globals_only_module;

struct MyModule;
globals_only_module!(MyModule, |globals| {
    // Set your globals here
    globals.set("hello", "world".to_string())?;
    Ok(())
});
```

Both result in a global variable named `hello` begin available at runtime.
