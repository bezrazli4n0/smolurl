use crate::{db, state};
use actix_web::{get, post, web, HttpResponse};
use base64::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CreateLink {
    pub url: String,
}

#[get("/link/{key}")]
#[tracing::instrument]
pub async fn get_link(
    data: web::Data<state::App>,
    jwt_claim: web::ReqData<state::JwtClaim>,
    key: web::Path<String>,
) -> HttpResponse {
    let maybe_link = match db::get_link_by_key(&data.db_pool, &key).await {
        Ok(maybe_link) => maybe_link,
        Err(error) => {
            tracing::event!(tracing::Level::ERROR, %error, "Can't query db for link by key");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let Some(link) = maybe_link else {
        tracing::event!(tracing::Level::TRACE, "Link is not found by key");
        return HttpResponse::NotFound().finish();
    };

    let maybe_user = match db::get_user_by_username(&data.db_pool, &jwt_claim.username).await {
        Ok(maybe_user) => maybe_user,
        Err(error) => {
            tracing::event!(tracing::Level::ERROR, %error, "Can't obtain user by username from db");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let Some(user) = maybe_user else {
        tracing::event!(tracing::Level::TRACE, "User is not found");
        return HttpResponse::Unauthorized().finish();
    };

    if link.userid != user.id {
        return HttpResponse::Unauthorized().finish();
    }

    HttpResponse::Ok().json(link)
}

#[post("/link")]
#[tracing::instrument]
pub async fn create_link(
    data: web::Data<state::App>,
    jwt_claim: web::ReqData<state::JwtClaim>,
    body: web::Json<CreateLink>,
) -> HttpResponse {
    let maybe_user = match db::get_user_by_username(&data.db_pool, &jwt_claim.username).await {
        Ok(maybe_user) => maybe_user,
        Err(error) => {
            tracing::event!(tracing::Level::ERROR, %error, "Unable to get user by username from db");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let Some(user) = maybe_user else {
        tracing::event!(tracing::Level::TRACE, "User is not found");
        return HttpResponse::Unauthorized().finish();
    };

    // WARN: Only for demo usage
    let key = BASE64_STANDARD.encode(format!("{}:{}", user.id, user.url_counter));
    let link = match db::create_link(&data.db_pool, &body.url, &key, user.id).await {
        Ok(link) => link,
        Err(error) => {
            tracing::event!(tracing::Level::ERROR, %error, "Can't create link in db");
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(link)
}

#[get("/links")]
#[tracing::instrument]
pub async fn get_links(
    data: web::Data<state::App>,
    jwt_claim: web::ReqData<state::JwtClaim>,
) -> HttpResponse {
    let maybe_user = match db::get_user_by_username(&data.db_pool, &jwt_claim.username).await {
        Ok(maybe_user) => maybe_user,
        Err(error) => {
            tracing::event!(tracing::Level::ERROR, %error, "Can't get user by username from db");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let Some(user) = maybe_user else {
        tracing::event!(tracing::Level::TRACE, "User is not found");
        return HttpResponse::Unauthorized().finish();
    };

    let links = match db::get_links_from_user(&data.db_pool, user.id).await {
        Ok(links) => links,
        Err(error) => {
            tracing::event!(tracing::Level::ERROR, %error, "Can't obtain all user links from db");
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(links)
}
