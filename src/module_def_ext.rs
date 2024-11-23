use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    Ctx, JsLifetime, Object, Result,
};

pub trait ModuleDefExt<O = ()> {
    type Implementation: ModuleImplementationType<O>;

    fn globals(globals: &Object<'_>, options: &O) -> Result<()> {
        Ok(())
    }

    fn implementation() -> &'static Self::Implementation;

    fn options(self) -> O;
}

// Marker traits for implementation types
pub trait ModuleImplementationType<T> {}

pub struct GlobalsOnly;
impl<T> ModuleImplementationType<T> for GlobalsOnly {}

pub trait HasModule {
    fn name() -> &'static str;
    fn is_module() -> bool;
}

pub struct ModuleImpl<O = ()> {
    pub declare: fn(&Declarations) -> Result<()>,
    pub evaluate: for<'js> fn(&Ctx<'js>, &Exports<'js>, &O) -> Result<()>,
    pub name: &'static str,
}
impl<T> ModuleImplementationType<T> for ModuleImpl<T> {}

pub trait AsModule<O, R>
where
    Self: ModuleDefExt<O>,
    R: ModuleDef + HasModule,
    for<'js> O: JsLifetime<'js>,
{
    fn as_module(&self) -> R;
}

impl<T, O> AsModule<O, ModuleDefWrapper<T, O>> for T
where
    T: ModuleDefExt<O, Implementation = ModuleImpl<O>>,
    for<'js> O: JsLifetime<'js> + 'static,
{
    fn as_module(&self) -> ModuleDefWrapper<T, O> {
        ModuleDefWrapper {
            _marker: std::marker::PhantomData::<T>,
            _marker2: std::marker::PhantomData::<O>,
            name: T::implementation().name,
        }
    }
}

impl<T, O> AsModule<O, GlobalDefWrapper<T, O>> for T
where
    T: ModuleDefExt<O, Implementation = GlobalsOnly>,
    for<'js> O: JsLifetime<'js>,
{
    fn as_module(&self) -> GlobalDefWrapper<T, O> {
        GlobalDefWrapper {
            _marker: std::marker::PhantomData::<T>,
            _marker2: std::marker::PhantomData::<O>,
        }
    }
}

impl<T, O> HasModule for GlobalDefWrapper<T, O>
where
    T: ModuleDefExt<O, Implementation = GlobalsOnly>,
    for<'js> O: JsLifetime<'js>,
{
    fn name() -> &'static str {
        ""
    }

    fn is_module() -> bool {
        false
    }
}

impl<T, O> HasModule for ModuleDefWrapper<T, O>
where
    T: ModuleDefExt<O, Implementation = ModuleImpl<O>>,
    for<'c> O: JsLifetime<'c> + 'static,
{
    fn name() -> &'static str {
        T::implementation().name
    }

    fn is_module() -> bool {
        true
    }
}

impl<T, O> ModuleDef for ModuleDefWrapper<T, O>
where
    T: ModuleDefExt<O, Implementation = ModuleImpl<O>>,
    for<'js> O: JsLifetime<'js> + 'static,
{
    fn declare(decl: &Declarations) -> Result<()> {
        let module_impl = Self::implementation();
        (module_impl.declare)(decl)
    }

    fn evaluate<'a>(ctx: &Ctx<'a>, exports: &Exports<'a>) -> Result<()> {
        let module_impl = Self::implementation();
        let options = ctx.userdata::<O>().unwrap();
        (module_impl.evaluate)(ctx, exports, &options)
    }
}

// Wrapper implementation
pub struct ModuleDefWrapper<T, O>
where
    T: ModuleDefExt<O, Implementation = ModuleImpl<O>>,
    for<'js> O: JsLifetime<'js>,
{
    _marker: std::marker::PhantomData<T>,
    _marker2: std::marker::PhantomData<O>,
    pub name: &'static str,
}

impl<T, O> ModuleDefWrapper<T, O>
where
    T: ModuleDefExt<O, Implementation = ModuleImpl<O>>,
    for<'js> O: JsLifetime<'js>,
{
    fn implementation() -> &'static T::Implementation {
        T::implementation()
    }
}

pub struct GlobalDefWrapper<T, O>
where
    T: ModuleDefExt<O, Implementation = GlobalsOnly>,
    for<'js> O: JsLifetime<'js>,
{
    _marker: std::marker::PhantomData<T>,
    _marker2: std::marker::PhantomData<O>,
}

impl<T, O> ModuleDef for GlobalDefWrapper<T, O>
where
    T: ModuleDefExt<O, Implementation = GlobalsOnly>,
    for<'js> O: JsLifetime<'js>,
{
    //unused
    fn declare(decl: &Declarations) -> Result<()> {
        Ok(())
    }

    //unused
    fn evaluate<'a>(ctx: &Ctx<'a>, exports: &Exports<'a>) -> Result<()> {
        Ok(())
    }
}

#[macro_export]
macro_rules! globals_only_module {
    ($name:ident, $globals_impl:expr) => {
        impl ModuleDefExt for $name {
            type Implementation = GlobalsOnly;

            fn globals(globals: &Object<'_>, _options: &()) -> Result<()> {
                $globals_impl(globals)
            }

            fn implementation() -> &'static Self::Implementation {
                &GlobalsOnly
            }

            fn options(self) -> () {
                ()
            }
        }
    };
}
