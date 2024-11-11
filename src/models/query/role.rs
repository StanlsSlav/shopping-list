use super::*;

#[derive(Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Role {
    pub guid: String,
    pub name: String,
}
