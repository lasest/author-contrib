use dioxus::prelude::*;

use crate::assets::{FAVICON, MAIN_CSS, TAILWIND_CSS};

#[component]
pub fn HeadElements() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Meta {
            name: "description",
            content: "Generate journal-ready CRediT author contribution statements instantly. Free tool for researchers & academics. Save time with compliant role taxonomy for scientific publications.",
        }
        document::Meta {
            name: "keywords",
            content: "CRediT generator, author contribution tool, CRediT taxonomy, research contribution statement, scientific publishing, academic authorship, co-author roles, journal submission",
        }
        document::Meta { name: "robots", content: "index, follow" }
    }
}
