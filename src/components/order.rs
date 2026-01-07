use core::fmt;

use dioxus::prelude::*;

#[component]
pub fn Order() -> Element {
    let mut pizzas: Signal<Vec<Pizza>> = use_signal(|| vec![]);

    rsx! {
        div { class: "order-container",
            div { class: "order-options",
                div { class: "sizes",
                    button {
                        onclick: move |_| {
                            // blah
                            pizzas.write().push(Pizza { size: PizzaSize::Personal, topping: PizzaTopping::Onion });
                        },
                        "Personal" }
                    button { "Small" }
                    button { "Large" }
                    button { "Sheet" }
                }
                div { class: "toppings",
                    button {
                        // onclick: move |_| {
                        //     let _ = Pizza();
                        // },
                        "Pepperoni"
                    }
                    button { "Onions" }
                    button { "Olives" }
                    button { "Spinach" }
                    for _ in 0..8 {
                        button { "blah" }
                    }
                }
            } // order options bracket
            div { class: "current-order-container",
                CurrentOrder { pizzas: pizzas }
                button { class: "send-to-kitchen",
                    "Send to Kitchen"
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum PizzaSize {
    Personal,
    Small,
    Large,
}

impl fmt::Display for PizzaSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PizzaSize::Personal => write!(f, "Personal"),
            PizzaSize::Small => write!(f, "Small"),
            PizzaSize::Large => write!(f, "Large"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum PizzaTopping {
    Pepperoni,
    Onion,
}

impl fmt::Display for PizzaTopping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PizzaTopping::Pepperoni => write!(f, "Pepperoni"),
            PizzaTopping::Onion => write!(f, "Onion"),
        }
    }
}

#[component]
pub fn PizzaDiv(size: PizzaSize, topping: PizzaTopping) -> Element {
    let mut size = use_signal(|| PizzaSize::Small);
    let mut topping = use_signal(|| PizzaTopping::Pepperoni);

    rsx! {
        div {
            "Size: {size} -- Toppings: {topping}"
        }
    }
}

struct Pizza {
    size: PizzaSize,
    topping: PizzaTopping,
}

#[component]
pub fn CurrentOrder(mut pizzas: Signal<Vec<Pizza>>) -> Element {
    // let mut pizzas: Signal<Vec<Pizza>> = use_signal(|| vec![]);
    // pizzas.push(Pizza {
    //     size: PizzaSize::Small,
    //     topping: PizzaTopping::Pepperoni,
    // });
    // pizzas.push(Pizza {
    //     size: PizzaSize::Large,
    //     topping: PizzaTopping::Onion,
    // });

    // pizzas.write().push(Pizza {
    //     size: PizzaSize::Large,
    //     topping: PizzaTopping::Pepperoni,
    // });

    rsx! {
        div { class: "current-order",
            for p in pizzas.iter() {
                PizzaDiv { size: p.size, topping: p.topping }
            }
            // "argh"
        }
    }
}
