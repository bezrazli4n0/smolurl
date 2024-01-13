#[derive(sqlx::FromRow, Debug, serde::Serialize, serde::Deserialize)]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub key: String,
    pub userid: i32,
    pub count: i32,
}
