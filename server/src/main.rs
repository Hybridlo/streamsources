mod db;
mod util;
mod errors;
mod routes;
mod twitch_api;

use actix_web::cookie::Key;
use log::info;

use actix_files::{Files, NamedFile};
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use actix_web::web as web_ax;
use actix_web::dev::{fn_service, ServiceResponse, ServiceRequest};
use paperclip::actix::OpenApiExt;
use paperclip::actix::web;

use actix_session::{SessionMiddleware, Session};

pub use util::DbPool;
pub use util::RedisPool;
use errors::e500;
use twitch_api::get_app_token;
use routes::login_url;
use routes::twitch_login_end;


const SCOPES: [&str; 1] = ["channel:read:predictions"];
const REDIRECT_URL: &str = "/twitch_login/";

async fn test(redis_pool: Data<RedisPool>, http_client: Data<reqwest::Client>) -> Result<impl Responder, actix_web::Error> {
    let token = get_app_token(&mut redis_pool.get().await.map_err(e500)?, &**http_client).await.map_err(e500)?;

    //session.insert("hi", "sup");
    
    Ok(token)
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
                    .route("/request_login", web::get().to(login_url))
            )
            .with_json_spec_at("/api_spec/v2")
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
            .build()
            // i really don't know how else to exclude this from the schema
            .route(REDIRECT_URL, web_ax::get().to(twitch_login_end))
    })
    .bind("127.0.0.1:80")?
    .run()
    .await
}