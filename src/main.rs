use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::engine::local::Mem;
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[get("/")]
async fn hello(db: web::Data<Surreal<Db>>) -> String  {
    let result: Result<Vec<Record>, surrealdb::Error> = db.select("select * from person;").await;

    match result {
        Ok(res) => json!(res).to_string(),
        Err(err) => err.to_string(),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Debug, Serialize, Deserialize)]
struct Person<'a> {
    title: &'a str,
    marketing: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = Surreal::new::<Mem>(()).await.expect("Error initializing surreal");

    
    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await.expect("Error on use surreal db test");

    let created: Vec<Record> = db.create("person")
    .content(Person {
        title: "Tester",
marketing:false,
    }).await.expect("Error on creating person");
    dbg!(created);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}