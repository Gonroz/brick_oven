use dioxus::prelude::*;

use crate::pizza;
use crate::server;

#[derive(Clone, Copy)]
struct OrderContext {
    pizza_count: Signal<i64>,
    pizzas: Signal<Vec<pizza::Pizza>>,
    active_pizza: Signal<Option<i64>>,
}

impl OrderContext {
    fn new() -> Self {
        OrderContext {
            pizza_count: Signal::new(0),
            pizzas: Signal::new(vec![]),
            active_pizza: Signal::new(None),
        }
    }
}

#[component]
pub fn Order() -> Element {
    use_context_provider(|| OrderContext::new());

    rsx! {
        div { class: "order-container",
            div { class: "order-options",
                div { class: "sizes",
                    SizeButton { pizza_size: pizza::PizzaSize::Personal }
                    SizeButton { pizza_size: pizza::PizzaSize::Small }
                    SizeButton { pizza_size: pizza::PizzaSize::Large }
                    SizeButton { pizza_size: pizza::PizzaSize::Sheet }
                }
                div { class: "toppings",
                    ToppingButton { topping_type: pizza::PizzaTopping::Pepperoni }
                    ToppingButton { topping_type: pizza::PizzaTopping::Onions }
                    ToppingButton { topping_type: pizza::PizzaTopping::Olives }
                    ToppingButton { topping_type: pizza::PizzaTopping::Spinach }
                    for _ in 0..8 {
                        button { "blah" }
                    }
                }
            } // order options bracket
            div { class: "current-order-container",
                CurrentOrder {}
                SendToKitchen {}
            }
        }
    }
}

#[component]
fn SizeButton(pizza_size: pizza::PizzaSize) -> Element {
    let mut order_context = use_context::<OrderContext>();
    let pizza_id = *order_context.pizza_count.read();

    rsx! {
        button {
            onclick: move |_| {
                order_context.pizzas.write().push(pizza::Pizza::new(pizza_id, pizza_size, vec![]));
                order_context.active_pizza.set(Some(pizza_id));
                order_context.pizza_count.set(pizza_id + 1);
            },
            "{pizza_size}"
        }
    }
}

#[component]
fn ToppingButton(topping_type: pizza::PizzaTopping) -> Element {
    let mut order_context = use_context::<OrderContext>();
    let active_pizza = *order_context.active_pizza.read();

    rsx! {
        button {
            onclick: move |_| {
                if active_pizza.is_some() {
                    for p in order_context.pizzas.write().iter_mut() {
                        if p.id == active_pizza.unwrap() {
                            p.toppings.push(topping_type);
                        }
                    }
                }
            },
            "{topping_type}"
        }
    }
}

#[component]
fn PizzaDiv(pizza: pizza::Pizza) -> Element {
    let mut order_context = use_context::<OrderContext>();
    let active_pizza = *order_context.active_pizza.read();

    let class = if active_pizza.is_some() && pizza.id == active_pizza.unwrap() {
        "pizza-div active"
    } else {
        "pizza-div"
    };

    rsx! {
        div {
            class: class,
            onclick: move |_| {
                order_context.active_pizza.set(Some(pizza.id));
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
            div {
                "ID: {pizza.id}"
            }
        }
    }
}

#[component]
fn CurrentOrder() -> Element {
    let order_context = use_context::<OrderContext>();

    rsx! {
        div { class: "current-order",
            for p in order_context.pizzas.read().iter() {
                PizzaDiv { pizza: p.clone() }
            }
        }
    }
}

#[component]
fn SendToKitchen() -> Element {
    let order_context = use_context::<OrderContext>();

    rsx! {
        button { class: "send-to-kitchen",
            onclick: move |_| async move {
                // _ = server::server_test("test".to_string()).await;
                _ = server::save_order(order_context.pizzas.read().clone()).await;
            },
            "Send to Kitchen"
        }
    }
}
