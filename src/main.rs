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
    #[route("/OrderScreen")]
    OrderScreen,
    #[route("/Prep")]
    Prep,
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
    //         components::order::OrderScreen {}
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
            //     Link { to: Route::OrderScreen,
            //         "OrderScreen" }
            // }
            Link { class:"nav-item", to: Route::OrderScreen,
                style: if current_route.to_string() == "/OrderScreen" {
                    "background-color: #EBB7FF;"
                },
                // div { class: "nav-item", "OrderScreen"}
                "Order"
            }
            Link { class:"nav-item", to: Route::Prep,
                style: if current_route.to_string() == "/Prep" {
                    "background-color: #EBB7FF;"
                },
                // div { class: "nav-item", "Prep"}
                "Prep"
            }
            div { class: "nav-item", "Oven" }
            div { class: "nav-item", "Pickup" }
            div { class: "nav-item", "Delivery" }
            div { class: "nav-item", "Settings" }
        }
        Outlet::<Route> {}
    }
}
