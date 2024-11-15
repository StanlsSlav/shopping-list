use super::*;

#[derive(Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct ShoppingList {
    pub guid: String,
    pub product: String,
    pub user: String,
    pub amount: i32,
}
