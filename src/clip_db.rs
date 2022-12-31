
mod schema;
use diesel::{prelude::*, insert_into};
// use diesel::sqlite::Sqlite;
use clipboard_entries::dsl::*;


use schema::clipboard_entries;

    #[derive(Queryable, PartialEq, Debug)]
    // #[diesel(table_name = clipboard_entries)]
    pub struct ClipboardEntry {
        id: i32,
        clip_text: String,
        
    }


    pub fn insert_default_values(conn: &mut SqliteConnection) -> QueryResult<usize> {
        insert_into(clipboard_entries).default_values().execute(conn)
    }


    pub fn write_to_db(connection:&mut SqliteConnection,clip_entry:&str) -> QueryResult<usize>{

        insert_into(clipboard_entries).values(clip_text.eq(clip_entry)).execute(connection)

    }


    pub fn establish_connection() -> SqliteConnection {
        let database_url = "./dbase.sqlite";
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }


    pub fn retrieve_clipboard_history(connection: &mut SqliteConnection){

        
            // let connection = &mut establish_connection();
            let results = clipboard_entries
                // .filter(published.eq(true))
                // .limit(5)
                .load::<ClipboardEntry>(connection)
                .expect("Error loading clipboard entries");
        
            println!("Displaying {} posts", results.len());
            for retrieved_entry in results {
                println!("{}", retrieved_entry.id);
                println!("----------\n");
                println!("{}", retrieved_entry.clip_text);
            }
        }
    