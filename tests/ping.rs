mod setup;

use actix_web::{test, web};

#[actix_web::test]
async fn success_ping() {
    let (app, _) = setup::init().await;

    let req = test::TestRequest::get().uri("/ping").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let resp_body = test::read_body(resp).await;
    assert_eq!(resp_body, web::Bytes::from_static(b"Pong"));
}
