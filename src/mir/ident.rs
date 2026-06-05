use rnix::ast;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Ident {
    value: String,
    // span: TextRange,
}

impl Ident {
    pub fn new(value: String) -> Self {
        Ident { value }
    }
}

impl From<ast::Ident> for Ident {
    fn from(value: ast::Ident) -> Self {
        Self {
            value: value.to_string(),
            // span: value.syntax().text_range(),
        }
    }
}

impl From<Ident> for String {
    fn from(value: Ident) -> Self {
        value.value
    }
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        &self.value
    }
}
