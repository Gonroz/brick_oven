use core::fmt;

use rusqlite::types::{ToSql, ToSqlOutput, Value};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(PartialEq, Clone, Deserialize, Serialize)]
pub struct Pizza {
    pub id: i64,
    pub size: PizzaSize,
    pub toppings: Vec<PizzaTopping>,
}

impl Pizza {
    pub fn new(id: i64, size: PizzaSize, toppings: Vec<PizzaTopping>) -> Self {
        Self { id, size, toppings }
    }
}

impl ToSql for Pizza {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        Ok(ToSqlOutput::Owned(Value::Text(json)))
    }
}

#[derive(PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum PizzaSize {
    Personal,
    Small,
    Large,
    Sheet,
}

#[derive(PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum PizzaTopping {
    Pepperoni,
    Onions,
    Olives,
    Spinach,
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
