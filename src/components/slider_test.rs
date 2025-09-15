use dioxus::prelude::*;

#[component]
pub fn SliderTest() -> Element {
    rsx! {
        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            // Attributes should be defined in the element before any children
            id: "hero",
            // After all attributes are defined, we can define child elements and components
            h1 { "A first test by Sam Jaques" }
            div { id: "links",
                a { href: "https://bitbucket.org", "A link to Bitbucket" }
            }
            div {
                label { "Adjust the slider:" }
                input {
                    r#type: "range",
                    min: "0",
                    max: "100",
                    value: "50",
                    id: "slider",
                }
            }
            output { "Current value: 50" }
        }
    }
}
