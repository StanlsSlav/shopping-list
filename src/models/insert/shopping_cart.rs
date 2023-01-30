use super::*;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = shopping_carts)]
pub struct ShoppingCart {
    pub guid: String,
}