use ntex::web;

#[utoipa::path(
  get,
  path = "/ping",
  responses(
    (status = 200, description = "ping", body = [Todo]),
  ),
)]
#[web::get("/ping")]
pub async fn ping() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}


pub fn ntex_config(cfg: &mut web::ServiceConfig) {
  cfg.service(ping);
}
