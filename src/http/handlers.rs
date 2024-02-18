use std::io::ErrorKind;

use ntex::web;
use serde::{Deserialize, Serialize};

use crate::utils::E;
use crate::{auth::IdentificationPayload, entities, http};

#[derive(Serialize)]
pub struct Response<Value: Serialize> {
    ok: bool,
    error: Option<String>,
    value: Option<Value>,
}

#[derive(Serialize)]
struct ListNotesResponse {
    notes: Vec<entities::Note>,
}

fn extract_identification(
    state: web::types::State<http::HandlerState>,
    req: web::HttpRequest,
) -> Result<IdentificationPayload, E> {
    let authorization_header = match req.headers().get("Authorization") {
        Some(header) => header,
        None => {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::InvalidData,
                "Error: no authorzation provided",
            )))
        }
    };
    let token = match authorization_header
        .to_str()
        .unwrap()
        .strip_prefix("Bearer ")
    {
        Some(token) => token,
        None => {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::InvalidData,
                "Error: authorization format wrong",
            )))
        }
    };
    state.auth_provider.verify_secret(token.to_string())
}

pub async fn list_notes(
    state: web::types::State<http::HandlerState>,
    req: web::HttpRequest,
) -> impl web::Responder {
    let identification_payload = match extract_identification(state.clone(), req) {
        Ok(payload) => payload,
        Err(e) => {
            return web::HttpResponse::Unauthorized().json(&Response::<String> {
                ok: false,
                error: Some(e.to_string()),
                value: None,
            })
        }
    };
    match state.db.get_user_notes(identification_payload.id).await {
        Ok(notes) => web::HttpResponse::Ok().json(&Response {
            ok: true,
            error: None,
            value: Some(ListNotesResponse { notes }),
        }),
        Err(e) => web::HttpResponse::InternalServerError().json(&Response::<String> {
            ok: false,
            error: Some(e.to_string()),
            value: None,
        }),
    }
}

#[derive(Deserialize)]
pub struct AddNoteRequest {
    text: String,
}

pub async fn add_note(
    state: web::types::State<http::HandlerState>,
    req: web::HttpRequest,
    req_body: web::types::Json<AddNoteRequest>,
) -> impl web::Responder {
    let identification_payload = match extract_identification(state.clone(), req) {
        Ok(payload) => payload,
        Err(e) => {
            return web::HttpResponse::Unauthorized().json(&Response::<String> {
                ok: false,
                error: Some(e.to_string()),
                value: None,
            })
        }
    };
    let _ai_api = state.ai_api.clone();
    let _db = state.db.clone();
    tokio::task::spawn(async move {
        let embedding = _ai_api.get_embedding(&req_body.text).await?;
        _db.insert_note(entities::Note {
            text: req_body.text.clone(),
            embedding,
            owner_telegram_id: identification_payload.id,
        })
        .await?;
        Ok::<(), E>(())
    });
    web::HttpResponse::Ok().json(&Response::<&str> {
        ok: true,
        error: None,
        value: Some("Note is indexing"),
    })
}
