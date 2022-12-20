use dotenv::dotenv;
use std::{env, io::Error};
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    sync::{Client, Collection, Database, Cursor},
    results::{InsertOneResult}};
use crate::schema::project_schema::{CreateEmployee, DeleteEmployee, Employee, UpdateEmployee, Status, Store, CreateStore, Location, Rank, CreateLocation, CreateRank};

pub struct MongoDB {
    db: Database,
}

impl MongoDB {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI"){
            Ok(v) => v.to_string(),
            Err(_) => format!("Error while loading environment file!"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("praktikum");
        MongoDB { db }
    }

    fn column_helper<T>(data_source: &Self, collection_name: &str) -> Collection<T> {
        data_source.db.collection(collection_name)
    }

    /*
     * Employee Repository
     */
    pub fn delete_employee(&self, delete_entry: DeleteEmployee) -> Result<Employee, Error> {
        let obj_id: ObjectId = ObjectId::parse_str(&delete_entry.id).unwrap();
        let col: Collection<Employee> =  MongoDB::column_helper::<Employee>(&self, "employee");
        let filter: Document  = doc! {"_id": obj_id};

        let _data = col.delete_one(filter, None)
            .ok()
            .expect(&*format!("Error while deleting requested Employee with ID '{}'", obj_id));

        Ok(Employee{id: None, first_name: String::from(""), last_name: String::from(""), status: None, stores: None, rank_id: None })
    }

    pub fn update_employee(&self, update_entry: UpdateEmployee) -> Result<Employee, Error> {
        let obj_id: ObjectId = ObjectId::parse_str(&update_entry.id).unwrap();
        let fetch_filter: Document  = doc! {"_id": obj_id};

        let col: Collection<Employee> =  MongoDB::column_helper::<Employee>(&self, "employee");
        let employee_result: Employee = col
            .find_one(fetch_filter, None)
            .ok()
            .expect(&*format!("Error while fetching requested Employee with ID '{}'", obj_id))
            .unwrap();

        let validated_rank = if update_entry.rank_id == None {employee_result.rank_id.unwrap()} else {
            let opt_rank = self.get_single_rank(&update_entry.rank_id.unwrap()).unwrap();
            if opt_rank.is_some() {
                opt_rank.unwrap().id.unwrap().to_string()
            }else {
                employee_result.rank_id.unwrap().clone()
            }
        };

        let first_name: String = if update_entry.first_name == None {employee_result.first_name} else { update_entry.first_name.clone().unwrap() };
        let last_name: String= if update_entry.last_name == None {employee_result.last_name} else { update_entry.last_name.clone().unwrap() };
        let status: String = if update_entry.status == None {Status::None.to_string()} else { update_entry.status.unwrap().to_string() };
        let validated_stores: Vec<String> = if update_entry.stores == None {vec![]} else { self.validate_store_vec(&update_entry.stores.unwrap()) };

        let update_filter: Document  = doc! {"_id": obj_id};
        let update: Document  = doc! {"$set": {"first_name": String::from(&first_name), "last_name": String::from(&last_name), "stores": &validated_stores, "rank_id": &validated_rank, "status": status}};

        col.update_one(update_filter, update,None).ok().expect(&*format!("Error while updating requested Employee with ID '{}", obj_id));

        let updated_employee = self.get_single_employee(&update_entry.id);

        Ok(updated_employee.unwrap())
    }

    pub fn create_employee(&self, new_entry: CreateEmployee) -> Result<Employee, Error> {
        let col: Collection<Employee> = MongoDB::column_helper::<Employee>(&self, "employee");
        let validated_stores: Vec<String> = self.validate_store_vec(&new_entry.stores.unwrap());
        let validated_rank: String =  self.validate_rank(&new_entry.rank_id);

        let mut new_doc = Employee{
            id: None,
            first_name: new_entry.first_name.clone(),
            last_name: new_entry.last_name.clone(),
            status: Option::from(if new_entry.status == None { Status::None } else { new_entry.status.unwrap() }),
            stores: Option::from(validated_stores),
            rank_id: Option::from(validated_rank),
        };

        let data: InsertOneResult = col.insert_one(&new_doc, None)
            .ok()
            .expect("Error while creating new employee.");
        new_doc.id =  data.inserted_id.as_object_id();

        Ok(new_doc)
    }

    pub fn get_all_employees(&self) -> Result<Vec<Employee>, Error> {
        let col: Collection<Employee> = MongoDB::column_helper(&self, "employee");
        let cursor: Cursor<Employee>= col.find(None, None)
            .ok()
            .expect("Error while fetching list of employees.");

        let employee_vec: Vec<Employee> = cursor
            .map(|doc| doc.unwrap())
            .collect();

        Ok(employee_vec)
    }

    pub fn get_single_employee(&self, id: &String) -> Result<Employee, Error> {
        let obj_id: ObjectId = ObjectId::parse_str(id).unwrap();
        let filter: Document  = doc! {"_id": obj_id};
        let col: Collection<Employee> =  MongoDB::column_helper::<Employee>(&self, "employee");

        let opt_employee: Option<Employee> = col
            .find_one(filter, None)
            .ok()
            .expect(&*format!("Error while fetching requested employee with ID '{}'", obj_id));

        Ok(opt_employee.unwrap())
    }

