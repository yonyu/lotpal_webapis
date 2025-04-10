mod auth;

use actix_web::dev::ServiceRequest;
use actix_web::{delete, get, App, guard, HttpServer, Responder, web};
use actix_web::{post, web::Form, web::Json, HttpResponse};
use actix_web::web::scope;
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use serde::{Deserialize, Serialize};

use std::sync::Mutex;

async fn validator(
    req: ServiceRequest,
    _credentials: BasicAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    // TODO: add business logic that restricts access to authorized requests
    Ok(req)
}

#[derive(Debug, Serialize, Deserialize)]
struct Subscriber {
    email: String,
    name: String,
}

#[post("/subscribe")]
async fn subscribe(info: Form<Subscriber>) -> HttpResponse {
    println!("🎉 new subscription: {:?}", info.into_inner());

    // actix_web::rt::spawn(async move {
    //     let mut counts = stats.counters.lock().unwrap();
    //     counts.to_fahrenheit += 1;
    // });

    HttpResponse::NoContent().finish()
}

async fn subscribe_with_json(info: Json<Subscriber>) -> HttpResponse {
    println!("🎉 new subscription: {:?}", info.into_inner());
    HttpResponse::NoContent().finish()
}

#[derive(Serialize)]
struct UsageStatsResponse {
    subscribe: u32,
}
#[derive(Default)]
struct Counters {
    subscribe: u32,
}

#[derive(Default)]
struct UsageStats {
    counters: Mutex<Counters>,
}

impl UsageStats {
    fn new() -> Self {
        UsageStats::default()
    }
}

#[get("/usage-statistics")]
async fn usage_statistics(stats: web::Data<UsageStats>) -> impl Responder {
    let mut counts = stats.counters.lock().unwrap();

    let response = UsageStatsResponse {
        subscribe: counts.subscribe,
    };

    counts.subscribe = 0;

    web::Json(response)
}

#[post("/reset-usage-statistics")]
async fn reset_usage_statistics(stats: web::Data<UsageStats>) -> impl Responder {
    let mut counts = stats.counters.lock().unwrap();

    counts.subscribe = 0;

    HttpResponse::NoContent()
}

#[get("/api-key")]
async fn request_api_key() -> actix_web::Result<impl Responder> {
    // TODO: replace with functionality to generate a unique key
    let api_key = String::from("12345");

    Ok(api_key + "\r\n")
}

#[delete("/api-key")]
async fn delete_api_key(_auth: BasicAuth) -> actix_web::Result<impl Responder> {
    // TODO: actually delete the api_key

    Ok(HttpResponse::NoContent().finish())
}

#[get("/")]
async fn index() -> HttpResponse {
    let webpage = r#"
        <!doctype html>
            <head>
                <style>
                    * { <font-family: sans-serif;> }

                    form { display: table; }
                    form > div { display: table-row; }
                    input, label { display: table-cell; margin-bottom: 8px; }
                    label { padding-right: 1rem; }
                </style>
                <title>Actix Web</title>
            </head>
            <body>
                <p>small webapp. Subscribe for more info.</p>

                <form action="/subscribe" method="post">
                    <div>
                        <label for="n">Name</label>
                        <input id="n" type="text" name="name" placeholder="Name" required=true/>
                    </div>
                    <div>
                        <label for="e">Email</label>
                        <input id="e" type="text" name="email" placeholder="Email" required=true/>
                    </div>
                    <div>
                        <label>&nbsp;</label>
                        <input id=submit type=submit value="Subscribe"/>
                    </div>
                </form>
            </body>
        </html>
    "#;

    HttpResponse::Ok().content_type("text/html").body(webpage)
}

#[get("/healthz")]
async fn liveness() -> &'static str {
    "ok\r\n"
}

#[actix_web::main]
 async fn main() -> std::io::Result<()> {
    auth::load_api_keys().expect("could not load api keys");

    let counts = web::Data::new(UsageStats::new());

    HttpServer::new(move || {
        App::new()
            .app_data(counts.clone())
            .service(
                scope("/api")
                    .wrap(HttpAuthentication::basic(validator))
                    .service(subscribe),
            )
            .service(index)
            .service(usage_statistics)
            .service(reset_usage_statistics)
            .service(web::resource("/subscribe")
                         .guard(guard::Header("Content-Type", "application/json"))
                         .route(web::post().to(subscribe_with_json)),)
            .service(liveness)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}