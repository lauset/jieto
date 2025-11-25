use crate::config::ApplicationConfig;
use crate::error::WebError;
use crate::log4r::init_logger;
use actix_cors::Cors;
use actix_web::web::ServiceConfig;
use actix_web::{App, HttpResponse, HttpServer, Responder, ResponseError, web};
use jieto_db::database::DbManager;
use serde::Serialize;
use std::sync::{Arc, OnceLock};

mod config;
mod error;
mod log4r;
mod resp;
#[cfg(feature = "ws")]
mod ws;

pub use resp::ApiResult;

#[cfg(feature = "database")]
pub static GLOBAL_DBMANAGER: OnceLock<Arc<DbManager>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct BusinessError {
    pub code: u16,
    pub msg: &'static str,
}

pub type JietoResult<T> = Result<ApiResult<T>, WebError>;

pub struct Success<T>(pub T);

impl<T> Responder for Success<T>
where
    T: Serialize,
{
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let res = ApiResult {
            code: 200,
            msg: "success".to_string(),
            data: Some(self.0),
        };
        HttpResponse::Ok().json(res)
    }
}

#[derive(Default, Debug)]
pub struct AppState {
    #[cfg(feature = "database")]
    pub db_manager: Arc<DbManager>,
    #[cfg(feature = "ws")]
    pub ws_server: Option<jieto_ws::WsServerHandle>,
}

#[cfg(feature = "database")]
impl AppState {
    fn with_db(&mut self, db_manager: Arc<DbManager>) {
        self.db_manager = db_manager;
    }
}

#[cfg(feature = "ws")]
impl AppState {
    fn with_ws(&mut self, server_tx: jieto_ws::WsServerHandle) {
        self.ws_server = Some(server_tx);
    }
}

pub async fn jieto_web_start<F, Init, Fut>(
    path: &str,
    init: Init,
    configure_fn: F,
) -> anyhow::Result<()>
where
    F: Fn(&mut ServiceConfig) + Clone + Send + Sync + 'static,
    Init: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let config = ApplicationConfig::from_toml(path).await?;

    let mut state = AppState::default();

    init_logger(&config.log, &config.name.unwrap_or(String::from("app")))?;

    #[cfg(feature = "ws")]
    let ws_handle = {
        let (ws_server, server_tx) = jieto_ws::WsServer::new();
        let ws_server_handle = tokio::task::spawn(ws_server.run());
        state.with_ws(server_tx);
        ws_server_handle
    };

    #[cfg(feature = "database")]
    {
        let db_manager = jieto_db::jieto_db_init("db.toml").await?;
        let db_manager = Arc::new(db_manager);
        let db_manager = GLOBAL_DBMANAGER.get_or_init(|| db_manager);
        state.with_db(db_manager.clone());
    }

    init().await;

    let app_state = web::Data::new(state);
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // 允许任意域名（仅开发用！）
            .allow_any_method()
            .allow_any_header()
            .supports_credentials() // 如果需要携带 cookie
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .configure(|cfg| {
                #[cfg(feature = "ws")]
                {
                    use crate::ws::configure_ws;
                    configure_ws(cfg, config.ws.path.as_deref());
                }
                configure_fn(cfg)
            })
    })
    .bind(("0.0.0.0", config.web.port))?
    .run();

    #[cfg(feature = "ws")]
    {
        tokio::try_join!(server, async move { ws_handle.await.unwrap() })?;
    }

    #[cfg(not(feature = "ws"))]
    {
        server.await?;
    }

    Ok(())
}
