#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-preview";

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME", indent_children = false))]
#[cfg_attr(feature = "parse", derive(mrml_parse_macros::MrmlParseComponent))]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MJPreview {
    pub children: String,
}

impl MJPreview {
    pub fn content(&self) -> &str {
        &self.children
    }
}

impl From<String> for MJPreview {
    fn from(children: String) -> Self {
        Self { children }
    }
}

impl From<&str> for MJPreview {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
