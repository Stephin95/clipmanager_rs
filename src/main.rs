mod clip_db;
use std::io::{self, Read};
use diesel_migrations::MigrationHarness;
use diesel::prelude::*;
use clip_db::*;
use clap::Parser;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

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
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn run_migration(conn: &mut SqliteConnection, MIGRATIONS: EmbeddedMigrations) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

fn main() {
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    let args = Args::parse();
    let mut conn = clip_db::establish_connection();
    run_migration(&mut conn, MIGRATIONS);
    if args.store {
        // save_copied_val(& conn)
        save_copied_val(&mut conn)
    } else if args.list {
        let clip_hist_iter = retrieve_clipboard_history(&mut conn);
        if args.gui {
            // show_gui(clip_hist_iter)
        } else {
            // print_cliphist(clip_hist_iter)
        }
    }
}
fn save_copied_val(conn: &mut SqliteConnection) {
    let mut bytes = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .expect("No arguments supplied");
    let clipboard_entry = String::from_utf8(bytes).expect("Error converting copied value to utf8");
    println!("{}", clipboard_entry);
    match write_to_db(conn, &clipboard_entry) {
        Ok(result) => {
            println!("{:?}", result)
        }
        Err(error_val) => {
            println!("{:?}", error_val);
            insert_default_values(conn).unwrap();
            write_to_db(conn, &clipboard_entry)
                .expect("Error Occured even after creating a table with default values");
        }
    }

}

fn show_gui(cliphist_vec: Vec<String>) {
    // iced_gui::show(cliphist_vec);
}
