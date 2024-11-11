use super::*;

#[derive(Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub guid: String,
}
