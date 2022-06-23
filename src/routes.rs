use crate::api;
use actix_web::web;
use log::info;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes");
    cfg.service(
        web::scope("/api").service(
            web::scope("/users")
                .service(
                    web::resource("")
                        .route(web::get().to(api::find_all))
                        .route(web::post().to(api::add_user)),
                )
                .service(
                    web::resource("/{id}")
                        .route(web::get().to(api::find_by_id))
                        .route(web::put().to(api::update_user))
                        .route(web::delete().to(api::delete_user)),
                ),
        ),
    );
}
