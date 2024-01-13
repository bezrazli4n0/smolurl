mod setup;

use actix_web::test;
use smolurl::{db, service};

const TEST_USERNAME: &str = "Admin";
const TEST_PASSWORD: &str = "1337";

#[actix_web::test]
async fn success_register() {
    let (app, test_app_state) = setup::init().await;

    assert!(
        db::get_user_by_username(&test_app_state.db_pool, TEST_USERNAME)
            .await
            .expect("Unexpected - can't get user by username from test db")
            .is_none()
    );

    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(service::Register {
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let headers = resp.headers();
    assert!(headers.get("Authorization").is_some());

    assert!(
        db::get_user_by_username(&test_app_state.db_pool, TEST_USERNAME)
            .await
            .expect("Unexpected - can't get user by username from test db")
            .is_some()
    );
}

#[actix_web::test]
async fn fail_username_already_in_use() {
    let (app, test_app_state) = setup::init().await;

    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(service::Register {
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    assert!(
        db::get_user_by_username(&test_app_state.db_pool, TEST_USERNAME)
            .await
            .expect("Unexpected - can't get user by username from test db")
            .is_some()
    );

    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(service::Register {
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
