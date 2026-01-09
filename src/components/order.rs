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
                            pizzas.write().push(Pizza { id: pizza_count(), size: PizzaSize::Personal, toppings: vec![] });
                            active_pizza.set(Some(pizza_count()));
                            pizza_count.set(pizza_count() + 1);
                        },
                        "Personal" }
                    button {
                        onclick: move |_| {
                            pizzas.write().push(Pizza { id: pizza_count(), size: PizzaSize::Small, toppings: vec![] });
                            active_pizza.set(Some(pizza_count()));
                            pizza_count.set(pizza_count() + 1);
                        },
                        "Small" }
                    button {
                        onclick: move |_| {
                            pizzas.write().push(Pizza { id: pizza_count(), size: PizzaSize::Large, toppings: vec![] });
                            active_pizza.set(Some(pizza_count()));
                            pizza_count.set(pizza_count() + 1);
                        },
                        "Large" }
                    button {
                        onclick: move |_| {
                            pizzas.write().push(Pizza { id: pizza_count(), size: PizzaSize::Sheet, toppings: vec![] });
                            active_pizza.set(Some(pizza_count()));
                            pizza_count.set(pizza_count() + 1);
                        },
                        "Sheet" }
                }
                div { class: "toppings",
                    button {
                        onclick: move |_| {
                            if active_pizza().is_some() {
                                let mut p = pizzas.write();
                                for a in p.iter_mut() {
                                    if a.id == active_pizza.unwrap() {
                                        a.toppings.push(PizzaTopping::Pepperoni);
                                    }
                                }
                            }
                        },
                        "Pepperoni"
                    }
                    button {
                        onclick: move |_| {
                            if active_pizza().is_some() {
                                let mut p = pizzas.write();
                                for a in p.iter_mut() {
                                    if a.id == active_pizza.unwrap() {
                                        a.toppings.push(PizzaTopping::Onions);
                                    }
                                }
                            }
                        },
                        "Onions"
                    }
                    button {
                        onclick: move |_| {
                            if active_pizza().is_some() {
                                let mut p = pizzas.write();
                                for a in p.iter_mut() {
                                    if a.id == active_pizza.unwrap() {
                                        a.toppings.push(PizzaTopping::Olives);
                                    }
                                }
                            }
                        },
                        "Olives"
                    }
                    button {
                        onclick: move |_| {
                            if active_pizza().is_some() {
                                let mut p = pizzas.write();
                                for a in p.iter_mut() {
                                    if a.id == active_pizza.unwrap() {
                                        a.toppings.push(PizzaTopping::Spinach);
                                    }
                                }
                            }
                        },
                        "Spinach"
                    }
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
    let class = if active_pizza.read().is_some() && pizza.id == active_pizza.read().unwrap() {
        "pizza-div active"
    } else {
        "pizza-div"
    };

    rsx! {
        div {
            class: class,
            onclick: move |_| {
                active_pizza.set(Some(pizza.id));
            },
            div {
                "Size: {pizza.size}"
            }
            ul {
                for topping in pizza.toppings.iter() {
                    li {
                        "{topping}"
                    }
                }
            }
            // for topping in pizza.toppings.iter() {
            //     div {
            //         "{topping}"
            //     }
            // }
            div {
                "ID: {pizza.id}"
            }
        }
    }
}

#[derive(PartialEq, Clone)]
struct Pizza {
    id: u64,
    size: PizzaSize,
    toppings: Vec<PizzaTopping>,
}

#[component]
fn CurrentOrder(mut pizzas: Signal<Vec<Pizza>>, active_pizza: Signal<Option<u64>>) -> Element {
    rsx! {
        div { class: "current-order",
            for p in pizzas.read().iter() {
                PizzaDiv { pizza: p.clone(), active_pizza: active_pizza }
            }
        }
    }
}
