use ntex::web;
use serde::Serialize;

use crate::{entities, http};

#[derive(Serialize)]
struct ListNotesResponse {
    notes: Vec<entities::Note>,
}

pub async fn list_notes(
    state: web::types::State<http::HandlerState>,
    req: web::HttpRequest,
) -> impl web::Responder {
    let authorization_header = match req.headers().get("Authorization") {
        Some(header) => header,
        None => return web::HttpResponse::Unauthorized().body("Error: no authorzation provided"),
    };
    let token = match authorization_header
        .to_str()
        .unwrap()
        .strip_prefix("Bearer ")
    {
        Some(token) => token,
        None => return web::HttpResponse::Unauthorized().body("Error: authorization format wrong"),
    };
    let identification_payload = match state.auth_provider.verify_secret(token.to_string()) {
        Ok(payload) => payload,
        Err(e) => return web::HttpResponse::Unauthorized().body(e.to_string()),
    };
    match state.db.get_user_notes(identification_payload.id).await {
        Ok(notes) => web::HttpResponse::Ok().json(&ListNotesResponse { notes }),
        Err(e) => web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}
