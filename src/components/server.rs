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
                order_id INTEGER,
                id INTEGER,
                data TEXT NOT NULL
            );"
        ).unwrap();

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

    // let last_id: i32 = f
    //     .query_row("SELECT COALESCE(MAX(order_id), 0) FROM pizzas", [], |row| {
    //         row.get(0)
    //     })
    //     .unwrap_or(0);

    // DB.with(|f| {
    //     let last_id: i32 = f.query_row(
    //         "SELECT COALESCE(MAX(order_id), 0) FROM pizzas",
    //         [],
    //         |row| row.get(0),
    //     ).unwrap_or(0)
    // });

    for pizza in pizzas {
        DB.with(|f| {
            // TODO: this is completely wrong generated code. dumb af LLMs
            // let last_id: i32 = f
            //     .query_row("SELECT COALESCE(MAX(order_id), 0) FROM pizzas", [], |row| {
            //         row.get(0)
            //     })
            //     .unwrap_or(0);

            // let order_id = (last_id % 99) + 1;

            let order_id: i32 = 0;

            f.execute(
                "INSERT INTO pizzas (order_id, id, data) VALUES (?1, ?2, ?3)",
                (order_id, pizza.id, &pizza),
            )
        })?;
    }
    Ok(())
}
