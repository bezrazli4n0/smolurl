mod setup;

use actix_web::test;
use smolurl::service;

const TEST_USERNAME: &str = "Admin";
const TEST_PASSWORD: &str = "1337";

#[actix_web::test]
async fn success_login() {
    let (app, _test_app_state) = setup::init().await;

    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(service::Register {
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(service::Login {
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let _jwt_token = resp
        .headers()
        .get("Authorization")
        .expect("Unexpected - jwt token not found")
        .to_str()
        .expect("Unexpected - invalid jwt token data");
}

#[actix_web::test]
async fn fail_wrong_username() {
    let (app, _test_app_state) = setup::init().await;

    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(service::Register {
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(service::Login {
            username: "1337".to_string(),
            password: TEST_PASSWORD.to_string(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn fail_wrong_password() {
    let (app, _test_app_state) = setup::init().await;

    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(service::Register {
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(service::Login {
            username: TEST_USERNAME.to_string(),
            password: "13371337".to_string(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
