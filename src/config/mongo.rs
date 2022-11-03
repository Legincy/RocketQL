use dotenv::dotenv;
use std::{env, io::Error};
use mongodb::{
    bson::{doc, oid::ObjectId},
    sync::{Client, Collection, Database}};
use crate::schema::project_schema::{Owner, Product};

pub struct MongoDB {
    db: Database,
}

impl MongoDB {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI"){
            Ok(v) => v.to_string(),
            Err(_) => format!("Error while loading env variable!"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("projectMngt");
        MongoDB { db }
    }

    fn column_helper<T>(data_source: &Self, collection_name: &str) -> Collection<T> {
        data_source.db.collection(collection_name)
    }
}