mod setup;

use actix_web::test;
use smolurl::{db, service, state};

const TEST_USERNAME: &str = "Admin";
const TEST_PASSWORD: &str = "1337";
const TEST_URL: &str = "http://example.com";

#[actix_web::test]
async fn success_redirect() {
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

    let jwt_token = resp
        .headers()
        .get("Authorization")
        .expect("Unexpected - jwt token not found")
        .to_str()
        .expect("Unexpected - invalid jwt token data");

    // Create link
    let req = test::TestRequest::post()
        .uri("/api/link")
        .set_json(service::CreateLink {
            url: TEST_URL.to_string(),
        })
        .insert_header(("Authorization", jwt_token))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let link: state::Link = test::read_body_json(resp).await;
    assert_eq!(link.count, 0);

    // Redirect
    let req = test::TestRequest::get()
        .uri(&format!("/r/{}", link.key))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_redirection());
    assert_eq!(
        resp.headers()
            .get("Location")
            .expect("Unexpected - location header is not set")
            .to_str()
            .expect("Unexpected - invalid location header data"),
        TEST_URL
    );

    let link = db::get_link_by_key(&test_app_state.db_pool, &link.key)
        .await
        .expect("Unexpected - can't obtain link by key from test db")
        .expect("Unexpected - link not found in test db");
    assert_eq!(link.count, 1);
}

#[actix_web::test]
async fn fail_link_not_found() {
    let (app, _test_app_state) = setup::init().await;

    // Redirect
    let req = test::TestRequest::get().uri("/r/fakelink").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
