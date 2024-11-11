use super::*;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = shopping_lists)]
pub struct ShoppingList {
    pub product: String,
    pub user: String,
    pub amount: i32,
}
