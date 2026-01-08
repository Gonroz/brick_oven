use core::fmt;

use dioxus::prelude::*;

#[component]
pub fn Order() -> Element {
    let mut pizza_count = use_signal::<u64>(|| 0);
    let mut pizzas: Signal<Vec<Pizza>> = use_signal(|| vec![]);
    let mut active_pizza = use_signal::<Option<u64>>(|| None);

    rsx! {
        div { class: "order-container",
            div { class: "order-options",
                div { class: "sizes",
                    button {
                        onclick: move |_| {
                            pizzas.write().push(Pizza { id: pizza_count(), size: PizzaSize::Personal, toppings: None });
                            // active_pizza.set(Some(pizza_count()));
                            // println!("first {}", pizza_count());
                            active_pizza.set(Some(pizza_count()));
                            // println!("ap: {}", active_pizza().unwrap());
                            pizza_count.set(pizza_count() + 1);
                            // println!("second {}", pizza_count());
                        },
                        "Personal" }
                    button {
                        onclick: move |_| {
                            pizzas.write().push(Pizza { id: pizza_count(), size: PizzaSize::Small, toppings: None });
                            active_pizza.set(Some(pizza_count()));
                            pizza_count.set(pizza_count() + 1);
                        },
                        "Small" }
                    button {
                        onclick: move |_| {
                            pizzas.write().push(Pizza { id: pizza_count(), size: PizzaSize::Large, toppings: None });
                            active_pizza.set(Some(pizza_count()));
                            pizza_count.set(pizza_count() + 1);
                        },
                        "Large" }
                    button {
                        onclick: move |_| {
                            pizzas.write().push(Pizza { id: pizza_count(), size: PizzaSize::Sheet, toppings: None });
                            active_pizza.set(Some(pizza_count()));
                            pizza_count.set(pizza_count() + 1);
                        },
                        "Sheet" }
                }
                div { class: "toppings",
                    button {
                        onclick: move |_| {
                            println!("Pepperoni pressed -- is some?: {}", active_pizza().is_some());
                            if active_pizza().is_some() {
                                let mut p = pizzas.write();
                                for a in p.iter_mut() {
                                    if a.id == active_pizza.unwrap() {
                                        a.toppings = Some(PizzaTopping::Pepperoni);
                                        println!("{}", a.toppings.unwrap());
                                    }
                                }
                            }
                        },
                        "Pepperoni"
                    }
                    button {
                        onclick: move |_| {
                            println!("Onions pressed -- is some?: {}", active_pizza().is_some());
                            if active_pizza().is_some() {
                                let mut p = pizzas.write();
                                for a in p.iter_mut() {
                                    if a.id == active_pizza.unwrap() {
                                        a.toppings = Some(PizzaTopping::Onions);
                                        println!("{}", a.toppings.unwrap());
                                    }
                                }
                            }
                        },
                        "Onions"
                    }
                    button { "Olives" }
                    button { "Spinach" }
                    for _ in 0..8 {
                        button { "blah" }
                    }
                }
            } // order options bracket
            div { class: "current-order-container",
                CurrentOrder { pizzas: pizzas, active_pizza: active_pizza }
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

// This is required in order to actually display the values
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

// This is required in order to actually display the values
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
fn PizzaDiv(pizza: Pizza, active_pizza: Signal<Option<u64>>) -> Element {
    rsx! {
        if !pizza.toppings.is_none() {
            div {
                onclick: move |_| {
                    active_pizza.set(Some(pizza.id));
                },
                "Size: {pizza.size} -- Toppings: {pizza.toppings.unwrap()} -- ID: {pizza.id}"
            }
        } else {
            div {
                onclick: move |_| {
                    active_pizza.set(Some(pizza.id));
                },
                "Size: {pizza.size} -- Toppings: None -- ID: {pizza.id}"
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
struct Pizza {
    id: u64,
    size: PizzaSize,
    toppings: Option<PizzaTopping>,
}

#[component]
fn CurrentOrder(mut pizzas: Signal<Vec<Pizza>>, active_pizza: Signal<Option<u64>>) -> Element {
    rsx! {
        div { class: "current-order",
            for p in pizzas.read().iter() {
                PizzaDiv { pizza: *p, active_pizza: active_pizza }
            }
            // div {
            //     if active_pizza.read().is_some() {
            //         "Active: {active_pizza.unwrap().size}"
            //         PizzaDiv { pizza: active_pizza.unwrap() }
            //         // PizzaDiv { pizza: active_pizza.unwrap().clone() }
            //     } else {
            //         div {
            //             "no active pizza"
            //         }
            //     }
            // }
        }
    }
}
