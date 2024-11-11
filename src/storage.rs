use chrono::{DateTime, Local};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStats {
    articles_in_queue: u32,
    articles_read: u32,
    last_article_added_time: DateTime<Local>,
    last_article_read_time: DateTime<Local>,
}

impl Default for UserStats {
    fn default() -> Self {
        UserStats {
            articles_in_queue: 0,
            articles_read: 0,
            last_article_added_time: Local::now(),
            last_article_read_time: Local::now(),
        }
    }
}

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
pub fn add_item(conn: &Connection, article_link: &String, stat_data: &mut UserStats) -> Result<()> {
    let _ = conn.execute(
        "
            insert into reading_items (article_link) values (?1)
        ",
        &[article_link],
    )?;

    (*stat_data).articles_in_queue = (*stat_data).articles_in_queue + 1;
    (*stat_data).last_article_added_time = Local::now();

    let _ = save_user_stats(stat_data);
    Ok(())
}

pub fn remove_last_read_article(conn: &Connection, stat_data: &mut UserStats) -> Result<()> {
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

    //assign and check for underflow
    (*stat_data).articles_in_queue = (*stat_data).articles_in_queue - 1;
    (*stat_data).articles_read = (*stat_data).articles_read + 1;

    if (*stat_data).articles_in_queue == std::u32::MAX {
        (*stat_data).articles_in_queue = 0
    }

    let _ = save_user_stats(stat_data);

    Ok(())
}

pub fn get_article_fifo(conn: &Connection, stat_data: &mut UserStats) -> Result<String> {
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

    (*stat_data).last_article_read_time = Local::now();
    let _ = save_user_stats(stat_data);

    Ok(article[0].clone())
}

pub fn get_article_random(conn: &Connection, stat_data: &mut UserStats) -> Result<String> {
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

    (*stat_data).last_article_read_time = Local::now();
    let _ = save_user_stats(stat_data);

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

fn save_user_stats(user_stats: &UserStats) -> Result<(), io::Error> {
    const FILE_PATH: &str = "user_stats.json";
    let path = Path::new(FILE_PATH);

    match OpenOptions::new().write(true).create(true).open(&path) {
        Ok(mut file) => {
            // Serialize UserStats to a JSON string
            let json_data = serde_json::to_string(user_stats)?;

            // Write JSON data to the file
            file.write_all(json_data.as_bytes())?;
            Ok(())
        }
        Err(err) => Err(err),
    }
}

pub fn user_stats() -> Result<UserStats, io::Error> {
    const FILE_PATH: &str = "user_stats.json";

    let path = Path::new(FILE_PATH);

    match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
    {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents)? == 0 {
                // File is empty, write the default ID
                let default_stats = UserStats::default();
                file.write_all(serde_json::to_string(&default_stats)?.as_bytes())?;
                return Ok(default_stats);
            }

            match serde_json::from_str(&contents) {
                Ok(user_stats) => Ok(user_stats),
                Err(_) => {
                    // If parsing fails, overwrite with the default ID
                    let default_stats = UserStats::default();
                    file.set_len(0)?; // Truncate the file
                    file.write_all(serde_json::to_string(&default_stats)?.as_bytes())?;
                    Ok(default_stats)
                }
            }
        }
        Err(err) => Err(err),
    }
}
