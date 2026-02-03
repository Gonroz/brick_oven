use rusqlite::types::{ToSql, ToSqlOutput, Value};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::pizza;

#[derive(PartialEq, Clone, Deserialize, Serialize)]
pub struct Order {
    pub order_id: i64,
    pub created_at: String,
    pub pizzas: Vec<pizza::Pizza>,
    pub status: String,
}

impl Order {
    pub fn new(
        order_id: i64,
        created_at: String,
        pizzas: Vec<pizza::Pizza>,
        status: String,
    ) -> Self {
        Self {
            order_id,
            created_at,
            pizzas,
            status,
        }
    }
}

impl ToSql for Order {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(self)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        Ok(ToSqlOutput::Owned(Value::Text(json)))
    }
}
