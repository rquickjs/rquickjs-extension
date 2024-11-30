use rquickjs::{
    module::{Declarations, Exports},
    Ctx, Object, Result,
};

/// A trait for defining Javascript module and globals in Rust
/// with options.
///
/// # Example
///
/// ```
/// use rquickjs::{Ctx, JsLifetime, Object, Result};
/// use rquickjs_extension::{Extension, ModuleImpl};
///
/// #[derive(JsLifetime, Debug)]
/// struct MyExtensionOptions {
///     user: String,
/// }
///
/// struct MyExtension {
///     options: MyExtensionOptions,
/// }
///
/// impl MyExtension {
///     pub fn new<T: Into<String>>(user: T) -> Self {
///         Self {
///             options: MyExtensionOptions {
///                 user: user.into(),
///             },
///         }
///     }
/// }
///
/// impl Extension<MyExtensionOptions> for MyExtension {
///     type Implementation = ModuleImpl<MyExtensionOptions>;
///
///     fn implementation() -> &'static Self::Implementation {
///         &ModuleImpl {
///             declare: |decl| {
///                 decl.declare("user")?;
///                 Ok(())
///             },
///             evaluate: |ctx, exports, options| {
///                 exports.export("user", options.user.clone())?;
///                 Ok(())
///             },
///             name: "my-module",
///         }
///     }
///
///     fn options(self) -> MyExtensionOptions {
///         self.options
///     }
///
///     fn globals(globals: &Object<'_>, options: &MyExtensionOptions) -> Result<()> {
///         globals.set("user", options.user.clone())?;
///         Ok(())
///     }
/// }
/// ```
pub trait Extension<O = ()> {
    type Implementation: ModuleImplementationType<O>;

    fn globals(_globals: &Object<'_>, _options: &O) -> Result<()> {
        Ok(())
    }

    fn implementation() -> &'static Self::Implementation;

    fn options(self) -> O;
}

/// Marker trait for implementation types
pub trait ModuleImplementationType<T> {}

/// Implementation type when you only need to define globals
pub struct GlobalsOnly;
impl<T> ModuleImplementationType<T> for GlobalsOnly {}

/// Implementation type when you need to define a module and
/// optionally globals.
pub struct ModuleImpl<O = ()> {
    pub declare: fn(&Declarations) -> Result<()>,
    pub evaluate: for<'js> fn(&Ctx<'js>, &Exports<'js>, &O) -> Result<()>,
    pub name: &'static str,
}
impl<T> ModuleImplementationType<T> for ModuleImpl<T> {}
