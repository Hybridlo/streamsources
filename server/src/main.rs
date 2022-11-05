mod db;
mod util;
mod errors;
mod twitch_api;

use std::sync::Mutex;

use actix_web::cookie::Key;
use log::info;

use actix_files::{Files, NamedFile};
use actix_web::{get, web, web::Data, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::{fn_service, ServiceResponse, ServiceRequest};

use actix_session::{SessionMiddleware, Session};

use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::Pool;

use twitch_sources_rework::MyTestStruct;

#[get("/test")]
async fn test(db_pool: Data<Pool<AsyncPgConnection>>, session: Session) -> Result<impl Responder, actix_web::Error> {
    db_pool.get().await.map_err(errors::e500)?;

    //session.insert("hi", "sup");
    
    Ok("test success")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    util::init_debug_log();
    util::init_dotenv();

    let db_pool = util::create_connection_pool()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    let redis_session = util::get_redis_session().await
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    let redis_pool = util::get_redis_client_pool()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    let secret_key_val = std::env::var("SECRET").expect("SECRET must be set");
    let secret_key = Key::from(secret_key_val.as_bytes());

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(redis_session.clone(), secret_key.clone()))
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(redis_pool.clone()))
            .service(test)
            .default_service(Files::new("/", "./dist/").index_file("index.html").default_handler(
                fn_service(
                    |req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let file = NamedFile::open_async("./dist/index.html").await?;
                        let res = file.into_response(&req);
                        return Ok(ServiceResponse::new(req, res));
                    }
                )
            ))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}