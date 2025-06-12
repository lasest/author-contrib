use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx!(
        h1 { "Error 404: Page not Found" }
    )
}
