#[derive(PartialEq, Clone)]
pub struct Pizza {
    pub id: u64,
    pub size: PizzaSize,
    pub toppings: Vec<PizzaTopping>,
}

impl Pizza {
    pub fn new(id: u64, size: PizzaSize, toppings: Vec<PizzaTopping>) -> Self {
        Self { id, size, toppings }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum PizzaSize {
    Personal,
    Small,
    Large,
    Sheet,
}

#[derive(PartialEq, Clone, Copy)]
pub enum PizzaTopping {
    Pepperoni,
    Onions,
    Olives,
    Spinach,
}
