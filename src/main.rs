use dioxus::prelude::*;

mod components;

use crate::components::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    RenderArea,
    #[route("/Order")]
    Order,
}

#[component]
fn App() -> Element {
    rsx! {
        div { class: "dashboard-container",
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            // NavBar {}
            // RenderArea {}
            Router::<Route> {}
        }
    }
}

#[component]
pub fn RenderArea() -> Element {
    // rsx! {
    //     div { class: "renderarea",
    //         components::order::Order {}
    //     }
    // }
    rsx! {
        div { class: "renderarea",
            "RenderArea"
        }
    }
}

#[component]
pub fn NavBar() -> Element {
    let current_route: Route = use_route();
    // println!("Current route: {}", current_route.to_string());

    rsx! {
        div { class: "navbar",
            // div { class: "nav-item",
            //     Link { to: Route::Order,
            //         "Order" }
            // }
            Link { class:"nav-item", to: Route::Order,
                style: if current_route.to_string() == "/Order" {
                    "background-color: #EBB7FF; transition: background-color .1s ease-in-out"
                },
                // div { class: "nav-item", "Order"}
                "Order"
            }
            div { class: "nav-item", "Prep" }
            div { class: "nav-item", "Oven" }
            div { class: "nav-item", "Pickup" }
            div { class: "nav-item", "Delivery" }
            div { class: "nav-item", "Settings" }
        }
        Outlet::<Route> {}
    }
}
