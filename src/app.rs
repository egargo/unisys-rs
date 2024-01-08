use crate::services::index::{index, oops};
use actix_web::web::{self, ServiceConfig};

pub fn app(cfg: &mut ServiceConfig) {
    cfg.default_service(web::route().to(oops)).service(web::resource("/").to(index));
}
