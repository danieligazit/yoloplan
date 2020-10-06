mod routes;

use {
    actix_web::{
        web::ServiceConfig
    },
    crate::routes::{
        index
    }
};

pub fn main_config(service_config: &mut ServiceConfig) {
    index::config(service_config);
}