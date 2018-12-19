use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Bson, Client, ThreadedClient};
use serde_derive::*;

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

    let mut db_name = String::new();
    let mut coll_name = String::new();

    // read from stdin not working
    println!("db name: ");
    std::io::stdin()
        .read_line(&mut db_name)
        .expect("could not read from stdin");
    println!("collection name: ");
    std::io::stdin()
        .read_line(&mut coll_name)
        .expect("could not read from stdin");

    let coll = client.db(&db_name).collection(&coll_name);

    println!("collection found.");

    let cursor = coll.find(None, None).expect("could not create a cursor");
    let total = cursor.count();

    println!("Total number of steps: {}", total);
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerConnectionConfig {
    url: String,
    port: u16,
}
