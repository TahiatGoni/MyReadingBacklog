use rusqlite::{Connection, Result};

//I want to use a sqlite database for now to store my items
pub fn create_tables(conn: &Connection) -> Result<()> {
    let _ = conn.execute(
        "create table if not exists reading_items (
                 id integer primary key,
                 article_link text not null unique,
                 name text,
                 article_type text
             )",
        [],
    )?;
    Ok(())
}

//Simple initial implementation with simple insertion of item into db
pub fn add_item(conn: &Connection, article_link: &String) -> Result<()> {
    let _ = conn.execute(
        "
            insert into reading_items (article_link) values (?1)
        ",
        &[article_link],
    )?;
    Ok(())
}

// pub fn remove_article_with_id(conn: &Connection, article_id: u32) -> Result<()> {
//     let _ = conn.execute(
//         "
//             delete from reading_items where id = (?1)
//         ",
//         &[&article_id],
//     )?;
//     Ok(())
// }

// pub fn remove_article_with_name(conn: &Connection, article_name: &String) -> Result<()> {
//     let _ = conn.execute(
//         "
//             delete from reading_items where name = (?1)
//         ",
//         &[&article_name],
//     )?;
//     Ok(())
// }

pub fn get_article_fifo(conn: &Connection) -> Result<String> {
    let mut stmt = conn.prepare("SELECT *,min(id) from reading_items")?;

    let article: Vec<String> = stmt
        .query_map([], |row| row.get(1))?
        .filter_map(Result::ok) // Filter out any errors
        .collect();

    Ok(article[0].clone())
}
