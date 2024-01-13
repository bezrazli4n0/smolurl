mod setup;

use actix_web::test;
use smolurl::{db, service, state};

const TEST_USERNAME: &str = "Admin";
const TEST_PASSWORD: &str = "1337";
const TEST_URL: &str = "http://example.com";

#[actix_web::test]
async fn success_link() {
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

    let test_user = db::get_user_by_username(&test_app_state.db_pool, TEST_USERNAME)
        .await
        .expect("Unexpected - can't obtain user from test db")
        .expect("Unexpected - user not found in test db");

    let links = db::get_links_from_user(&test_app_state.db_pool, test_user.id)
        .await
        .expect("Unexpected - can't obtain user links from test db");
    assert!(links.is_empty());

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

    let new_link: state::Link = test::read_body_json(resp).await;
    assert_eq!(new_link.url, TEST_URL);
    assert_eq!(new_link.userid, test_user.id);
    assert_eq!(new_link.count, 0);

    let links = db::get_links_from_user(&test_app_state.db_pool, test_user.id)
        .await
        .expect("Unexpected - can't obtain user links from test db");
    assert!(links.len() == 1);
    assert_eq!(links[0].url, TEST_URL);

    // Get all links
    let req = test::TestRequest::get()
        .uri("/api/links")
        .insert_header(("Authorization", jwt_token))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let user_links: Vec<state::Link> = test::read_body_json(resp).await;
    assert_eq!(user_links.len(), 1);

    // Get link by key
    let req = test::TestRequest::get()
        .uri(&format!("/api/link/{}", new_link.key))
        .insert_header(("Authorization", jwt_token))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let link_by_key: state::Link = test::read_body_json(resp).await;
    assert_eq!(link_by_key.url, TEST_URL);
    assert_eq!(link_by_key.userid, test_user.id);
    assert_eq!(link_by_key.count, 0);
}

#[actix_web::test]
async fn fail_user_unauthorized() {
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

    let test_user = db::get_user_by_username(&test_app_state.db_pool, TEST_USERNAME)
        .await
        .expect("Unexpected - can't obtain user from test db")
        .expect("Unexpected - user not found in test db");

    let links = db::get_links_from_user(&test_app_state.db_pool, test_user.id)
        .await
        .expect("Unexpected - can't obtain user links from test db");
    assert!(links.is_empty());

    let req = test::TestRequest::post()
        .uri("/api/link")
        .set_json(service::CreateLink {
            url: TEST_URL.to_string(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
