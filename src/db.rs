use crate::state;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn init(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new().max_connections(5).connect(url).await
}

pub async fn create_user(
    db_pool: &PgPool,
    username: &str,
    phash: &str,
) -> Result<state::User, sqlx::Error> {
    sqlx::query_as::<_, state::User>("INSERT INTO users VALUES (DEFAULT, $1, $2, 0) RETURNING *")
        .bind(username)
        .bind(phash)
        .fetch_one(db_pool)
        .await
}

pub async fn create_link(
    db_pool: &PgPool,
    url: &str,
    key: &str,
    userid: i32,
) -> Result<state::Link, sqlx::Error> {
    let mut tx = db_pool.begin().await?;

    sqlx::query("UPDATE users set url_counter = url_counter + 1 WHERE id = $1")
        .bind(userid)
        .execute(&mut *tx)
        .await?;

    let new_link = sqlx::query_as::<_, state::Link>(
        "INSERT INTO links VALUES (DEFAULT, $1, $2, $3, 0) RETURNING *",
    )
    .bind(url)
    .bind(key)
    .bind(userid)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(new_link)
}

pub async fn increment_link_redirect(db_pool: &PgPool, key: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE links set count = count + 1 WHERE key = $1")
        .bind(key)
        .execute(db_pool)
        .await?;

    Ok(())
}

pub async fn get_user_by_username(
    db_pool: &PgPool,
    username: &str,
) -> Result<Option<state::User>, sqlx::Error> {
    sqlx::query_as::<_, state::User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_links_from_user(
    db_pool: &PgPool,
    userid: i32,
) -> Result<Vec<state::Link>, sqlx::Error> {
    sqlx::query_as::<_, state::Link>("SELECT * FROM links WHERE userid = $1")
        .bind(userid)
        .fetch_all(db_pool)
        .await
}

pub async fn get_link_by_key(
    db_pool: &PgPool,
    key: &str,
) -> Result<Option<state::Link>, sqlx::Error> {
    sqlx::query_as::<_, state::Link>("SELECT * FROM links WHERE key = $1")
        .bind(key)
        .fetch_optional(db_pool)
        .await
}
