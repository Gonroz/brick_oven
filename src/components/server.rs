use dioxus::prelude::*;

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
    // use std::io::Write;

    // let mut file = std::fs::OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .create(true)
    //     .open("test.txt")
    //     .unwrap();

    // file.write_fmt(format_args!("{s}\n"));

    DB.with(|f| f.execute("INSERT INTO pizzas (size) VALUES (?1)", &[&s]))?;

    Ok(())
}
