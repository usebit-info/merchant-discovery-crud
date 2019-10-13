#[macro_use]
extern crate serde_derive;

use actix_web::{web, App, HttpResponse, HttpServer, Result};

// the merchant
#[derive(Serialize, Deserialize)]
struct Merchant {
	name: String,
	url: String,
}

fn get_index() -> Result<HttpResponse> {
	Ok(HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(include_str!("../site/index.html")))
}

fn post_merchant(params: web::Form<Merchant>) -> Result<HttpResponse> {
	Ok(HttpResponse::Ok()
	.content_type("text/plain; charset=utf-8")
	.body(format!("name: {},\nurl: {}", params.name, params.url)))		
}

fn get_all_merchants(merchants: web::Data<Vec<Merchant>>) -> Result<HttpResponse> {
	
	// let mut mrch: String;

	// for merchant in merchants {
	// 	mrch.append(format!("{:?}", merchant))
	// }

	Ok(HttpResponse::Ok()
		.content_type("text/plain; charset=utf-8")
		.body(""))
}

fn main() {

	//let mut merchants: Vec<Merchant> = Vec::with_capacity(8);

    HttpServer::new(|| {
    	App::new()
    		.route("/", web::get().to(get_index))
    		.route("/merch/add", web::post().to(post_merchant))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .unwrap();
}
