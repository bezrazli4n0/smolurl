use actix_web::{get, Responder};

#[get("/ping")]
#[tracing::instrument]
pub async fn ping() -> impl Responder {
    tracing::event!(tracing::Level::TRACE, "Pong!");

    "Pong"
}
