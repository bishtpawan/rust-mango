use bson::Document;
use mongodb::coll::Collection;

static SUCCESS_MESSAGE: &str = "Successfully Inserted";
static FAILURE_MESSAGE: &str = "Unable to  insert data";

pub fn insert_data(collection: Collection, data: Document) -> &'static str {
    match collection.insert_one(data, None) {
        Ok(_) => SUCCESS_MESSAGE,
        Err(_) => FAILURE_MESSAGE
    }
}

pub fn fetch_collection(collection: &Collection)-> Vec<String> {
    let mut user_ids: Vec<String> = Vec::new();
    match collection.find(None, None) {
        Ok(user_data) => {
            let user_document: Vec<bson::Document> = user_data.map(|data| data.unwrap()).collect();
            for user_id in user_document {
                user_ids.push(user_id.get_str("ID").expect("No such key exist").to_string());
            }
        }
        Err(_) => ()
    }
    user_ids
}