use dioxus::prelude::*;

#[component]
pub fn GoogleAnalytics() -> Element {
    let script_body = r#"
        window.dataLayer = window.dataLayer || [];
        function gtag(){dataLayer.push(arguments);}
        gtag('js', new Date());

        gtag('config', 'G-0EF0NEEH1Y');
    "#;

    rsx! {
        script {
            r#async: true,
            src: "https://www.googletagmanager.com/gtag/js?id=G-0EF0NEEH1Y",
        }
        script { dangerous_inner_html: script_body }
    }
}
