use dioxus::prelude::*;

use crate::order;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let connection = rusqlite::Connection::open("brick_oven.db").expect("Failed to open database.");

        connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS pizzas (
                id INTEGER PRIMARY KEY,
                size TEXT NOT NULL
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
pub async fn save_order() -> Result<()> {
    Ok(())
}
