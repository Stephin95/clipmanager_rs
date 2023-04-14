mod clip_db;
use clap::Parser;
use clip_db::*;
use diesel::prelude::*;
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use std::io::{self, Read};
mod iced_gui;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    store: bool,
    #[arg(short, long)]
    list: bool,
    #[arg(short, long)]
    gui: bool,
    #[arg(short, long)]
    clear: bool,
}


fn main() {
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");
    let args = Args::parse();
    let mut conn = clip_db::establish_connection();
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    if args.store {
        // save_copied_val(& conn)
        save_copied_val(&mut conn,MIGRATIONS)
    } else if args.list {
        let clip_hist_iter = retrieve_clipboard_history(&mut conn);
        print_clipboard(clip_hist_iter)
    } else if args.gui {
        let clip_hist_iter = retrieve_clipboard_history(&mut conn);
        show_gui(clip_hist_iter)
    }
    else if args.clear {
        revert_migrations(&mut conn, MIGRATIONS)
    }
    else {
        println!("Invalid Arguments Supplied")
    }
}

fn print_clipboard(clip_hist:Vec<ClipboardEntry>){
    println!("Displaying {} posts", clip_hist.len());
    for retrieved_entry in clip_hist {
        // println!("{}", retrieved_entry.id);
        // println!("----------\n");
        println!("{}", retrieved_entry.clip_text);
        println!("----------\n");
    }
}
fn save_copied_val(conn: &mut SqliteConnection,migrations:EmbeddedMigrations) {
    let mut bytes = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .expect("No arguments supplied");
    let clipboard_entry = String::from_utf8(bytes).expect("Error converting copied value to utf8");

    remove_duplicates(conn, &clipboard_entry);
    match write_to_db(conn, &clipboard_entry) {
        Ok(result) => {
            println!("{:?}", result)
        }
        Err(error_val) => {
            println!("{:?}", error_val);

            if let Err(_error)=write_to_db(conn, &clipboard_entry){
                conn.run_pending_migrations(migrations).unwrap();
                write_to_db(conn, &clipboard_entry).expect("Error Occured even after creating a table with default values");
            };
        }
    }
}

fn show_gui(cliphist_vec: Vec<ClipboardEntry>) {
    iced_gui::show(cliphist_vec).unwrap();
}

fn revert_migrations(conn:&mut SqliteConnection, migrations:EmbeddedMigrations){
    conn.revert_last_migration(migrations).expect("Error reverting changes to the database");
}
