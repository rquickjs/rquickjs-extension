#[macro_export]
macro_rules! globals_only_module {
    ($name:ident, |$globals:ident| { $($t:tt)* }) => {
        impl ModuleDefExt for $name {
            type Implementation = GlobalsOnly;

            fn globals(globals: &Object<'_>, _options: &()) -> Result<()> {
                (|$globals: &Object<'_>| {
                    $($t)*
                })(globals)
            }

            fn implementation() -> &'static Self::Implementation {
                &GlobalsOnly
            }

            fn options(self) -> () {}
        }
    };
}
