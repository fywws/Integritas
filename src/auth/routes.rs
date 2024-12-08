use actix_web::web;

use crate::auth::handlers::{login, register};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(register)
            .service(login),
    );
}
