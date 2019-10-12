#[macro_use]
extern crate serde_derive;

use actix_web::{web, App, HttpResponse, HttpServer, Result};

// serve the merchant form page
fn index() -> Result<HttpResponse> {
	Ok(HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(include_str!("../site/index.html")))
}

// the merchant
#[derive(Serialize, Deserialize)]
struct Merchant {
	name: String,
	url: String,
}

// add a merchant
fn merchants_add(params: web::Form<Merchant>) -> Result<HttpResponse> {
	Ok(HttpResponse::Ok()
		.content_type("text/plain; charset=utf-8")
		.body(format!("name: {},\nurl: {}", params.name, params.url)))
}

fn main() {
    HttpServer::new(|| {
    	App::new()
    		.route("/", web::get().to(index))
    		.route("/merchants/add", web::post().to(merchants_add))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .unwrap();
}
