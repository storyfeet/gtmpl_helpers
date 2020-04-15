//! The point in this Library is to provide useful helper methods for cases where you really do
//! need the templates to be capable.
//! My personal use case was to be able to generate specific svg layouts from numbers in card
//! files. This meant doing more maths in the template than is common.
//! I've broken up the helpers roughly by subject so it should be easy to find the ones you need.
//!
//! The simplest way to use this is to import the trait and call with_all on a template.
//! ```rust
//! use gtmpl::{Template,Context};
//! use gtmpl_value::{Value,Number};
//!
//! use gtmpl_helpers::THelper;
//! let mut t = Template::default().with_all();
//! t.parse(r#"<rect {{xywh (mul . 5) (add . 11) 40 20 "px"}}/>"#);
//!
//! let s = t.q_render(4).unwrap();
//! assert_eq!(s,r#"<rect x="20px" y="15px" width="40px" height="20px" />"#.to_string())
//!
//! ```
//!
//! I wanted to keep the demo function simple, but q render works with anything that impls
//! Into for Value And these can of course be Gtmpl Derived like most things.
//! However that is tricky to demo in a doctest.
//!
//!

use gtmpl::{Context, Template};
use gtmpl_value::Value;

/// Functions like add and mul
pub mod math;

/// Functions that help with ranging
/// is_list checks you have a list returning bool
/// as_list creates a list from whatever you give it
/// safe_len returns a length for lists maps and strings, else 0
pub mod range;

/// Provides the ternary option
pub mod select;
pub mod string;
pub mod svg;

pub trait THelper: Sized {
    fn push_helper(self, fname: &str, f: fn(&[Value]) -> Result<Value, String>) -> Self;
    fn render_me(&self, c: &Context) -> Result<String, String>;

    fn q_render<V: Into<Value>>(&self, v: V) -> Result<String, String> {
        let c = Context::from(Value::from(v))?;
        self.render_me(&c)
    }

    //fn q_render<T:Into<Value>(&self)
    fn with_math(self) -> Self {
        self.push_helper("add", math::add)
            .push_helper("mul", math::mul)
    }

    fn with_range(self) -> Self {
        self.push_helper("is_list", range::is_list)
            .push_helper("as_list", range::as_list)
            .push_helper("safe_len", range::safe_len)
    }

    fn with_select(self) -> Self {
        self.push_helper("b_sel", select::b_sel)
            .push_helper("match", select::v_match)
    }

    fn with_string(self) -> Self {
        self.push_helper("ccat", string::ccat)
            .push_helper("sep", string::sep)
    }

    fn with_svg(self) -> Self {
        self.push_helper("xywh", svg::xywh)
            .push_helper("fl_stk", svg::fl_stk)
            .push_helper("font", svg::font)
            .push_helper("xml_es", svg::xml_es)
    }

    fn with_all(self) -> Self {
        self.with_string()
            .with_math()
            .with_range()
            .with_select()
            .with_svg()
    }
}

impl THelper for Template {
    fn push_helper(mut self, fname: &str, f: fn(&[Value]) -> Result<Value, String>) -> Self {
        self.add_func(fname, f);
        self
    }
    fn render_me(&self, c: &Context) -> Result<String, String> {
        self.render(c)
    }
}
