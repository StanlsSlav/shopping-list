use super::*;

#[derive(Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct ShoppingList {
    pub guid: String,
}