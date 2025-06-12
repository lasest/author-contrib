use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::models::roles::Roles;

#[component]
pub fn CreditRoles() -> Element {
    rsx! {
        table { class: "m-3",
            thead {
                tr { class: "border-b border-gray-600",
                    th { "Role" }
                    th { "Description" }
                }
            

            }
            tbody {
                for role in Roles::iter() {
                    tr { class: "border-b border-gray-600",
                        td { class: "p-1", "{role.to_string()}" }
                        td { class: "p-1", "{role.description()}" }
                    }
                }
            
            }
        }
    }
}
