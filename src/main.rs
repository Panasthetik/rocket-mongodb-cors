mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{hello, create_user, get_user, update_user, delete_user, get_all_users};
use repository::mongodb_repo::MongoRepo;
use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins,
    Cors, CorsOptions
    };

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[ // 4.
    //CHANGE THESE TO MATCH YOUR PORTS
    "http://localhost:3000",
    "http://127.0.0.1:3000",
    "http://localhost:8000/users",
    "http://0.0.0.0:8080",
    ]);
    CorsOptions { // 5.
    allowed_origins,
    allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(), // 1.
    allowed_headers: AllowedHeaders::some(&[
    "Authorization",
    "Accept",
    "Access-Control-Allow-Origin", // 6.
    ]),
    allow_credentials: true,
    ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
    }

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![hello])
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
        .attach(make_cors())
}

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![hello])
// }
