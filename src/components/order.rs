use core::fmt;

use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct OrderContext {
    pizza_count: Signal<u64>,
    pizzas: Signal<Vec<Pizza>>,
    active_pizza: Signal<Option<u64>>,
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
                    SizeButton { pizza_size: PizzaSize::Personal }
                    SizeButton { pizza_size: PizzaSize::Small }
                    SizeButton { pizza_size: PizzaSize::Large }
                    SizeButton { pizza_size: PizzaSize::Sheet }
                }
                div { class: "toppings",
                    ToppingButton { topping_type: PizzaTopping::Pepperoni }
                    ToppingButton { topping_type: PizzaTopping::Onions }
                    ToppingButton { topping_type: PizzaTopping::Olives }
                    ToppingButton { topping_type: PizzaTopping::Spinach }
                    for _ in 0..8 {
                        button { "blah" }
                    }
                }
            } // order options bracket
            div { class: "current-order-container",
                CurrentOrder {}
                button { class: "send-to-kitchen",
                    "Send to Kitchen"
                }
            }
        }
    }
}

#[component]
fn SizeButton(pizza_size: PizzaSize) -> Element {
    let mut order_context = use_context::<OrderContext>();
    let pizza_id = *order_context.pizza_count.read();

    rsx! {
        button {
            onclick: move |_| {
                order_context.pizzas.write().push(Pizza::new(pizza_id, pizza_size, vec![]));
                order_context.active_pizza.set(Some(pizza_id));
                order_context.pizza_count.set(pizza_id + 1);
            },
            "{pizza_size}"
        }
    }
}

#[component]
fn ToppingButton(topping_type: PizzaTopping) -> Element {
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

#[derive(PartialEq, Clone)]
struct Pizza {
    id: u64,
    size: PizzaSize,
    toppings: Vec<PizzaTopping>,
}

impl Pizza {
    fn new(id: u64, size: PizzaSize, toppings: Vec<PizzaTopping>) -> Self {
        Self { id, size, toppings }
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
fn PizzaDiv(pizza: Pizza) -> Element {
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
