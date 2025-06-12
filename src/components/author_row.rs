use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsXCircle;
use dioxus_free_icons::Icon;
use strum::IntoEnumIterator;

use crate::models::author::Author;
use crate::models::roles::Roles;

#[component]
pub fn AuthorRow(
    index: usize,
    author: Author,
    on_change: EventHandler<(usize, Option<Author>)>,
) -> Element {
    rsx! {
        tr {
            td { class: "py-2 pr-2",
                input {
                    r#type: "text",
                    class: "w-80 bg-gray-800 text-white placeholder-gray-400 border border-gray-700 focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none rounded px-1 py-1 transition duration-200",
                    value: "{author.name}",
                    placeholder: "First name [Middle name] Last name",
                    oninput: {
                        let author = author.clone();
                        move |e: FormEvent| {
                            let mut new_author = author.clone();
                            new_author.name = e.value().clone();
                            on_change.call((index, Some(new_author)));
                        }
                    },
                }
            }
            for role in Roles::iter() {
                td { class: "p-1",
                    input {
                        r#type: "checkbox",
                        class: "h-5 w-5 text-blue-600 bg-white border-gray-300 rounded focus:ring-2 focus:ring-blue-500 hover:shadow-lg transition-all duration-200 ease-in-out",
                        checked: false,
                        onchange: {
                            let author = author.clone();
                            move |e: FormEvent| {
                                let mut new_author = author.clone();
                                new_author.update_role(&role, e.checked());
                                on_change.call((index, Some(new_author)));
                            }
                        },
                    }
                }
            }
            td { class: "px-1",
                button {
                    class: "bg-red-700 rounded px-1.5 py-1.5 text-gray-200 hover:bg-red-800",
                    onclick: {
                        move |_| {
                            on_change.call((index, None));
                        }
                    },
                    Icon {
                        width: 16,
                        height: 16,
                        fill: "white",
                        icon: BsXCircle,
                    }
                }
            }
        }
    }
}
