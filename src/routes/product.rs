use diesel::prelude::*;
use diesel::{delete, insert_into, update, PgConnection};
use rocket::http::Status;
use rocket::serde::json::Json;

use shopping_list::establish_connection;

use crate::models::{insert, query};
use crate::schema::products::dsl::products;
use crate::schema::products::{billing_type, price, qr_code};

#[get("/")]
pub fn get_all() -> Json<Vec<query::product::Product>> {
    let connection: &mut PgConnection = &mut establish_connection();

    let results: Vec<query::product::Product> = products
        .load::<query::product::Product>(connection)
        .expect("Error loading the products");

    Json::from(results)
}

#[get("/<guid>")]
pub fn get(guid: String) -> Json<query::product::Product> {
    let product: query::product::Product =
        get_all().0.into_iter().find(|x| x.guid == guid).unwrap();

    Json::from(product)
}

#[post("/", data = "<product>")]
pub fn post(product: Json<insert::product::Product>) -> Status {
    let connection: &mut PgConnection = &mut establish_connection();

    insert_into(products)
        .values(product.into_inner())
        .execute(connection)
        .expect("Failed to insert product");

    Status::Created
}

#[put("/", data = "<product>")]
pub fn put(product: Json<query::product::Product>) -> Status {
    let connection: &mut PgConnection = &mut establish_connection();

    update(products.find(&product.guid))
        .set((
            price.eq(&product.price),
            billing_type.eq(&product.billing_type),
            qr_code.eq(&product.qr_code),
        ))
        .get_result::<query::product::Product>(connection)
        .expect("Failed to update product");

    Status::Ok
}

#[delete("/<guid>")]
pub fn remove(guid: String) -> Status {
    let connection: &mut PgConnection = &mut establish_connection();

    let rows_changed: usize = delete(products.find(&guid))
        .execute(connection)
        .expect("Failed to delete product");

    match rows_changed {
        1 => Status::Ok,
        _ => Status::BadRequest,
    }
}
