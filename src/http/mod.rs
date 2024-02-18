use crate::ai::AIApi;
use crate::auth::AuthProvider;
use crate::config::HttpServerConfig;
use crate::database::Database;
use crate::utils::E;
use ntex::web;
use std::sync::Arc;

mod handlers;

#[derive(Clone)]
pub struct HttpServer {
    config: HttpServerConfig,
    handler_state: HandlerState,
}

#[derive(Clone)]
pub struct HandlerState {
    db: Arc<dyn Database>,
    auth_provider: Arc<dyn AuthProvider>,
    ai_api: Arc<dyn AIApi>,
}

impl HttpServer {
    pub async fn new(
        db: Arc<dyn Database>,
        ai_api: Arc<dyn AIApi>,
        auth_provider: Arc<dyn AuthProvider>,
        config: &HttpServerConfig,
    ) -> HttpServer {
        Self {
            config: config.clone(),
            handler_state: HandlerState {
                db,
                ai_api,
                auth_provider,
            },
        }
    }

    #[ntex::main]
    pub async fn inner_start(self) -> Result<(), E> {
        let state = self.handler_state.clone();
        web::HttpServer::new(move || {
            web::App::new()
                .state(state.clone())
                .route("/notes", web::get().to(handlers::list_notes))
                .route("/note", web::post().to(handlers::add_note))
        })
        .backlog(1024)
        .bind(("127.0.0.1", self.config.port))
        .unwrap()
        .run()
        .await
        .unwrap();
        Ok(())
    }

    pub async fn start(self) -> Result<(), E> {
        tokio::task::spawn_blocking(move || self.inner_start()).await?
    }
}
