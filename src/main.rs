use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use dummy::{get_comic_issue_json_response, RequestBody, parse_comic_issue_urls, parse_comic_urls, ComicsRequestBody};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/details")]
async fn details(req_body: String) -> impl Responder {
    let body: RequestBody = serde_json::from_str(&req_body).unwrap();
    let result = get_comic_issue_json_response(&body.url).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[post("/issues")]
async fn issues(req_body: String) -> impl Responder {
    let body: RequestBody = serde_json::from_str(&req_body).unwrap();
    let result = parse_comic_issue_urls(&body.url).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[post("/comics")]
async fn comics(req_body: String) -> impl Responder {
    let body: ComicsRequestBody = serde_json::from_str(&req_body).unwrap();
    let result = parse_comic_urls(&body.name).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(details)
            .service(issues)
            .service(comics)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// curl --location 'http://localhost:8080/issues' \
// --header 'Content-Type: application/json' \
// --data '{"url":"https://comicbookroundup.com/comic-books/reviews/marvel-comics/immortal-x-men-(2022)/8"}' | json_pp

// curl --location 'http://localhost:8080/issues' \
// --header 'Content-Type: application/json' \
// --data '{"url":"https://comicbookroundup.com/comic-books/reviews/marvel-comics/immortal-x-men-(2022)"}' | json_pp

// curl --location 'http://localhost:8080/comics' \
// --header 'Content-Type: application/json' \
// --data '{"name":"marvel"}' | json_pp
