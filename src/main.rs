#[macro_use]
extern crate rocket;

use rocket::http::Status;

use routes::product;

mod schema;
mod models;
mod routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![index])
        .mount("/api/product", routes![
            product::get_all, product::get, product::post, product::put, product::remove
        ])
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
fn index() -> Status {
    Status::ImATeapot
}
