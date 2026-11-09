use alloc::string::String;

use crate::{Markup, Render};


pub trait Component: Sized {
    type Body: ComponentBody;
    fn render_to(self, body: Self::Body, buffer: &mut String);
}


trait Sealed {}
#[allow(private_bounds)]
pub trait ComponentBody: Render + Sealed {}

pub struct NoBody;
impl ComponentBody for NoBody {}
impl Sealed for NoBody {}
impl Render for NoBody {
    fn render_to(&self, _buffer: &mut String) {}
}

pub enum Body<'a> {
    Static(&'static str),
    Dynamic(&'a dyn Fn(&mut String))
}

impl ComponentBody for Body<'_> {}
impl Sealed for Body<'_> {}
impl Render for Body<'_> {
    fn render_to(&self, buffer: &mut String) {
        match self {
            Body::Static(str) => buffer.push_str(str),
            Body::Dynamic(body) => body(buffer)
        }
    }
}

pub type MaybeBody<'a> = Option<Body<'a>>;

impl ComponentBody for MaybeBody<'_> {}
impl Sealed for MaybeBody<'_> {}
impl Render for MaybeBody<'_> {
    fn render_to(&self, buffer: &mut String) {
        if let Some(block) = self {
            block.render_to(buffer);
        }
    }
}

impl From<NoBody> for MaybeBody<'static> {
    fn from(_value: NoBody) -> Self {
        None
    }
}