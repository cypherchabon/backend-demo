use std::net::TcpListener;

use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(connection);

    let app = move || {
        App::new()
            .service(health_check)
            .service(subscribe)
            .app_data(db_pool.clone())
    };

    let server = HttpServer::new(app).listen(listener)?.run();

    Ok(server)
}
