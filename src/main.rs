#[macro_use] extern crate bson;

use hawk_conf::{connection, time_operations, data_operations};
use log::info;
use mongodb::coll::Collection;

static LOG_LEVEL_KEY: &str = "RUST_LOG";
static LOG_LEVEL_VALUE: &str = "hawk_conf=info";
static TOTAL_STRENGTH: &str = "Total_Strength";
static TIME: &str = "Time";

fn main() {
    std::env::set_var(LOG_LEVEL_KEY, LOG_LEVEL_VALUE);
    env_logger::init();

    let collection: Collection =
        connection::connect_to_collection(connection::create_connection());
    let time: i64 = time_operations::get_date_time();
    let name = "abs";
    let data = doc! {
    "ID": "2013",
    "Name": name,
    "E-mail": "alok.jha@knoldus.in",
    "Status": "Absent"};
    //info!("{}", data_operations::insert_data(collection, data));
    info!("{:?}", data_operations::fetch_collection(&collection));
    let filter_doc = doc! {"E-mail": "alok.jha@knoldus.in"};
 //   info!("{:?}", collection.count(Some(filter_doc), None).unwrap());
    match collection.count(Some(filter_doc), None).expect("Unable to connect to Mongo DB") {
        0 => {
            println!("12");
        }
        _ => {
            println!("User with email  already registered");
        }
    };
}
