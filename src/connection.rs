use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;

static HOST: &str = "localhost";
const PORT: u16 = 27017;
static DB_NAME: &str = "Conf_Count_Test";
static COLLECTION_NAME: &str = "rust_meetup3";

pub fn create_connection() -> Client {
    Client::connect(HOST, PORT).unwrap()
}
pub fn connect_to_collection(client: Client) -> Collection {
    client.db(DB_NAME).collection(COLLECTION_NAME)
}
