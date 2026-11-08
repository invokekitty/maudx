use maud::{ComponentBody, PreEscaped, Render, html};
use maud_macros::write_html;


struct Project<'a> {
    id: &'a str,
    title: &'a str,
    desc: &'a str,
    deprecated: bool,
    body: ComponentBody<'a>
}

impl Render for Project<'_> {
    fn render_to(&self, buffer: &mut String) {
        write_html! { buffer =>
            #(self.id) .project .deprecated[self.deprecated] {
                h1 { (self.title) }
                p { (self.desc) }
                (self.body)
            }
        }
    }
}

#[test]
fn test_components() {
    let PreEscaped(html) = html! {
        h2 { "Projects" }
        Project #meow title="meow" deprecated=(false) desc="Eedges" {
            h1 { "eegs" }
        }
        Project id="eegs" title="uwu" deprecated=(true) desc="Eedges" {
            h2 { "eedges" }
            Project id="skeegs" title="meow" deprecated desc="arf" {
                "meow"
            }
        }
    };

    println!("{html}")
}
