use actix_identity::Identity;
use actix_web::HttpResponse;

pub async fn admin_page(id: Identity) -> HttpResponse {
    let user = id.identity().unwrap_or_else(|| "annonymus".to_owned());

    let html = r##"
      <h1>hello admin</h1>
      <h2>COOKIE</h2>
      "##
    .replace("COOKIE", &user.to_string());

    HttpResponse::Ok()
        .content_type("text/html; charse=utf-8")
        .body(html)
}
