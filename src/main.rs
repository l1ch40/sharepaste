extern crate redis;
use redis::Commands;
use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use serde::Deserialize;
use uuid::Uuid;
use futures::future::{ready, Ready};

#[derive(Serialize)]
struct Content {
    content: String
}
impl Responder for Content {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[derive(Deserialize)]
struct RequestBody {
    content: String
}

#[derive(Serialize)]
struct Id {
    id: String
}

impl Responder for Id {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

async fn fetch(id: String) -> redis::RedisResult<String> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    con.get(id)
}

async fn set(content: String) -> redis::RedisResult<String> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let my_uuid = Uuid::new_v4().to_string();
    let _: () = con.set(&my_uuid, content)?;
    Ok(my_uuid)
}

#[get("/info/{id}")]
async fn display(web::Path(id): web::Path<String>) -> impl Responder {
    let content = fetch(id).await;
    let content = match content {
        Ok(res) => res,
        Err(_error) => {
            "Not Found".to_string()
        }
    };
    Content { content }
}


#[post("/info")]
async fn add(info: web::Json<RequestBody>) -> impl Responder {
    let id = set(info.content.to_string()).await;
    let id = match id {
        Ok(res) => res,
        Err(_error) => {
            "-1".to_string()
        }
    };
    Id{ id }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/v1")
                .service(display)
                .service(add)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
