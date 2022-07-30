use std::sync::Mutex;

use actix_files::{Files, NamedFile};
use actix_web::{get, web, web::Data, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::{fn_service, ServiceResponse, ServiceRequest};
use log::info;
use twitch_sources_rework::MyTestStruct;

#[get("/hello")]
async fn hello() -> impl Responder {
    info!("Sending a String.");
    "Hallo Welt"
}

#[get("/json-data")]
async fn jsondata(counter: Data<Mutex<i32>>) -> impl Responder {
    let mut v = counter.lock().unwrap();
    *v += 1;
    let data = MyTestStruct::from(*v);
    info!("Data: {:?}", data);
    info!("Sending: {:?}", counter);
    serde_json::to_string(&data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(Mutex::new(0))
            .service(hello)
            .service(jsondata)
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