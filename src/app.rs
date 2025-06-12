use dioxus::prelude::*;

use crate::components::GoogleAnalytics;
use crate::components::HeadElements;
use crate::router::Route;

#[component]
pub fn App() -> Element {
    rsx! {
        HeadElements {}
        GoogleAnalytics {}

        Router::<Route> {}
    }
}
