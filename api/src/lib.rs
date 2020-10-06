mod routes;

use {
    actix_web::{
        web::ServiceConfig
    },
    crate::routes::{
        index, trip
    }
};

pub fn main_config(service_config: &mut ServiceConfig) {
    index::config(service_config);
    trip::config(service_config);
}