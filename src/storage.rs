use rusqlite::{Connection, Result};

//I want to use a sqlite database for now to store my items
pub fn create_tables(conn: &Connection) -> Result<()> {
    let _ = conn.execute(
        "create table if not exists reading_items (
                 id integer primary key,
                 article_link text not null unique,
                 name text,
                 type text
             )",
        [],
    )?;
    Ok(())
}

//Simple initial implementation with simple insertion of item into db
pub fn add_item(conn: &Connection, article_link: String) -> Result<()> {
    let _ = conn.execute(
        "
            insert into reading_items (article_link) values (?1)
        ",
        &[&article_link],
    )?;
    Ok(())
}
