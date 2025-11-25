use crate::AppState;
use actix_web::{HttpRequest, HttpResponse, web};
use jieto_ws::{actix_ws_handle, handler};
use tokio::task::spawn_local;

pub async fn ws_api(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let (res, session, msg_stream) = actix_ws_handle(&req, stream)?;
    if let Some(ws_server) = &data.ws_server {
        spawn_local(handler::chat_ws(ws_server.clone(), session, msg_stream));
    }

    Ok(res)
}

pub fn configure_ws(cfg: &mut web::ServiceConfig, ws_path: Option<&str>) {
    cfg.service(web::resource(ws_path.unwrap_or("/ws")).route(web::get().to(ws_api)));
}
