use dioxus::prelude::*;

use crate::pizza;
use crate::server;

#[derive(Clone, Copy)]
struct PrepContext {
    pizzas: Signal<Vec<pizza::Pizza>>,
    active_pizza: Signal<Option<i64>>,
}

impl PrepContext {
    fn new() -> Self {
        PrepContext {
            pizzas: Signal::new(vec![]),
            active_pizza: Signal::new(None),
        }
    }
}

#[component]
pub fn Prep() -> Element {
    use_context_provider(|| PrepContext::new());

    rsx! {
        div { class: "order-container",
            PrepStation { }
        }
    }
}

#[component]
pub fn PrepStation() -> Element {
    let mut prep_context = use_context::<PrepContext>();
    let mut refresh_trigger = use_signal(|| 0);

    let _ = use_resource(move || async move {
        let _ = refresh_trigger.read();

        if let Ok(polled_pizzas) = server::get_prep_pizzas().await {
            prep_context.pizzas.set(polled_pizzas);
        }
    });

    // make sure to uncomment when actually running
    // use_future(move || async move {
    //     loop {
    //         tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    //         refresh_trigger += 1;
    //     }
    // });

    rsx! {
        div{ class: "prep-station",
            for p in prep_context.pizzas.read().iter() {
                PizzaPrep { pizza: p.clone() }
            }
        }
    }
}

#[component]
pub fn PizzaPrep(pizza: pizza::Pizza) -> Element {
    let mut order_context = use_context::<PrepContext>();
    let active_pizza = *order_context.active_pizza.read();

    rsx! {
        div { class: "pizza-prep",
            "pizza"
        }
    }
}
