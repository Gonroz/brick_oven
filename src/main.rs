use dioxus::prelude::*;

mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    RenderArea,
}

#[component]
fn App() -> Element {
    rsx! {
        div { class: "dashboard-container",
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            NavBar {}
            RenderArea {}
        }
    }
}

#[component]
pub fn RenderArea() -> Element {
    rsx! {
        div { class: "renderarea",
            components::order::Order {}
        }
    }
}

#[component]
pub fn NavBar() -> Element {
    rsx! {
        // div { class: "container",
        div { class: "navbar",
            ul {
                li { class: "nav-item", "Order" }
                li { class: "nav-item", "Prep" }
                li { class: "nav-item", "Oven" }
                li { class: "nav-item", "Pickup" }
                li { class: "nav-item", "Delivery" }
                li { class: "nav-item", "Settings" }
            }
        }
        // }
    }
}
