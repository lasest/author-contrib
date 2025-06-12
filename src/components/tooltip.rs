use dioxus::prelude::*;

#[derive(PartialEq, Clone, Default)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    #[allow(unused)]
    Left,
    #[allow(unused)]
    Right,
}

#[component]
pub fn Tooltip(
    content: Element,
    children: Element,
    #[props(default)] position: TooltipPosition,
    #[props(default = "max-w-xs".into())] width_class: String,
) -> Element {
    let mut show = use_signal(|| false);

    // Base classes for tooltip container
    let container_classes = "relative inline-block".to_string();

    // Classes for tooltip content (common for all positions)
    let mut tooltip_classes =
        "absolute px-3 py-2 bg-gray-800 border border-gray-300 text-white text-sm rounded shadow-lg z-50 \
                              transform transition-opacity duration-150 pointer-events-none "
            .to_string();

    tooltip_classes += &width_class;

    // Classes for tooltip arrow (common base)
    let mut arrow_classes =
        "border-t border-l border-gray-300 absolute w-2 h-2 bg-gray-800 transform rotate-45"
            .to_string();

    // Position-specific adjustments
    match position {
        TooltipPosition::Top => {
            tooltip_classes += " bottom-full left-1/2 -translate-x-1/2 mb-2";
            arrow_classes += " top-full left-1/2 -translate-x-1/2 -translate-y-1/2";
        }
        TooltipPosition::Bottom => {
            tooltip_classes += " top-full left-1/2 -translate-x-1/2 mt-2";
            arrow_classes += " bottom-full left-1/2 -translate-x-1/2 translate-y-1/2";
        }
        TooltipPosition::Left => {
            tooltip_classes += " right-full top-1/2 -translate-y-1/2 mr-2";
            arrow_classes += " left-full top-1/2 -translate-y-1/2 -translate-x-1/2";
        }
        TooltipPosition::Right => {
            tooltip_classes += " left-full top-1/2 -translate-y-1/2 ml-2";
            arrow_classes += " right-full top-1/2 -translate-y-1/2 translate-x-1/2";
        }
    }

    rsx! {
        div {
            class: "{container_classes}",
            onmouseenter: move |_| show.set(true),
            onmouseleave: move |_| show.set(false),
            // Wrap children in span for stable layout
            span { class: "inline-flex", {children} }
            // Render tooltip when visible
            if *show.read() {
                div { class: "{tooltip_classes}",
                    // Tooltip content
                    div { class: "relative z-10", {content} }
                    // Arrow element
                    div { class: "{arrow_classes}" }
                }
            }
        }
    }
}
