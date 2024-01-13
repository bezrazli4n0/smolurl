#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub phash: String,
    pub url_counter: i64,
}