    /*
     * Store Repository
     */
    pub fn create_store(&self, new_entry: CreateStore) -> Result<Store, Error> {
        let col: Collection<Store> = MongoDB::column_helper::<Store>(&self, "store");
        let validated_location = self.validate_location(&new_entry.location_id);

        let mut new_doc = Store{
            id: None,
            name: new_entry.name.clone(),
            location_id: String::from("")
        };

        let data: InsertOneResult = col.insert_one(&new_doc, None)
            .ok()
            .expect("Error while creating new store.");
        new_doc.id =  data.inserted_id.as_object_id();

        Ok(new_doc)
    }

    pub fn get_all_stores(&self) -> Result<Vec<Store>, Error> {
        let col: Collection<Store> = MongoDB::column_helper(&self, "store");
        let cursor: Cursor<Store>= col.find(None, None)
            .ok()
            .expect("Error while fetching list of stores.");

        let store_vec: Vec<Store> = cursor
            .map(|doc| {
                doc.unwrap()
            })
            .collect();

        Ok(store_vec)
    }

    pub fn get_single_store(&self, id: &String) -> Result<Option<Store>, Error> {
        let obj_id: ObjectId = ObjectId::parse_str(id).expect("Ungültige ID");
        let filter: Document  = doc! {"_id": obj_id};
        let col: Collection<Store> =  MongoDB::column_helper::<Store>(&self, "store");

        let opt_store: Option<Store> = col
            .find_one(filter, None)
            .ok()
            .expect(&*format!("Error while fetching requested store with ID '{}'", obj_id));

        Ok(opt_store)
    }

    pub fn validate_store_vec(&self, store_vec: &Vec<String>) -> Vec<String> {
        let mut valid_store_vec: Vec<String> = Vec::new();

        for store_id in store_vec.clone().iter(){
            let fetched_store: Option<Store> = self.get_single_store(&store_id).unwrap();

            if fetched_store.is_some() {
                valid_store_vec.push(String::from(store_id));
            }
        }

        valid_store_vec
    }

    /*
     * Location Repository
     */
    pub fn create_location(&self, new_entry: CreateLocation) -> Result<Location, Error> {
        let col: Collection<Location> = MongoDB::column_helper::<Location>(&self, "location");
        let mut new_doc = Location{
            id: None,
            country: new_entry.country.clone(),
            state: new_entry.state.clone()
        };

        let data: InsertOneResult = col.insert_one(&new_doc, None)
            .ok()
            .expect("Error while creating new location.");
        new_doc.id =  data.inserted_id.as_object_id();

        Ok(new_doc)
    }

    pub fn get_all_locations(&self) -> Result<Vec<Location>, Error> {
        let col: Collection<Location> = MongoDB::column_helper(&self, "location");
        let cursor: Cursor<Location>= col.find(None, None)
            .ok()
            .expect("Error while fetching list of stores.");

        let location_vec: Vec<Location> = cursor
            .map(|doc| {
                doc.unwrap()
            })
            .collect();

        Ok(location_vec)
    }

    pub fn get_single_location(&self, id: &String) -> Result<Option<Location>, Error> {
        let obj_id: ObjectId = ObjectId::parse_str(id).expect("Ungültige ID");
        let filter: Document  = doc! {"_id": obj_id};
        let col: Collection<Location> =  MongoDB::column_helper::<Location>(&self, "location");

        let opt_location: Option<Location> = col
            .find_one(filter, None)
            .ok()
            .expect(&*format!("Error while fetching requested location with ID '{}'", obj_id));

        Ok(opt_location)
    }

    pub fn validate_location(&self, location_id: &String) -> Option<Location> {
        let mut valid_location_id: String = location_id.clone();

        let fetched_location: Option<Location> = self.get_single_location(location_id).unwrap();
        //if fetched_location.is_none() { valid_location_id = String::from("")}

        fetched_location
        //valid_location_id
    }

    /*
     * Rank Repository
     */
    pub fn create_rank(&self, new_entry: CreateRank) -> Result<Rank, Error> {
        let col: Collection<Rank> = MongoDB::column_helper::<Rank>(&self, "rank");
        let mut new_doc = Rank{
            id: None,
            name: new_entry.name,
            description: new_entry.description
        };

        let data: InsertOneResult = col.insert_one(&new_doc, None)
            .ok()
            .expect("Error while creating new rank.");
        new_doc.id =  data.inserted_id.as_object_id();

        Ok(new_doc)
    }

    pub fn get_all_ranks(&self) -> Result<Vec<Rank>, Error> {
        let col: Collection<Rank> = MongoDB::column_helper(&self, "rank");
        let cursor: Cursor<Rank>= col.find(None, None)
            .ok()
            .expect("Error while fetching list of ranks.");

        let rank_vec: Vec<Rank> = cursor
            .map(|doc| {
                doc.unwrap()
            })
            .collect();

        Ok(rank_vec)
    }

    pub fn get_single_rank(&self, id: &String) -> Result<Option<Rank>, Error> {
        let obj_id: ObjectId = ObjectId::parse_str(id).expect("Ungültige ID");
        let filter: Document  = doc! {"_id": obj_id};
        let col: Collection<Rank> =  MongoDB::column_helper::<Rank>(&self, "rank");

        let opt_rank: Option<Rank> = col
            .find_one(filter, None)
            .ok()
            .expect(&*format!("Error while fetching requested rank with ID '{}'", obj_id));

        Ok(opt_rank)
    }

    pub fn validate_rank(&self, rank_id: &String) -> String {
        let mut valid_rank_id: String = rank_id.clone();

        let fetched_rank: Option<Rank> = self.get_single_rank(rank_id).unwrap();
        if fetched_rank.is_none() { valid_rank_id = String::from("")}

        valid_rank_id
    }
}