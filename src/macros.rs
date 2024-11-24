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

            fn options(self) -> () {}
        }
    };
}
