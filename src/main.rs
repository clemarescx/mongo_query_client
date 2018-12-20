use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Bson, Client, ThreadedClient};
use serde_derive::*;
use std::io::{self, Write};

const CONFIG_FILEPATH: &'static str = "db_connection.json";

fn main() {
    println!("Hello, world!");
    let config_content = std::fs::read_to_string(CONFIG_FILEPATH)
        .expect(&format!["could not find {}", CONFIG_FILEPATH]);

    let db_config: ServerConnectionConfig = serde_json::from_str(&config_content).expect(&format!(
        "could not deserialize into a ServerConnectionConfig:\n {}",
        config_content
    ));
    println!("deserialisation successful:\n {:#?}", db_config);
    let client = Client::connect(&db_config.url, db_config.port)
        .expect("Failed to connect client to database");

    let mut db_name_buffer = String::new();
    let mut coll_name_buffer = String::new();

    read_from_stdin("db name: ", &mut db_name_buffer);
    read_from_stdin("collection name: ", &mut coll_name_buffer);

    let db_name = if db_name_buffer.trim().is_empty() {
        db_config.default_db
    } else {
        db_name_buffer.trim().to_owned()
    };
    let coll_name = if coll_name_buffer.trim().is_empty() {
        db_config.default_collection
    } else {
        coll_name_buffer.trim().to_owned()
    };

    let coll = client.db(&db_name).collection(&coll_name);

    let cursor = coll.find(None, None).expect("could not create a cursor");
    let total = cursor.count();

    println!("Total number of entries: {}", total);
}

fn read_from_stdin(prompt: &str, buffer: &mut String) {
    print!("{}", prompt);
    io::stdout()
        .flush()
        .expect("could not print prompt to stdout");
    io::stdin()
        .read_line(buffer)
        .expect("could not read from stdin");
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerConnectionConfig {
    url: String,
    port: u16,
    default_db: String,
    default_collection: String,
}
