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
                            pizzas.write().push(Pizza { size: PizzaSize::Personal, toppings: None });
                        },
                        "Personal" }
                    button {
                        onclick: move |_| {
                            // blah
                            pizzas.write().push(Pizza { size: PizzaSize::Small, toppings: None });
                        },
                        "Small" }
                    button {
                        onclick: move |_| {
                            // blah
                            pizzas.write().push(Pizza { size: PizzaSize::Large, toppings: None });
                        },
                        "Large" }
                    button {
                        onclick: move |_| {
                            // blah
                            pizzas.write().push(Pizza { size: PizzaSize::Sheet, toppings: None });
                        },
                        "Sheet" }
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
enum PizzaSize {
    Personal,
    Small,
    Large,
    Sheet,
}

impl fmt::Display for PizzaSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PizzaSize::Personal => write!(f, "Personal"),
            PizzaSize::Small => write!(f, "Small"),
            PizzaSize::Large => write!(f, "Large"),
            PizzaSize::Sheet => write!(f, "Sheet"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum PizzaTopping {
    Pepperoni,
    Onions,
    Olives,
    Spinach,
}

impl fmt::Display for PizzaTopping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PizzaTopping::Pepperoni => write!(f, "Pepperoni"),
            PizzaTopping::Onions => write!(f, "Onions"),
            PizzaTopping::Olives => write!(f, "Olives"),
            PizzaTopping::Spinach => write!(f, "Spinach"),
        }
    }
}

#[component]
fn PizzaDiv(pizza: Pizza) -> Element {
    let size = use_signal(|| pizza.size);
    let signal_toppings = use_signal(|| pizza.toppings);

    rsx! {
        if let Some(toppings) = *signal_toppings.read() {
            div {
                "Size: {size} -- Toppings: {toppings}"
            }
        } else {
            div {
                "Size: {size} -- Toppings: None"
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
struct Pizza {
    size: PizzaSize,
    toppings: Option<PizzaTopping>,
}

#[component]
fn CurrentOrder(mut pizzas: Signal<Vec<Pizza>>) -> Element {
    rsx! {
        div { class: "current-order",
            for p in pizzas.iter() {
                PizzaDiv { pizza: p.clone() }
            }
            // "argh"
        }
    }
}
