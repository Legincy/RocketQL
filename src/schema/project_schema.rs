use async_graphql::{Enum, InputObject, SimpleObject};
use mongodb::bson::{
    oid::ObjectId
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Employee {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub status: Option<Status>,
    pub stores: Option<Vec<String>>,
    pub rank_id: Option<String>,
}

#[derive(InputObject)]
pub struct CreateEmployee {
    pub first_name: String,
    pub last_name: String,
    pub status: Option<Status>,
    pub stores: Option<Vec<String>>,
    pub rank_id: String
}

#[derive(InputObject)]
pub struct FetchEmployee {
    pub id: String,
}

#[derive(InputObject)]
pub struct DeleteEmployee {
    pub id: String,
}

#[derive(InputObject)]
pub struct UpdateEmployee {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub status: Option<Status>,
    pub stores: Option<Vec<String>>,
    pub rank_id: String
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum Status {
    None,
    Working,
    EmergencyService,
    Vacation,
    Illness,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Store {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub location_id: String
}

#[derive(InputObject)]
pub struct CreateStore {
    pub name: String,
    pub location_id: String
}


#[derive(InputObject)]
pub struct FetchStore {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Location {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub country: String,
    pub state: String,
}

#[derive(InputObject)]
pub struct CreateLocation {
    pub country: String,
    pub state: String,
}

#[derive(InputObject)]
pub struct FetchLocation {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Rank {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: String,
}

#[derive(InputObject)]
pub struct CreateRank {
    pub name: String,
    pub description: String,
}

#[derive(InputObject)]
pub struct FetchRank {
    pub id: String,
}