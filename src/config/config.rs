use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub sulfur: Vec<Sulfur>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sulfur {
    pub name: String,
    pub cron: Option<String>,
    pub transform: Option<Transform>,
    pub truncate: Option<bool>,
    pub source: Engine,
    pub destination: Engine,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Engine {
    pub r#type: Option<String>,
    pub host: Option<String>,
    pub port: Option<String>,
    pub auth: Option<Auth>,
    pub credentials: Option<String>,

    // Database
    pub database: Option<String>,
    pub collection: Option<String>,
    pub query: Option<String>,

    // Redis
    pub key: Option<String>,

    // BQ
    pub project_id: Option<String>,
    pub dataset_id: Option<String>,
    pub table_id: Option<String>,

    // ElasticSearch
    pub index: Option<String>,
    pub size: Option<i32>,

    // Sqlite
    pub sqlite_database: Option<String>,
    pub sqlite_table: Option<String>,
    pub sqlite_column: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Auth {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transform {
    pub file: Option<String>,
    pub r#fn: Option<String>,
}
