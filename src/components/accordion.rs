use dioxus::prelude::*;

#[component]
pub fn AccordionItem(title: String, children: Element, is_open: bool) -> Element {
    let mut is_open = use_signal(|| is_open);

    rsx! {
        div { class: "border border-gray-500 rounded-md mb-2 overflow-hidden",
            button {
                class: "w-full flex items-center p-4 text-left bg-gray-800 hover:bg-gray-700 transition-colors duration-200",
                onclick: move |_| {
                    let is_open_copy = *is_open.read();
                    *is_open.write() = !is_open_copy;
                },
                role: "button",
                div { class: "mr-2 transform transition-transform duration-300",
                    if *is_open.read() {
                        "▼"
                    } else {
                        "►"
                    }
                }
                div { class: "", "{title}" }
            
            }

            div {
                class: "transition-all duration-300 overflow-hidden",
                hidden: !*is_open.read(),
                {children}
            }
        }
    }
}

#[component]
pub fn Accordion(children: Element) -> Element {
    rsx! {
        div { class: "accordion-container", role: "region", {children} }
    }
}
