use dioxus::prelude::*;

use crate::pizza;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let connection = rusqlite::Connection::open("brick_oven.db").expect("Failed to open database.");

        connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS pizzas (
                database_id INTEGER PRIMARY KEY,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                order_id INTEGER NOT NULL,
                id INTEGER NOT NULL,
                data TEXT NOT NULL,
                status TEXT NOT NULL
            );"
        ).expect("Couldn't make table.");

        connection
    };
}

#[post("/api/server_test")]
pub async fn server_test(s: String) -> Result<()> {
    DB.with(|f| f.execute("INSERT INTO pizzas (size) VALUES (?1)", &[&s]))?;

    Ok(())
}

#[post("/api/save_order")]
pub async fn save_order(pizzas: Vec<pizza::Pizza>) -> Result<()> {
    if pizzas.is_empty() {
        return Ok(());
    }

    // Figure out what order number to give order
    let mut order_id: i32 = 0;

    DB.with(|f| {
        let last_id: i32 = f
            .query_row(
                "SELECT order_id FROM pizzas ORDER BY database_id DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        order_id = if last_id >= 99 { 1 } else { last_id + 1 };
    });

    // Add the pizzas into the database including order_id to know what is what
    for pizza in pizzas {
        DB.with(|f| {
            f.execute(
                "INSERT INTO pizzas (order_id, id, data, status) VALUES (?1, ?2, ?3, ?4)",
                (order_id, pizza.id, &pizza, "Prep".to_string()),
            )
        })
        .expect("Couldn't insert into pizzas table.");
    }
    Ok(())
}

#[get("/api/get_prep_pizzas")]
pub async fn get_prep_pizzas() -> Result<Vec<pizza::Pizza>> {
    DB.with(|f| {
        let mut stmt = f
            .prepare("SELECT data FROM pizzas WHERE status = 'Prep' ORDER BY database_id ASC")
            .expect("Failed to prepare statement");

        let pizza_iter = stmt
            .query_map([], |row| {
                let data: String = row.get(0)?;
                let pizza: pizza::Pizza = serde_json::from_str(&data).unwrap();
                Ok(pizza)
            })
            .expect("Query failed");

        let mut pizzas = Vec::new();
        for pizza in pizza_iter {
            pizzas.push(pizza.expect("Row error"));
        }
        Ok(pizzas)
    })
}
