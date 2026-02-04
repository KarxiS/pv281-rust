// traits necessary for any Database repository, some handy functions and error handling
pub use common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, PoolHandler,
    error, query_parameters, run_migration::run_migration,
};

mod common;
// models used by the repositories (serialization/deserialization)
pub mod models;
// repositories to access the database
pub mod repositories;
