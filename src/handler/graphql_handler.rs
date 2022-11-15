use crate::{
    config::mongo::MongoDB,
    schema::project_schema::{CreateOwner, CreateProject, FetchOwner, FetchProject, Owner, Project},
};
use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema};

pub struct Query;
pub struct Mutation;

#[Object(extends)]
impl Query {
    async fn owner(&self, context: &Context<'_>, input: FetchOwner) -> FieldResult<Owner> {
        let db = &context.data_unchecked::<MongoDB>();
        let owner = db.get_single_owner(&input._id).unwrap();
        Ok(owner)
    }

    async fn get_owners(&self, context: &Context<'_>) -> FieldResult<Vec<Owner>> {
        let db = &context.data_unchecked::<MongoDB>();
        let owner_vec = db.get_owners().unwrap();
        Ok(owner_vec)
    }

    async fn project(&self, context: &Context<'_>, input: FetchProject) -> FieldResult<Project> {
        let db = &context.data_unchecked::<MongoDB>();
        let project = db.get_single_project(&input._id).unwrap();
        Ok(project)
    }

    async fn get_projects(&self, context: &Context<'_>) -> FieldResult<Vec<Project>> {
        let db = &context.data_unchecked::<MongoDB>();
        let project_vec = db.get_projects().unwrap();
        Ok(project_vec)
    }
}

#[Object]
impl Mutation {
    async fn create_owner(&self, context: &Context<'_>, input: CreateOwner) -> FieldResult<Owner> {
        let db = &context.data_unchecked::<MongoDB>();
        let new_owner = Owner {
            _id: None,
            email: input.email,
            name: input.name,
            phone: input.phone,
        };
        let owner = db.create_owner(new_owner).unwrap();
        Ok(owner)
    }

    async fn create_project(&self, context: &Context<'_>, input: CreateProject) -> FieldResult<Project> {
        let db = &context.data_unchecked::<MongoDB>();
        let new_project = Project {
            _id: None,
            owner_id: input.owner_id,
            name: input.name,
            description: input.description,
            status: input.status,
        };
        let project = db.create_project(new_project).unwrap();
        Ok(project)
    }
}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;