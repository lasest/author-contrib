use dioxus::prelude::*;

#[component]
pub fn CreditDescription() -> Element {
    rsx! {
        p { class: "p-3",
            "The Contributor Roles Taxonomy (CRediT) is a standardized classification system comprising 14 distinct roles used to acknowledge and describe individual contributions to published research. This taxonomy provides a transparent framework for researchers to indicate their specific involvement in a project, enhancing clarity and accountability in scientific and academic publishing."
        }
        p { class: "p-3",
            "By categorizing contributions such as conceptualization, data curation, formal analysis, methodology, and writing, CRediT aims to promote fair and accurate attribution for all individuals involved in a research endeavor. It helps to clarify who did what in multi-author publications, offering a consistent approach to recognizing diverse intellectual efforts within research projects."
        }
    }
}
