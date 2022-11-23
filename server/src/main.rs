mod db;
mod util;
mod errors;
mod routes;
mod twitch_api;

use actix_web::cookie::Key;

use actix_files::{Files, NamedFile};
use actix_web::{web::Data, App, HttpServer};
use actix_web::dev::{fn_service, ServiceResponse, ServiceRequest};
use paperclip::actix::OpenApiExt;
use paperclip::actix::web;

use actix_session::SessionMiddleware;

pub use util::DbPool;
pub use util::RedisPool;

const SCOPES: [&str; 1] = ["channel:read:predictions"];
const REDIRECT_URL: &str = "/twitch_login/";

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

    let http_client = reqwest::Client::new();

    let secret_key_val = std::env::var("SECRET").expect("SECRET must be set");
    let secret_key = Key::from(secret_key_val.as_bytes());

    HttpServer::new(move || {
        App::new()
            .wrap_api()
            .wrap(SessionMiddleware::new(redis_session.clone(), secret_key.clone()))
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(redis_pool.clone()))
            .app_data(Data::new(http_client.clone()))
            //.route("/test", web::get().to(test))
            .service(
                web::scope("/api")
                    .route("/request_login", web::get().to(routes::login_url))
                    .route("/login_check", web::get().to(routes::login_check))
                    .route("/generate_login_token", web::get().to(routes::generate_login_token))
            )
            .route(REDIRECT_URL, web::get().to(routes::twitch_login_end))
            .with_json_spec_at("/api_spec/v2")
            .build()
            .service(Files::new("/sources", "./sources/").index_file("index.html"))
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
    .bind("127.0.0.1:80")?
    .run()
    .await
}