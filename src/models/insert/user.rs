use super::*;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = shopping_lists)]
pub struct User {
    pub guid: String,
}
