use dioxus::prelude::*;

#[post("/api/server_test")]
pub async fn server_test(s: String) -> Result<()> {
    use std::io::Write;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("test.txt")
        .unwrap();

    file.write_fmt(format_args!("{s}\n"));

    Ok(())
}
