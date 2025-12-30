use dioxus::prelude::*;

#[component]
pub fn Order() -> Element {
    rsx! {
        div { class: "order-container",
            div { class: "order-options",
                div { class: "sizes",
                    button { "Personal" }
                    button { "Small" }
                    button { "Large" }
                    button { "Sheet" }
                }
                div { class: "toppings",
                    button { "Pepperoni" }
                    button { "Onions" }
                    button { "Olives" }
                    button { "Spinach" }
                    for _ in 0..8 {
                        button { "blah" }
                    }
                }
            } // order options bracket
            div { class: "current-order-container",
                "Hello there! General Kenobi!"
            }
        }
    }
}
