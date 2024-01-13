use crate::{db, state};
use actix_web::{post, web, HttpResponse};
use jwt_simple::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Register {
    pub username: String,
    pub password: String,
}

#[post("/register")]
#[tracing::instrument]
pub async fn register(data: web::Data<state::App>, form: web::Json<Register>) -> HttpResponse {
    match db::get_user_by_username(&data.db_pool, &form.username).await {
        Ok(maybe_user) => {
            if maybe_user.is_some() {
                tracing::event!(tracing::Level::WARN, "Username already in use");
                return HttpResponse::BadRequest().body("Username already in use");
            }
        }
        Err(error) => {
            tracing::event!(tracing::Level::ERROR, %error, "Unable to check if user exist in db");
            return HttpResponse::InternalServerError().finish();
        }
    }

    let claims = Claims::with_custom_claims(
        state::JwtClaim {
            username: form.username.clone(),
        },
        Duration::from_days(30),
    );
    let token = match data.keypair.sign(claims) {
        Ok(token) => token,
        Err(error) => {
            tracing::event!(tracing::Level::ERROR, %error, "Can't sign JWT claims");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let phash = sha256::digest(&form.password);
    if let Err(error) = db::create_user(&data.db_pool, &form.username, &phash).await {
        tracing::event!(tracing::Level::ERROR, %error, "Unable to create user in db");
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok()
        .insert_header(("Authorization", format!("Bearer {token}")))
        .finish()
}
