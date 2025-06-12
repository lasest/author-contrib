use dioxus::prelude::*;

use crate::components::AuthorTable;
use crate::components::CreditDescription;
use crate::components::CreditRoles;
use crate::components::{Accordion, AccordionItem};

#[component]
pub fn Home() -> Element {
    let credit_roles = rsx! {
        CreditRoles {}
    };

    let credit_description = rsx! {
        CreditDescription {}
    };

    let children = rsx! {
        AccordionItem {
            title: "What is CRediT?",
            children: credit_description,
            is_open: false,
        }
        AccordionItem { title: "CRediT roles", children: credit_roles, is_open: false }
    };

    rsx! {
        div { class: "mb-8", AuthorTable {} }
        Accordion { children }
    }
}
