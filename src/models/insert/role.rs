use super::*;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = roles)]
pub struct Role {
    pub name: String,
}
