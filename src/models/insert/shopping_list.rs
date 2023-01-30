use super::*;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = shopping_lists)]
pub struct ShoppingList {
    pub guid: String,
}
