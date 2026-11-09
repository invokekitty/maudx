#![cfg(feature = "components")]
use maud::{PreEscaped, Render, components::{Body, Component, MaybeBody}, html, attrs};
use maud_macros::write_html;


#[derive(Default)]
struct Project<'a> {
    id: &'a str,
    title: &'a str,
    desc: &'a str,
    deprecated: bool
}

impl<'a> Component for Project<'a> {
    type Body = MaybeBody<'a>;
    fn render_to(self, body: MaybeBody<'a>, buffer: &mut String) {
        write_html! { buffer =>
            #(self.id) .project .deprecated[self.deprecated] {
                h1 { (self.title) }
                p { (self.desc) }
                @if let Some(body) = body {
                    span { (body) }
                }
            }
        }
    }
}

#[test]
fn test_components() {
    let eegs = attrs!(eegs = "meow" eedges = "skeeg");
    
    let PreEscaped(html) = html! {
        h2 ..(eegs) { "Projects" }
        Project #meow title="meow" .. {
            h1 { "eegs" }
        }
        self::Project id="eegs" title="uwu" deprecated=(true) desc="Eedges" {
            h2 { "eedges" }
            Project id="skeegs" title="meow" deprecated desc="arf";
        }
    };

    println!("{html}")
}
