use rusqlite::{Connection, Result};
mod storage;

fn main() -> Result<()> {
    let mut conn = Connection::open("reading_items.db")?;

    match storage::create_tables(&conn) {
        Ok(_res) => println!("ok"),
        Err(err) => eprintln!("{}", err),
    };

    match storage::add_item(
        &mut conn,
        String::from("https://github.com/TahiatGoni/my_reading_backlog"),
    ) {
        Ok(_res) => println!("ok"),
        Err(err) => eprintln!("{}", err),
    };

    Ok(())
}
