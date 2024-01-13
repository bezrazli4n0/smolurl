use crate::{db, state};
use actix_web::{get, http, web, HttpResponse};

#[get("/{key}")]
#[tracing::instrument]
pub async fn redirect(data: web::Data<state::App>, key: web::Path<String>) -> HttpResponse {
    let maybe_link = match db::get_link_by_key(&data.db_pool, &key).await {
        Ok(maybe_link) => maybe_link,
        Err(error) => {
            tracing::event!(tracing::Level::ERROR, %error, "Unable to query link by key in db");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let Some(link) = maybe_link else {
        tracing::event!(tracing::Level::WARN, "Link is not found by key");
        return HttpResponse::NotFound().finish();
    };

    if let Err(error) = db::increment_link_redirect(&data.db_pool, &key).await {
        tracing::event!(tracing::Level::ERROR, %error, "Can't increment link redirect in db");
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::PermanentRedirect()
        .insert_header((http::header::LOCATION, link.url))
        .finish()
}
