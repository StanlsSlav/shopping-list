use super::*;

#[derive(Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub guid: String,
    pub name: String,
    pub price: f32,
    pub billing_type: String,
    pub qr_code: String,
}
