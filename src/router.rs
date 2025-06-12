use dioxus::prelude::*;

use crate::views::Home;
use crate::views::NotFound;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
