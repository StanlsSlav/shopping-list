use super::*;

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = products)]
pub struct Product {
    pub name: String,
    pub price: f32,
    pub billing_type: String,
    pub qr_code: String,
}
