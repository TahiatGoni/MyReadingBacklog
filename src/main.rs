use rusqlite::{Connection, Result};
use std::env;
mod storage;

fn print_help() {
    print!(
        "usage:
            --add <link>
            --get_queue_article
            --get_random_article
            --article_read
        "
    )
}

fn main() -> Result<()> {
    let mut conn = Connection::open("reading_items.db")?;

    match storage::create_tables(&conn) {
        Ok(_res) => {}
        Err(err) => eprintln!("{}", err),
    };

    //implemented arg --add and get for now
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_help();
    } else {
        if args[1] == "--add" {
            match storage::add_item(&mut conn, &args[2]) {
                Ok(_res) => println!("ok"),
                Err(err) => eprintln!("{}", err),
            };
        } else if args[1] == "--get_queue_article" {
            match storage::get_article_fifo(&conn) {
                Ok(result) => {
                    println!("{}", result)
                }
                Err(err) => eprintln!("{}", err),
            };
        } else if args[1] == "--get_random_article" {
            match storage::get_article_random(&conn) {
                Ok(result) => {
                    println!("{}", result)
                }
                Err(err) => eprintln!("{}", err),
            };
        } else if args[1] == "--article_read" {
            match storage::remove_last_read_article(&conn) {
                Ok(_) => {
                    println!("Article removed from list!")
                }
                Err(err) => {
                    eprintln!("{}", err)
                }
            };
        } else {
            print_help();
        }
    }

    Ok(())
}
