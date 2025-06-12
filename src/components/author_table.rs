use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsQuestionCircle;
use dioxus_free_icons::Icon;
use strum::IntoEnumIterator;

use crate::backend::analytics;
use crate::components::AuthorRow;
use crate::components::{Tooltip, TooltipPosition};
use crate::models::author::{render_credit_statement, Author};
use crate::models::credit_format::CreditFormat;
use crate::models::roles::Roles;

#[component]
pub fn AuthorTable() -> Element {
    let mut authors = use_signal(|| vec![Author::default()]);

    let handle_row_change = move |(index, new_data): (usize, Option<Author>)| {
        match new_data {
            Some(value) => authors.write()[index] = value,
            None => {
                authors.write().remove(index);
            }
        };
    };

    let add_author = move |_| async move {
        authors.write().push(Author::default());
        _ = analytics::analytics_author_added().await;
    };

    let mut credit_format = use_signal(|| CreditFormat::Long);

    let credit_statement =
        use_memo(move || render_credit_statement(authors.read().to_vec(), &credit_format.read()));

    let tooltip_content = rsx!(
        div {
            p { "Name format:" }
            p { "First name [Middle name] Last name" }
            p { "Note: Middle name is optional" }
        }
    );

    rsx! {
        div {
            table {
                thead {
                    th { class: "align-bottom",
                        "Author"
                        Tooltip {
                            content: tooltip_content,
                            position: TooltipPosition::Bottom,
                            width_class: "w-80",
                            Icon {
                                class: "mx-2",
                                width: 16,
                                height: 16,
                                fill: "white",
                                icon: BsQuestionCircle,
                            }
                        }
                    }
                    for role in Roles::iter() {
                        th { class: "vertical-text", "{role.to_string()}" }
                    }
                }
                tbody {
                    for (index , author) in authors.read().iter().enumerate() {
                        AuthorRow {
                            index,
                            author: author.clone(),
                            on_change: handle_row_change,
                        }
                    }
                }
            }
            button {
                class: "bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-3 my-2 rounded-lg shadow-md hover:shadow-lg transition-all duration-200 ease-in-out",
                onclick: add_author,
                "Add Row"
            }
            div { class: "mt-6",
                label { class: "mx-2", "Format:" }
                select {
                    class: "px-2 py-1 rounded shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-gray-800 text-gray-100",
                    onchange: {
                        move |e: FormEvent| {
                            let selected_value = e.value();
                            *credit_format.write() = match selected_value.as_str() {
                                "Full names" => CreditFormat::Long,
                                "Short names" => CreditFormat::Short,
                                _ => CreditFormat::Short,
                            };
                        }
                    },
                    option { class: "bg-white text-gray-800 dark:bg-gray-800 dark:text-white",
                        "Full names"
                    }
                    option { class: "bg-white text-gray-800 dark:bg-gray-800 dark:text-white",
                        "Short names"
                    }
                }
            }
            div { class: "flex items-baseline gap-4 my-4",
                label { class: "text-sm font-semibold text-gray-100", "CRediT statement" }
                pre { class: "bg-gray-700 border border-gray-300 font-semibold text-sm text-gray-100 p-4 rounded-lg whitespace-pre-wrap font-mono shadow-smr",
                    "{credit_statement}"
                }
            }
        }
    }
}
