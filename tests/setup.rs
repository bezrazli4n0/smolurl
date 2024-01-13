use actix_web::{dev, test, web, App};
use jwt_simple::prelude::*;
use smolurl::{app, state};

const DATABASE_TEST_URL: &str = "postgres://postgres:mysecretpassword@localhost:5432";

pub async fn init() -> (
    impl dev::Service<actix_http::Request, Response = dev::ServiceResponse, Error = actix_web::Error>,
    state::App,
) {
    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_TEST_URL)
        .await
        .expect("Can't connect to database");

    sqlx::query("DROP DATABASE IF EXISTS smolurl_test")
        .execute(&db_pool)
        .await
        .expect("Unexpected - can't drop test db");
    sqlx::query("CREATE DATABASE smolurl_test")
        .execute(&db_pool)
        .await
        .expect("Unexpected - can't create test db");

    db_pool.set_connect_options(sqlx::postgres::PgConnectOptions::new().database("smolurl_test"));

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY NOT NULL,
            username TEXT NOT NULL,
            phash TEXT NOT NULL,
            url_counter BIGINT NOT NULL
        )"#,
    )
    .execute(&db_pool)
    .await
    .expect("Unexpected - can't create users table in test db");

    sqlx::query("DELETE FROM users")
        .execute(&db_pool)
        .await
        .expect("Unexpected - can't delete all from users table in test db");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS links (
            id SERIAL PRIMARY KEY NOT NULL,
            url TEXT NOT NULL,
            key TEXT NOT NULL,
            userid INTEGER NOT NULL,
            count INTEGER NOT NULL,
            CONSTRAINT fk_user FOREIGN KEY (userid) REFERENCES users(id)
        )"#,
    )
    .execute(&db_pool)
    .await
    .expect("Unexpected - can't create links table in test db");

    sqlx::query("DELETE FROM links")
        .execute(&db_pool)
        .await
        .expect("Unexpected - can't delete all from links table in test db");

    let test_app_state = state::App {
        keypair: RS384KeyPair::generate(2048).expect("Unexpected - can't generate private key"),
        db_pool,
    };

    (
        test::init_service(
            App::new()
                .app_data(web::Data::new(test_app_state.clone()))
                .configure(app::config_app_service),
        )
        .await,
        test_app_state,
    )
}
