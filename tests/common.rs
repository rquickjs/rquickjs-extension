use rquickjs::{class::Trace, JsLifetime};

#[derive(Clone, Trace, JsLifetime)]
#[rquickjs::class(frozen)]
pub struct Printer {
    target: String,
}

impl Printer {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

#[rquickjs::methods]
impl Printer {
    fn print(&self) -> String {
        format!("hello {}", self.target)
    }
}

#[derive(JsLifetime, Debug)]
pub struct PrinterOptions {
    pub target: String,
}
