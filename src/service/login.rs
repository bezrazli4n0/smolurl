use crate::{db, state};
use actix_web::{post, web, HttpResponse};
use jwt_simple::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[post("/login")]
#[tracing::instrument]
pub async fn login(data: web::Data<state::App>, form: web::Json<Login>) -> HttpResponse {
    let maybe_user = match db::get_user_by_username(&data.db_pool, &form.username).await {
        Ok(maybe_user) => maybe_user,
        Err(error) => {
            tracing::event!(tracing::Level::ERROR, %error, "Unable to get user by username from db");
            return HttpResponse::InternalServerError().finish();
        }
    };

    if let Some(user) = maybe_user {
        let phash = sha256::digest(&form.password);
        if user.phash == phash {
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

            return HttpResponse::Ok()
                .insert_header(("Authorization", format!("Bearer {token}")))
                .finish();
        }
    }

    tracing::event!(tracing::Level::TRACE, "Invalid password");
    HttpResponse::Unauthorized().finish()
}
