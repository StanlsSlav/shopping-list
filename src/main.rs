#[macro_use]
extern crate rocket;

use rocket::{http::Status, Build, Rocket};

use routes::product;

mod models;
mod routes;
mod schema;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![index])
        .mount(
            "/api/product",
            routes![
                product::get_all,
                product::get,
                product::post,
                product::put,
                product::remove
            ],
        )
}

#[get("/")]
fn index() -> Status {
    Status::ImATeapot
}

#[cfg(test)]
mod test {
    use crate::models::insert::product::Product;

    use super::models::*;
    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn root_returns_tea_pot() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::index)).dispatch();

        assert_eq!(response.status(), Status::ImATeapot);
    }

    #[test]
    fn api_product_returns_some_products() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/api/product")).dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let products = response.into_json::<Vec<query::product::Product>>();
        assert!(products.is_some());
    }

    #[test]
    fn api_product_returns_error_on_invalid_guid() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/api/product/123")).dispatch();

        // TODO: Make api not respond with an 500 internal server error
        assert_eq!(response.status(), Status::InternalServerError);
    }

    #[test]
    fn api_product_inserts_a_valid_product() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let new_product: Product = Product {
            name: "Test Product".to_string(),
            price: 32.2f32,
            billing_type: "8fa3ffcb-8309-4530-80c6-11c3bfc9929c".to_string(),
            qr_code: "qwe".to_string(),
        };

        let response = client
            .post(uri!("/api/product"))
            .json(&new_product)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
    }
}
