mod db;
mod domain;
mod util;
mod errors;
mod http_client;
mod routes;
mod websockets;
mod twitch_api;
mod middlewares;
mod my_redis;

use actix_web::cookie::Key;

use actix_files::{Files, NamedFile};
use actix_web::web as web_ax;
use actix_web::{web::Data, App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::dev::{fn_service, ServiceResponse, ServiceRequest};
use http_client::twitch_client::SubCondition;
use paperclip::actix::OpenApiExt;
use paperclip::actix::web;

use actix_session::SessionMiddleware;

use twitch_sources_rework::common_data::SubType;
pub use util::DbPool;
pub use util::RedisPool;

const SCOPES: [&str; 1] = ["channel:read:predictions"];
const REDIRECT_URL: &str = "/twitch_login/";
const WEBHOOK_URL: &str = "/webhook/";

#[cfg(not(debug_assertions))]
const PROD_BASE_URL: &str = "https://will_see.com";

pub type RunningTests = dashmap::DashSet<String>;

#[actix_web::main]
async fn main() {
    util::init_debug_log();
    util::init_dotenv();

    let db_pool = util::create_connection_pool()
        .expect("Unable to create DB pool");

    let redis_session = util::get_redis_session().await
        .expect("Unable to connect to Redis");

    let redis_pool = util::get_redis_client_pool()
        .expect("Unable to connect to Redis");

    let context = util::Context::new();

    let http_client = reqwest::Client::new();

    let secret_key_val = std::env::var("SECRET").expect("SECRET must be set");
    let secret_key = Key::from(secret_key_val.as_bytes());

    // check and create needed subscriptions
    domain::subscription::Subscription::get_or_create_subscriptions(
        &context,
        vec![
            SubType::UserAuthorizationRevoke
        ],
        SubCondition::client_id(),
    )
        .await
        .expect("UserAuthorizationRevoke subscription failed");

    _ = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap_api()
            .wrap(SessionMiddleware::new(redis_session.clone(), secret_key.clone()))
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(redis_pool.clone()))
            .app_data(Data::new(http_client.clone()))
            .app_data(context.clone())
            .app_data(Data::new(RunningTests::new()))
            .service(
                web::scope("/api")
                    .route("/request_login", web::get().to(routes::login_url))
                    .route("/login_check", web::get().to(routes::login_check))
                    .route("/generate_login_token", web::get().to(routes::generate_login_token))
                    .route("/test", web::get().to(routes::execute_test))
            )
            .route(REDIRECT_URL, web::get().to(routes::twitch_login_end))
            .with_json_spec_at("/api_spec/v2")
            .build()
            // after openapi generation
            
            .service(
                web_ax::scope("/ws")
                    .service(
                        // i don't need two scopes now, but just in case
                        web_ax::scope("/sources")
                            .route("/predictions", web_ax::get().to(websockets::predictions_websocket))
                            .route("/hype_train", web_ax::get().to(websockets::hype_train_websocket))
                    )
            )
            .route(WEBHOOK_URL, web_ax::post().to(routes::webhook))
            .service(
                web_ax::scope("/sources")
                    .wrap(middlewares::AutoTwitchLoginFactory)
                    .wrap(middlewares::QuickLoginFactory)
                    .service(Files::new("", "./sources/").index_file("index.html"))
            )
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
    .bind("127.0.0.1:80")
    .expect("Server binding failed")
    .run()
    .await;
}