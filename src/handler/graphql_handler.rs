use crate::{
    config::mongo::MongoDB,
    schema::project_schema::{Employee, CreateEmployee, FetchEmployee, DeleteEmployee, UpdateEmployee,
                             Store, CreateStore, FetchStore,
                             Location, CreateLocation, FetchLocation,
                             Rank, CreateRank, FetchRank},
};
use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema};

pub struct Query;
pub struct Mutation;

#[Object(extends)]
impl Query {
    /*
     * Employee Queries
     */
    async fn get_employee(&self, context: &Context<'_>, input: FetchEmployee) -> FieldResult<Employee> {
        let db: &&MongoDB = &context.data_unchecked::<MongoDB>();
        let found_employee: Employee = db.get_single_employee(&input.id).unwrap();

        Ok(found_employee)
    }

    async fn get_all_employees(&self, context: &Context<'_>) -> FieldResult<Vec<Employee>> {
        let db: &&MongoDB = &context.data_unchecked::<MongoDB>();
        let employee_vec: Vec<Employee> = db.get_all_employees().unwrap();

        Ok(employee_vec)
    }

    /*
     * Store Queries
     */
    async fn get_store(&self, context: &Context<'_>, input: FetchStore) -> FieldResult<Store> {
        let db: &&MongoDB = &context.data_unchecked::<MongoDB>();
        let found_store: Store = db.get_single_store(&input.id).unwrap().unwrap();

        Ok(found_store)
    }

    async fn get_all_stores(&self, context: &Context<'_>) -> FieldResult<Vec<Store>> {
        let db: &&MongoDB = &context.data_unchecked::<MongoDB>();
        let store_vec: Vec<Store> = db.get_all_stores().unwrap();

        Ok(store_vec)
    }

    /*
     * Location Queries
     */
    async fn get_location(&self, context: &Context<'_>, input: FetchLocation) -> FieldResult<Location> {
        let db: &&MongoDB = &context.data_unchecked::<MongoDB>();
        let found_location: Location = db.get_single_location(&input.id).unwrap().unwrap();

        Ok(found_location)
    }

    async fn get_all_locations(&self, context: &Context<'_>) -> FieldResult<Vec<Location>> {
        let db: &&MongoDB = &context.data_unchecked::<MongoDB>();
        let location_vec: Vec<Location> = db.get_all_locations().unwrap();

        Ok(location_vec)
    }

    /*
     * Rank Queries
     */
    async fn get_rank(&self, context: &Context<'_>, input: FetchRank) -> FieldResult<Rank> {
        let db: &&MongoDB = &context.data_unchecked::<MongoDB>();
        let found_rank: Rank = db.get_single_rank(&input.id).unwrap().unwrap();

        Ok(found_rank)
    }

    async fn get_all_ranks(&self, context: &Context<'_>) -> FieldResult<Vec<Rank>> {
        let db: &&MongoDB = &context.data_unchecked::<MongoDB>();
        let rank_vec: Vec<Rank> = db.get_all_ranks().unwrap();

        Ok(rank_vec)
    }
}

#[Object]
impl Mutation {
    /*
     * Employee Mutations
     */
    async fn create_employee(&self, context: &Context<'_>, input: CreateEmployee) -> FieldResult<Employee> {
        let db: &&MongoDB  = &context.data_unchecked::<MongoDB>();
        let created_employee = db.create_employee(input).unwrap();

        Ok(created_employee)
    }

    async fn update_employee(&self, context: &Context<'_>, input: UpdateEmployee) -> FieldResult<Employee> {
        let db: &&MongoDB = &context.data_unchecked::<MongoDB>();
        let updated_employee = db.update_employee(input).unwrap();

        Ok(updated_employee)
    }

    async fn delete_employee(&self, context: &Context<'_>, input: DeleteEmployee) -> FieldResult<Employee> {
        let db: &&MongoDB = &context.data_unchecked::<MongoDB>();
        let deleted_employee = db.delete_employee(input).unwrap();

        Ok(deleted_employee)
    }

    /*
     * Store Mutations
     */
    async fn create_store(&self, context: &Context<'_>, input: CreateStore) -> FieldResult<Store> {
        let db: &&MongoDB  = &context.data_unchecked::<MongoDB>();
        let created_store: Store = db.create_store(input).unwrap();

        Ok(created_store)
    }

    /*
     * Location Mutations
     */
    async fn create_location(&self, context: &Context<'_>, input: CreateLocation) -> FieldResult<Location> {
        let db: &&MongoDB  = &context.data_unchecked::<MongoDB>();
        let created_location: Location = db.create_location(input).unwrap();

        Ok(created_location)
    }

    /*
     * Rank Mutations
     */
    async fn create_rank(&self, context: &Context<'_>, input: CreateRank) -> FieldResult<Rank> {
        let db: &&MongoDB  = &context.data_unchecked::<MongoDB>();
        let created_rank: Rank = db.create_rank(input).unwrap();

        Ok(created_rank)
    }
}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;