use rusqlite::{Connection, Result};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

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

pub fn remove_last_read_article(conn: &Connection) -> Result<()> {
    let article_id = match get_last_read_id() {
        Ok(id) => id,
        Err(err) => panic!("{}", err),
    };

    if article_id == -1 {
        println!("There is no last read article to remove");
    } else {
        let _ = conn.execute(
            "
                delete from reading_items where id = (?1)
            ",
            &[&article_id],
        )?;
    }

    Ok(())
}

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
    let mut stmt = conn.prepare("SELECT *,min(id) FROM reading_items")?;

    let article: Vec<String> = stmt
        .query_map([], |row| row.get(1))?
        .filter_map(Result::ok) // Filter out any errors
        .collect();

    let id: Vec<i64> = stmt
        .query_map([], |row| row.get(0))?
        .filter_map(Result::ok) // Filter out any errors
        .collect();

    match set_last_read_id(id[0]) {
        Ok(_) => {
            println!("You can now use --article_read to remove this article from the list");
        }
        Err(err) => {
            println!("{}", err);
        }
    };

    Ok(article[0].clone())
}

pub fn get_article_random(conn: &Connection) -> Result<String> {
    let mut stmt = conn.prepare("SELECT * FROM reading_items ORDER BY RANDOM() LIMIT 1")?;

    let article: Vec<String> = stmt
        .query_map([], |row| row.get(1))?
        .filter_map(Result::ok) // Filter out any errors
        .collect();

    let id: Vec<i64> = stmt
        .query_map([], |row| row.get(0))?
        .filter_map(Result::ok) // Filter out any errors
        .collect();

    match set_last_read_id(id[0]) {
        Ok(_) => {
            println!("You can now use --article_read to remove this article from the list");
        }
        Err(err) => {
            println!("{}", err);
        }
    };

    Ok(article[0].clone())
}

//I need to save the id of the last read so that I can remove it from my list
fn get_last_read_id() -> Result<i64, io::Error> {
    const FILE_PATH: &str = "last_read.txt";
    const DEFAULT_ID: i64 = -1;

    let path = Path::new(FILE_PATH);

    match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
    {
        Ok(mut file) => {
            // Attempt to read the file's contents
            let mut contents = String::new();
            if file.read_to_string(&mut contents)? == 0 {
                // If the file is empty, write the default ID
                writeln!(file, "{}", DEFAULT_ID)?;
                return Ok(DEFAULT_ID);
            }

            // Parse the content to i64
            match contents.trim().parse::<i64>() {
                Ok(id) => Ok(id),
                Err(_) => {
                    // If parsing fails, overwrite with the default ID
                    writeln!(file, "{}", DEFAULT_ID)?;
                    Ok(DEFAULT_ID)
                }
            }
        }
        Err(err) => Err(err),
    }
}

fn set_last_read_id(id: i64) -> Result<()> {
    const FILE_PATH: &str = "last_read.txt";

    let path = Path::new(FILE_PATH);
    let _ = match OpenOptions::new()
        .write(true)
        .truncate(true) // Truncate the file to overwrite its contents
        .create(true) // Create the file if it does not exist
        .open(&path)
    {
        Ok(mut file) => {
            let _ = writeln!(file, "{}", id);
            Ok(())
        }
        Err(err) => Err(err),
    };

    Ok(())
}
