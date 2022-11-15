use dotenv::dotenv;
use std::{env, io::Error};
use mongodb::{
    bson::{doc, oid::ObjectId},
    sync::{Client, Collection, Database}};
use crate::schema::project_schema::{Owner, Project};

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

    pub fn create_owner(&self, new_owner: Owner) -> Result<Owner, Error> {
        let new_doc = Owner{
            _id: None,
            name: new_owner.name.clone(),
            email: new_owner.email.clone(),
            phone: new_owner.phone.clone(),
        };
        let col = MongoDB::column_helper::<Owner>(&self, "owner");
        let data = col.insert_one(new_doc, None).ok().expect("Error while creating new owner.");
        let new_owner = Owner {
            _id: data.inserted_id.as_object_id(),
            name: new_owner.name.clone(),
            email: new_owner.email.clone(),
            phone: new_owner.phone.clone(),
        };
        Ok(new_owner)
    }

    pub fn get_owners(&self) -> Result<Vec<Owner>, Error> {
        let col = MongoDB::column_helper(&self, "owner");
        let cursor = col.find(None, None).ok().expect("Error while fetching list of owners.");
        let owner_vec: Vec<Owner> = cursor.map(|doc| doc.unwrap()).collect();
        Ok(owner_vec)
    }

    pub fn get_single_owner(&self, id: &String) -> Result<Owner, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter  = doc! {"_id": obj_id};
        let col =  MongoDB::column_helper::<Owner>(&self, "owner");
        let owner_result  = col
            .find_one(filter, None)
            .ok()
            .expect("Error while fetching requested Owner with ID '{obj_id}");
        Ok(owner_result.unwrap())
    }

    pub fn create_project(&self, new_project: Project) -> Result<Project, Error> {
        let new_doc = Project {
            _id: None,
            owner_id: new_project.owner_id.clone(),
            name: new_project.name.clone(),
            description: new_project.description.clone(),
            status: new_project.status.clone(),
        };
        let col = MongoDB::column_helper(&self, "project");
        let data = col.insert_one(new_doc, None).ok().expect("Error while creating new project.");
        let new_project = Project {
            _id: data.inserted_id.as_object_id(),
            owner_id: new_project.owner_id.clone(),
            name: new_project.name.clone(),
            description: new_project.description.clone(),
            status: new_project.status.clone(),
        };

        Ok(new_project)
    }

    pub fn get_projects(&self) -> Result<Vec<Project>, Error> {
        let col = MongoDB::column_helper(&self, "project");
        let cursor = col.find(None, None).ok().expect("Error while fetching list of projects");
        let project_vec: Vec<Project> = cursor.map(|doc| doc.unwrap()).collect();
        Ok(project_vec)
    }

    pub fn get_single_project(&self, id: &String) -> Result<Project, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let col = MongoDB::column_helper(&self, "project");
        let project_result = col
            .find_one(filter, None)
            .ok()
            .expect("Error while fetching requested Project with ID '{object_id}'");
        Ok(project_result.unwrap())
    }

}