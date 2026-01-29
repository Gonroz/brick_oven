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
pub fn Prep() -> Element {
    use_context_provider(|| OrderContext::new());

    rsx! {
        div { class: "order-container",
            "Prep"
        }
    }
}
